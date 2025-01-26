import React, { useEffect } from 'react';
import { Fragment, useState } from "react";
import { Dialog, Transition } from "@headlessui/react";
import TokenData from '../TokenData';
import { useAuth } from '../../use-auth-client';
import { Principal } from "@dfinity/principal"; // Import Principal
import SuccessModal from './SuccessModel';

const Buy = () => {
    const { whoamiActor, icpActor, mercx_Actor, isAuthenticated } = useAuth();
    const [Icpbalance, setIcpBalance] = useState(0n); // Keep balance as BigInt
    const { principal } = useAuth();
    const [balance, setBalance] = useState(0n);
    const [tokenName, setTokenName] = useState("");
    const [inputIcp, setInputIcp] = useState('');
    const [amountMercx, setAmountMercx] = useState('0.0');
    //const [logoUrl, setLogoUrl] = useState("");//not used 
    const [rate, setRate] = useState("0");
    const [canisterBalance, setCanisterBalance] = useState(""); //swap canister balance
    // Add state for fetchingRate at the top level of your component 
    // Loading upper rate 
    const [isloadingRate, setLoadingRate] = useState(false);
    const [fetchingRateDown, setFetchingRate] = useState(false);
    const [isModalVisible, setIsModalVisible] = useState(false); // State for modal visibility
    //Negative input error (icp)
    const [inputError, setInputError] = useState("");
    //Handling swapping time
    const [notSwapped, setNotSwapped] = useState(true);

    async function handleAmountChange(e) {
        const inputValue = Number(e.target.value);
        if (inputValue < 0.0001 && inputValue !== 0) {
            setInputError("Amount must be greater than 0.0001");
            setInputIcp('');  // Reset the input field
            return;
        }

        else {
            setInputError("");
            setInputIcp(e.target.value);
            setLoadingRate(true);  // Start fetching, show loading indicator
        }
        try {
            const rateResponse = await mercx_Actor.get_icp_rate();
            console.log('Rate fetched:', rateResponse);

            // Ensure rate.Ok is a number and not undefined or NaN
            if (!rateResponse.Ok || isNaN(rateResponse.Ok)) {
                console.error('Invalid rate:', rateResponse.Ok);
                setAmountMercx(0);  // Set Mercx amount to zero if rate is invalid
                setLoadingRate(false);  // Stop fetching, hide loading indicator
                return;
            }

            const inputIcp = Number(e.target.value);
            const rate = Number(rateResponse.Ok);
            // Fetching rate up 
            setRate(rate);
            // Fetching rate down 
            setFetchingRate(true);
            // Calculating and setting amount of Mercx, rounding to 4 decimal places
            const amountMercx = (inputIcp * rate);
            setAmountMercx(amountMercx);
            console.log('Calculated Mercx Amount:', amountMercx);
        } catch (error) {
            console.error('Failed to fetch rate:', error);
            setAmountMercx(0);  // Handle any errors by setting Mercx amount to zero
        }
        finally {
            setLoadingRate(false);  // Ensure fetching indicator is hidden after fetch attempt
        }
    }

    async function fetchData(principalId) {
        try {
            // Use principalId directly if it's a Principal object
            const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

            // Fetch token name
            const name = await whoamiActor.icrc1_name();
            setTokenName(name);

            // Fetch logo URL
            // const logo = await mercx_Actor.get_logo_url();
            // setLogoUrl(logo);

            // Fetch user balance
            const balanceResult = await whoamiActor.icrc1_balance_of({
                owner, // Use the Principal object directly
                subaccount: [],
            });
            const numericBalanceMercx = Number(balanceResult);
            const after_ap = numericBalanceMercx / 1e8;
            setBalance(after_ap);

            // Fetch icp balance
            const balanceicp = await icpActor.icrc1_balance_of({
                owner, // Use the Principal object directly
                subaccount: [],
            });
            const numericBalanceIcp = Number(balanceicp);
            const after_app = numericBalanceIcp / 1e8;
            setIcpBalance(after_app);

        } catch (error) {
            console.error("Error fetching data:", error);
        }
    }
    const handleIcpApprove = async (e) => {
        setNotSwapped(false);
        const icp_swap_canister_id = "zoa6c-riaaa-aaaan-qzmta-cai"; // Placeholder for actual canister ID
         let amount = (inputIcp * 1e8);
        let ApprovedIcp = Number(amount+20000); 
       
        let mercxAmountFormat = Math.floor(amountMercx * 1e8);
        try {

            const allowanceCheck = {
                account: { owner: principal, subaccount: [] },
                spender: { owner: Principal.fromText(icp_swap_canister_id), subaccount: [] }
            };

            console.log("Checking allowance with structure:", JSON.stringify(allowanceCheck));

            const currentAllowanceResult = await icpActor.icrc2_allowance(allowanceCheck);
            const currentAllowance = currentAllowanceResult.allowance;

            console.log("Current allowance:", currentAllowance);


            const balanceResult = await whoamiActor.icrc1_balance_of({
                owner: Principal.fromText(icp_swap_canister_id), // Use the Principal object directly
                subaccount: [],
            });

            const numericBalanceMercx = Number(balanceResult);
            const after_ap = numericBalanceMercx / 1e8;
            setCanisterBalance(after_ap);


            //no enough mercx token in mercx canister //3ashan maykhosh approve 
            if (after_ap <= 0 || after_ap < amountMercx) {
                alert("Cannot proceed with swap");
                console.log(after_ap);
                console.log(amountMercx);
                return;
            }

            // user does'nt have icps and handled also in backend, but made it here for 
            if (Icpbalance <= 0 || Icpbalance < inputIcp) {
                alert("Insufficient ICP balance");
                return;
            }

            if (BigInt(currentAllowance) < BigInt(ApprovedIcp)) {

                const resultIcpApprove = await icpActor.icrc2_approve({
                    spender: {
                        owner: Principal.fromText(icp_swap_canister_id),
                        subaccount: [],
                    },
                    amount: BigInt(ApprovedIcp),
                    fee: [BigInt(10000)], // Optional fee, set as needed
                    memo: [],  // Optional memo field
                    from_subaccount: [],  // From subaccount, if any
                    created_at_time: [],  // Specify if needed
                    expected_allowance: [],  // Optional, specify expected allowance if needed
                    expires_at: [],  // Specify if approval should expire
                });

                // If the approval was successful, call the backend function
                if (resultIcpApprove && "Ok" in resultIcpApprove) {
                  //  alert('Approval successful!');
                }

                else {
                    console.error("Approval failed:", resultIcpApprove.Err);
                    alert("Approval failed: " + resultIcpApprove.Err);
                }
            }
            // Call the backend function
            const backendResponse = await mercx_Actor.swap(amount, mercxAmountFormat);
            setInputIcp("");
            setAmountMercx('0.0');
            console.log('Backend response:', backendResponse);

            // Check the backend response for success confirmation
            if (backendResponse && backendResponse.Ok === 'Swapped Successfully!') {
                setIsModalVisible(true); // Show modal on successful swap
                setNotSwapped(true);
            } else {
                // Handle cases where swap was not successful
                console.error('Swap failed:', backendResponse);
            }

            fetchData(principal);

        } catch (error) {
            console.error("Approval process failed:", error);
            alert('Approval failed: ' + error.message);
        }
    };

    useEffect(() => {
        if (principal) {
            fetchData(principal);  // Fetch data when principal and actor are available
        }
    }, [principal, Icpbalance]);

    return (<>
        <div >
            <main>

                <div >
                    <div className="shadow-xl rounded-3xl h-[480px] border-t-[1px] border-slate-800 bg-slate-800">
                        <div className="border-b-[1px] border-gray-900 shadow-md p-3">
                            {/* <p className="text-lg font-bold text-center text-gray-200">
                                Swap
                            </p> */}
                            <p className="text-gray-300 text-center text-sm">
                                Buy BELLA with ICP
                            </p>
                        </div>
                        <div className="p-4">
                            <div className="p-4 mt-4 rounded-md shadow-md">
                                <TokenData TokenBalance={!isAuthenticated ? `0` : Icpbalance.toString()} TokenName="ICP" TokenLogo={"./favicon.ico"} />
                                <input
                                    type="number"
                                     min="0.0001"
                                    step="0.0001"
                                    inputMode="decimal"
                                    name="amount"
                                    id="amount"
                                    value={inputIcp}
                                    onFocus={(e) => e.target.select()}
                                    onChange={(e) => handleAmountChange(e)}
                                    className="block w-full text-right outline-0 text-gray-200 bg-inherit"
                                    //   disabled={token?.address ? false : true}
                                    placeholder="0.0"
                                />
                                {inputError && <p className="text-red-500 text-sm mt-2">{inputError}</p>}
                            </div>
                            <div className="p-4 mt-4 rounded-md shadow-md">
                                <TokenData TokenBalance={balance} TokenName={tokenName} TokenLogo={"./Bella.jpeg"} />
                                <label
                                    type="number"
                                    min='0'
                                    inputMode="decimal"
                                    name="amount2"
                                    id="amount"
                                    //  value={amountMercx}
                                    onFocus={(e) => e.target.select()}
                                    //   onChange={(e) => onChangeInput(e.target.value, isTokenA)}
                                    className="block w-full pr-4 text-right outline-0 text-gray-200 bg-inherit"
                                    //   disabled={token?.address ? false : true}
                                    placeholder="0.0"
                                >

                                    {isloadingRate ? (
                                        <div className="flex items-center justify-center space-x-2">
                                            <svg className="animate-spin h-5 w-5 text-gray-600" viewBox="0 0 24 24">
                                                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.86 1.861 7.298 4.708 9.291l1.292-1.292z"></path>
                                            </svg>
                                            <span className="text-sm text-gray-600">Fetching price...</span>
                                        </div>
                                    ) : amountMercx} {/* Assuming amountMercx is a number, format it to four decimal places */}
                                </label>
                            </div>
                            {fetchingRateDown && inputIcp !== "" && (
                                <div className="text-sm font-medium text-center text-gray-200 pt-5">
                                    1 ICP = {rate} Mercx (${rate})
                                </div>
                            )}
                        </div>

                        <div className="flex justify-center">
                            <button
                                type="button"
                                className="place-content-center py-2 px-6 text-sm font-bold text-white bg-indigo-600 rounded-md bg-opacity-85 hover:bg-opacity-90 disabled:bg-indigo-400"
                                disabled={!isAuthenticated || inputIcp === '0' || inputIcp === '' || isloadingRate || !notSwapped}
                                onClick={() => handleIcpApprove()}
                            >
                                {!isAuthenticated ? "Connect your wallet" :
                                    inputIcp === '0' || inputIcp === '' ? "Enter an amount" :
                                        !notSwapped ? "Processing..." : // This line checks if notSwapped is false
                                            "BUY"}

                            </button>

                        </div>
                        {/*transfer fees*/}

                        <div className=" p-2 m-2 border-gray-700 bg-gray-800 flex justify-center ">

                            <dl className="flex items-center gap-4">
                                <dt className="text-sm font-normal text-gray-400">Network Fees</dt>
                                <dd className="text-sm font-medium text-white">0.0002 ICP</dd>
                            </dl>
                        </div>



                        {/* Modal for success message */}
                        {isModalVisible && (
                            <div>
                                <SuccessModal isVisible={isModalVisible} onClose={() => setIsModalVisible(false)} />
                            </div>
                        )}
                    </div>
                </div>
            </main>
        </div>
    </>);
}

export default Buy;
