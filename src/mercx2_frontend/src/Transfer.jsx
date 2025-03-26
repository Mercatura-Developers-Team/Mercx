import React, { useState, useEffect,useRef } from 'react';
import { useAuth } from "./use-auth-client";
import { Principal } from "@dfinity/principal";
import * as Yup from "yup";
import { useNavigate } from 'react-router-dom'; 
import ReceiveModal from './ReceiveModal'; // Add this import

const Transfer = () => {
    const { whoamiActor, icpActor, mercx_Actor, isAuthenticated, tommy_Actor, fxmxActor, kycActor } = useAuth();
    const { principal } = useAuth();
    const navigate = useNavigate();  // Use navigate for redirection
    const transferSectionRef = useRef(null); // Add this ref
    const [showReceiveModal, setShowReceiveModal] = useState(false); // Add this state

    const [selectedToken, setSelectedToken] = useState("BELLA"); // Default selected token
    const [tokens, setTokens] = useState([
        { name: "BELLA", actor: whoamiActor, transferMethod: "icrc1_transfer", logo: "/Bella.jpeg", balances: "icrc1_balance_of", actions: ["Swap", "Send", "Receive"] }, // Assuming mercx_Actor has a transfer method
        { name: "ICP", actor: icpActor, transferMethod: "icrc1_transfer", logo: "/favicon.ico", balances: "icrc1_balance_of", actions: ["Swap", "Send", "Receive"], }, // Assuming icpActor has icrc1_transfer method
        { name: "TOMMY", actor: tommy_Actor, transferMethod: "icrc1_transfer", logo: "/Tommy.JPG", balances: "icrc1_balance_of", actions: ["Swap", "Send", "Receive"], },
        { name: "FXMX", actor: fxmxActor, transferMethod: "icrc1_transfer", logo: "/j.png", balances: "icrc1_balance_of", actions: ["Send", "Receive", "Transactions"], } // Assuming tommy_Actor has a transfer method

    ]);
    const [errorMessage, setErrorMessage] = useState("");
    const [tokenBalances, setTokenBalances] = useState({});

     // Modify your token actions handler
     const handleTokenAction = (action, tokenName) => {
        if (action === "Send") {
            setSelectedToken(tokenName);
            transferSectionRef.current?.scrollIntoView({ behavior: 'smooth' });
        } 
        else if (action === "Swap") {
            navigate('/trade');
        }
        else if (action === "Transactions") {
            navigate('/transactions');
        }
        else if (action === "Receive") {
            setShowReceiveModal(true); // Show receive modal
        }
    };


    async function fetchTokenBalances(principalId) {
        try {
            const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;
            const balances = {};

            for (const token of tokens) {
                try {
                    // Skip if actor is null
                    if (!token.actor) {
                        console.warn(`Actor not initialized for token: ${token.name}`);
                        balances[token.name] = 0;
                        continue;
                    }

                    // Skip if balance method doesn't exist
                    if (typeof token.actor[token.balances] !== 'function') {
                        console.warn(`Balance method not found for token: ${token.name}`);
                        balances[token.name] = 0;
                        continue;
                    }

                    const balanceResult = await token.actor[token.balances]({
                        owner,
                        subaccount: []
                    });
                    const numericBalance = Number(balanceResult) / 1e8;
                    balances[token.name] = numericBalance;
                } catch (error) {
                    console.error(`Error fetching balance for ${token.name}:`, error);
                    balances[token.name] = 0;
                }
            }

            setTokenBalances(balances);
        } catch (error) {
            console.error("Error fetching token balances:", error);
        }
    }


    useEffect(() => {
        if (principal) {
            fetchTokenBalances(principal);
            //fetchData(principal);
        }
    }, [principal, tokens]);

    const handleTransfer = async (event) => {
        event.preventDefault();

        try {
            //const toAccount = event.target.elements.to.value.trim();
            const toUsername = event.target.elements.to.value.trim();
            const amount = BigInt(event.target.elements.amount.value * 1e8); // Adjust for token decimals

            if (!toUsername || amount <= 0n) {
                setErrorMessage("Please provide valid inputs");
                return;
            }

            let recipientPrincipal;
            try {
                const principalResponse = await kycActor.get_principal_by_username(toUsername);
                console.log("Principal Response:", principalResponse);

                // Check if the response contains an "Err" field
                if (principalResponse.Err) {
                    setErrorMessage(principalResponse.Err); // Display the error message
                    return;
                }



                // Extract the Principal object from the "Ok" field
                recipientPrincipal = principalResponse.Ok;

                // Ensure the extracted value is a valid Principal object
                if (!recipientPrincipal || !recipientPrincipal._isPrincipal) {
                    console.log("Invalid Principal ID format received from the backend.");
                    return;
                }

                // Convert the Principal object to a string (optional)
                const principalText = recipientPrincipal.toString();
                console.log("Principal as string:", principalText);
                setErrorMessage(" ");
            } catch (err) {
                console.error("Error fetching Principal ID:", err);
                setErrorMessage("Invalid username Please try again.");
                return;
            }

            // Check if the recipient is whitelisted
            const isRecipientWhitelisted = await mercx_Actor.is_whitelisted(recipientPrincipal);
            if (!isRecipientWhitelisted) {
                setErrorMessage("The recipient is not whitelisted to receive tokens.");
                return; // Exit the function early
            }

            // Check if the caller is whitelisted
            const isCallerWhitelisted = await mercx_Actor.is_whitelisted(principal);
            if (!isCallerWhitelisted) {
                setErrorMessage("You are not whitelisted to perform this operation.");
                return; // Exit the function early
            }
            const selectedTokenData = tokens.find(token => token.name === selectedToken);
            const selectedTokenActor = selectedTokenData.actor;
            const transferMethod = selectedTokenData.transferMethod;

            // Ensure the transfer method exists
            if (typeof selectedTokenActor[transferMethod] !== 'function') {
                console.error(`Method ${transferMethod} is not a function on the selected token actor`);
                //setErrorMessage(`Transfer method not found for ${selectedToken}. Please check the token configuration.`);
                return;
            }

            // Execute the transfer
            const transferResult = await selectedTokenActor[transferMethod]({
                to: {
                    owner: recipientPrincipal,
                    subaccount: [], // Include subaccount if necessary
                },
                from_subaccount: [], // Include from_subaccount if required
                amount, // Ensure this is included
                fee: [], // Include fee if required
                memo: [], // Include memo if required
                created_at_time: [], // Include created_at_time if required
            });

            if ("Ok" in transferResult) {
                alert("Transfer successful: Block Index " + transferResult.Ok);
            } else {
                console.error("Transfer failed: ", transferResult.Err);
                alert("Transfer failed: " + transferResult.Err);
            }
        } catch (error) {
            //console.error("Transfer failed: ", error);
            alert("Transfer failed: " + error.message);
        }
    };

    return (
        <div className='md:py-10 bg-gray-900 py-8'>


            <div className=" bg-gray-900 p-6">
                <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                    {tokens.map((token, index) => (
                        <div key={index} className="bg-slate-800 rounded-lg shadow-lg p-6">
                            <div className="flex items-center gap-3 mb-4">
          {token.logo && (
            <div className="h-10 w-10 rounded-full bg-slate-700 flex items-center justify-center overflow-hidden">
              <img 
                src={token.logo}
                alt={token.name}
                className="h-8 w-8 object-contain"
              />
            </div>
          )}
          <h2 className="text-xl font-bold text-white">{token.name}</h2>
        </div>
                            <p className="text-gray-400">
                                Balance: {tokenBalances[token.name] !== undefined ?
                                    `${tokenBalances[token.name].toFixed(4)}` :
                                    "Loading..."}
                            </p>
                            <div className="mt-4 flex flex-wrap gap-2">
                                {token.actions.map((action, idx) => (
                                    <button
                                        key={idx}
                                        className=" text-white px-4 py-1.5 text-sm rounded-lg transition duration-200 bg-gradient-to-r from-indigo-500 to-indigo-700 hover:from-indigo-700 "
                                        onClick={() => handleTokenAction(action, token.name)}
                                    >
                                        {action}
                                    </button>
                                ))}
                                  {/* Add this at the bottom of your return */}
            {showReceiveModal && (
                <ReceiveModal 
                    principal={principal} 
                    onClose={() => setShowReceiveModal(false)} 
                />
            )}
                            </div>
                        </div>
                    ))}
                </div>
            </div>


            <section ref={transferSectionRef}  className="p-4 m-4 rounded-lg shadow bg-slate-800 text-gray-900 border border-gray-700">
                <h2 className="font-bold text-lg text-white">Transfer Token</h2>
                <form className="flex flex-col gap-4" onSubmit={handleTransfer}>
                    <div className="flex items-center gap-2 mb-2">
                        {tokens.find(token => token.name === selectedToken)?.logo && (
                            <img
                                src={tokens.find(token => token.name === selectedToken)?.logo}
                                alt={selectedToken}
                                className="h-6 w-6"
                            />
                        )}
                        <span className="text-white font-semibold">{selectedToken}</span>
                    </div>
                    <label className="block text-sm font-medium text-white w-full">
                        Select Token:
                        <select
                            value={selectedToken}
                            onChange={(e) => setSelectedToken(e.target.value)}
                            className="w-full mt-3 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900"
                        >
                            {tokens.map((token) => (
                                <option key={token.name} value={token.name}>
                                    {token.name}
                                </option>
                            ))}
                        </select>
                    </label>

                    <label className="block text-sm font-medium text-white w-full">
                        To Account:
                        <input
                            type="text"
                            name="to"
                            placeholder="Enter recipient's username"
                            required
                            className="w-full mt-3 rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-primary-500 focus:ring-primary-500"
                        />
                    </label>

                    <label className="block text-sm font-medium text-white w-full">
                        Amount:
                        <input
                            type="number"
                            name="amount"
                            step="any"
                            placeholder="Amount to transfer"
                            required
                            className="w-full mt-3 rounded-lg border border-gray-300 p-2.5 bg-gray-50 text-sm text-gray-900 focus:border-primary-500 focus:ring-primary-500"
                        />
                    </label>
                    <span className="text-gray-500 text-sm">Network fees 0.0001 {selectedToken}</span>


                    <button type="submit" className="bg-gradient-to-r from-indigo-500 to-indigo-700 hover:from-indigo-700 hover:to-indigo-900 text-white py-2 px-4 font-bold rounded-lg text-sm flex items-center">
                        Send
                    </button>

                    {errorMessage && (
                        <div className="p-2 text-sm rounded-lg bg-gray-800 text-red-400 flex items-center text-center">
                            <span className="font-medium">{errorMessage}</span>
                        </div>
                    )}

                </form>


            </section>
        </div>
    );
}

export default Transfer;
