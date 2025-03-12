import React, { useState, useEffect } from 'react';
import { useAuth } from "./use-auth-client";
import { Principal } from "@dfinity/principal";

const Transfer = () => {
    const { whoamiActor, icpActor, mercx_Actor, isAuthenticated, tommy_Actor, fxmxActor, kycActor } = useAuth();
    const { principal } = useAuth();
    const [tokenName, setTokenName] = useState("");
    const [icptokenName, setIcpTokenName] = useState("");
    const [TommytokenName, setTommyName] = useState("");
    const [FXMXtokenName, setFXMXName] = useState("");
    const [balance, setBalance] = useState(0n);
    const [Icpbalance, setIcpBalance] = useState(0n);
    const [Tommybalance, setTommyBalance] = useState(0n);
    const [FXMXbalance, setFXMXBalance] = useState(0n);
    const [selectedToken, setSelectedToken] = useState("BELLA"); // Default selected token
    const [tokens, setTokens] = useState([
        { name: "BELLA", actor: whoamiActor, transferMethod: "icrc1_transfer" }, // Assuming mercx_Actor has a transfer method
        { name: "ICP", actor: icpActor, transferMethod: "icrc1_transfer" }, // Assuming icpActor has icrc1_transfer method
        { name: "TOMMY", actor: tommy_Actor, transferMethod: "icrc1_transfer" },
        { name: "FXMX", actor: fxmxActor, transferMethod: "icrc1_transfer" } // Assuming tommy_Actor has a transfer method

    ]);
    const [errorMessage, setErrorMessage] = useState("");
    async function fetchData(principalId) {
        try {
            const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;
            const name = await whoamiActor.icrc1_name();
            setTokenName(name);
            const icptokenname = await icpActor.icrc1_symbol();
            setIcpTokenName(icptokenname);
            const tommyTokenname = await tommy_Actor.icrc1_name();
            setTommyName(tommyTokenname);
            const fxmxTokenname = await fxmxActor.icrc1_symbol();
            setFXMXName(fxmxTokenname);

            const balanceResult = await whoamiActor.icrc1_balance_of({ owner, subaccount: [] });
            const numericBalanceMercx = Number(balanceResult);
            const after_ap = numericBalanceMercx / 1e8;
            setBalance(after_ap);

            const balanceicp = await icpActor.icrc1_balance_of({ owner, subaccount: [] });
            const numericBalanceIcp = Number(balanceicp);
            const after_app = numericBalanceIcp / 1e8;
            setIcpBalance(after_app);

            const balanceTommy = await tommy_Actor.icrc1_balance_of({ owner, subaccount: [] });
            const numericBalanceTommy = Number(balanceTommy);
            const after_ap_tommy = numericBalanceTommy / 1e8;
            setTommyBalance(after_ap_tommy);

            const balanceFXMX = await fxmxActor.icrc1_balance_of({ owner, subaccount: [] });
            const numericBalanceFXMX = Number(balanceFXMX);
            const after_ap_fxmx = numericBalanceFXMX / 1e8;
            setFXMXBalance(after_ap_fxmx);
        } catch (error) {
            console.error("Error fetching data:", error);
        }
    }

    useEffect(() => {
        if (principal) {
            fetchData(principal);
        }
    }, [principal, balance, Icpbalance, Tommybalance]);

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
            <section className="p-4 m-4 rounded-lg shadow bg-slate-800 text-white border border-gray-700">
                <h2 className="text-lg font-bold">Your Balance</h2>
                <p className="text-xl">{!isAuthenticated ? `0 ${tokenName}` : `${balance.toString()} ${tokenName}`}</p>
                <p className="text-xl">{!isAuthenticated ? `0 ${TommytokenName}` : `${Tommybalance.toString()} ${TommytokenName}`}</p>
                <p className="text-xl">{!isAuthenticated ? `0 ${icptokenName}` : `${Icpbalance.toString()} ${icptokenName}`}</p>
                <p className="text-xl">{!isAuthenticated ? `0 ${FXMXtokenName}` : `${FXMXbalance.toString()} ${FXMXtokenName}`}</p>

            </section>

            <section className="p-4 m-4 rounded-lg shadow bg-slate-800 text-gray-900 border border-gray-700">
                <h2 className="font-bold text-lg text-white">Transfer Token</h2>
                <form className="flex flex-col gap-4" onSubmit={handleTransfer}>
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
                        To Account (Principal ID):
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