import React, { useEffect } from 'react';
import { useState } from "react";
import TokenData from '../TokenData';
import { useAuth } from '../../use-auth-client';
import { Principal } from "@dfinity/principal"; // Import Principal
import SuccessModal from './SuccessSell';

const Sell = () => {
    const { whoamiActor, mercx_Actor, isAuthenticated, tommy_Actor } = useAuth();
    // Keep balance as BigInt
    const [Tommybalance, setTommyBalance] = useState(0n);
    const { principal } = useAuth();
    const [balance, setBalance] = useState(0n);
    const [tokenName, setTokenName] = useState("");
    const [amountTommy, setAmountTommy] = useState('0.0');
    const [inputBella, setInputBella] = useState('');
    //const [logoUrl, setLogoUrl] = useState("");//not used 
    //const [rate, setRate] = useState("0");
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

    const canisterId = process.env.CANISTER_ID_TOMMY_ICRC1_LEDGER;

    console.log("Canister ID:", canisterId);
    async function handleAmountChange(e) {
        const inputValue = Number(e.target.value);
        if (inputValue < 0.0001 && inputValue !== 0) {
            setInputError("Amount must be greater than 0.001");
            setInputBella('');  // Reset the input field
            return;
        }

        else {
            setInputError("");
            setInputBella(e.target.value);
            setLoadingRate(true);  // Start fetching, show loading indicator
        }
        try {
            // const rateResponse = await mercx_Actor.get_icp_rate();

            // Ensure rate.Ok is a number and not undefined or NaN
            // if (!rateResponse.Ok || isNaN(rateResponse.Ok)) {
            //     console.error('Invalid rate:', rateResponse.Ok);
            //     setAmountTommy(0);  // Set Mercx amount to zero if rate is invalid
            //     setLoadingRate(false);  // Stop fetching, hide loading indicator
            //     return;
            // }

            const inputBella = Number(e.target.value);
            // const rate = Number(rateResponse.Ok);
            // Fetching rate up 
            //    setRate(rate);
            // Fetching rate down 
            setFetchingRate(true);
            // Calculating and setting amount of Mercx, rounding to 4 decimal places
            // const amountMercx = (inputIcp * rate);

            const amountTommy = (inputBella * 10);
            setAmountTommy(amountTommy);
            console.log('Calculated Tommy Amount:', amountTommy);
        } catch (error) {
            console.error('Failed to fetch rate:', error);
            setAmountTommy(0);  // Handle any errors by setting Mercx amount to zero
        }
        finally {
            setLoadingRate(false);  // Ensure fetching indicator is hidden after fetch attempt
        }
    }

    async function fetchData(principalId) {
        try {
            // Use principalId directly if it's a Principal object
            const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

            //   // Fetch token name
            const name = await whoamiActor.icrc1_name();
            setTokenName(name);

            // // Fetch logo URL
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

            const balancetommy = await tommy_Actor.icrc1_balance_of({
                owner, // Use the Principal object directly
                subaccount: [],
            });
            const numericBalanceTommy = Number(balancetommy);
            const formatted_Balance = numericBalanceTommy / 1e8;
            setTommyBalance(formatted_Balance);

        } catch (error) {
            console.error("Error fetching data:", error);
        }
    }
    const handleIcpApprove = async (e) => {
        setNotSwapped(false);
        const backend_canister = "aovwi-4maaa-aaaaa-qaagq-cai"; // Placeholder for actual canister ID
        let amountBella = (inputBella) * 1e8; // Approximate
        let approvedBella = Number((amountBella) + 20000); //0.0001 lel approve we 0.0001 lel transfer lw mafesh approve 0.0001 el hatetkhesem bas 
        
        //const totalAmountBigInt = BigInt(Math.floor((inputAmount + 0.0002) * 1e8));

        let amountBellaFormatted = Number(amountBella);
        // Convert the user input into a Number, then multiply by 1e8 to convert ICP to e8s
        let tommyAmount = Math.floor(amountTommy * 1e8);
        try {

            const allowanceCheck = {
                account: { owner: principal, subaccount: [] },
                spender: { owner: Principal.fromText(backend_canister), subaccount: [] }
            };

            console.log("Checking allowance with structure:", JSON.stringify(allowanceCheck));

            const currentAllowanceResult = await whoamiActor.icrc2_allowance(allowanceCheck);
            const currentAllowance = currentAllowanceResult.allowance;

            console.log("Current allowance:", currentAllowance);


            const balanceResult = await tommy_Actor.icrc1_balance_of({
                owner: Principal.fromText(backend_canister), // Use the Principal object directly
                subaccount: [],
            });

            const numericBalanceTommy = Number(balanceResult);
            const after_ap = numericBalanceTommy / 1e8;
            // setCanisterBalance(after_ap);


            //no enough tommy token in mercx canister //3ashan may3melsh approve 
            if (after_ap <= 0 || after_ap < amountTommy) {
                alert("Cannot proceed with swap");
                console.log(after_ap);
                console.log(amountTommy);
                return;
            }

            // user does'nt have icps and handled also in backend, but made it here for 
            if (balance <= 0 || balance < inputBella) {
                alert("Insufficient Bella balance");
                return;
            }

            if (BigInt(currentAllowance) < BigInt(approvedBella)) {

                const resultBellaApprove = await whoamiActor.icrc2_approve({
                    spender: {
                        owner: Principal.fromText(backend_canister),
                        subaccount: [],
                    },
                    amount: BigInt(approvedBella),
                    fee: [BigInt(10000)], // Optional fee, set as needed
                    memo: [],  // Optional memo field
                    from_subaccount: [],  // From subaccount, if any
                    created_at_time: [],  // Specify if needed
                    expected_allowance: [],  // Optional, specify expected allowance if needed
                    expires_at: [],  // Specify if approval should expire
                });

                // If the approval was successful, call the backend function
                if (resultBellaApprove && "Ok" in resultBellaApprove) {
                   // alert('Approval successful!');
                }

                else {
                    console.error("Approval failed:", resultBellaApprove.Err);
                    alert("Approval failed: " + resultBellaApprove.Err);
                }
            }
            let bella_ledger = process.env.CANISTER_ID_ICRC1_LEDGER_CANISTER;
            let tommy_ledger = process.env.CANISTER_ID_TOMMY_ICRC1_LEDGER;
            let bella_principal = Principal.fromText(bella_ledger);
            let tommy_principal = Principal.fromText(tommy_ledger);
            // Call the backend function
            console.log(amountBellaFormatted);
            const backendResponse = await mercx_Actor.sell(amountBellaFormatted, bella_principal, tommyAmount, tommy_principal);
            setInputBella("");
            setAmountTommy('0.0');
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
    }, [principal, Tommybalance, balance]);

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
                                Sell Bella and get Tommy
                            </p>
                        </div>
                        <div className="p-4">
                            <div className="p-4 mt-4 rounded-md shadow-md">
                                {/* <TokenData TokenBalance={!isAuthenticated ? `0` : Icpbalance.toString()} TokenName="Tommy" TokenLogo={"./favicon.ico"} /> */}
                                <TokenData TokenBalance={balance} TokenName={tokenName} TokenLogo={"./Bella.jpeg"} />
                                <input
                                    type="number"
                                    min="0.0001"
                                    step="0.0001"
                                    inputMode="decimal"
                                    name="amount"
                                    id="amount"
                                    value={inputBella}
                                    onFocus={(e) => e.target.select()}
                                    onChange={(e) => handleAmountChange(e)}
                                    className="block w-full text-right outline-0 text-gray-200 bg-inherit"
                                    //   disabled={token?.address ? false : true}
                                    placeholder="0.0"
                                />
                                {inputError && <p className="text-red-500 text-sm mt-2">{inputError}</p>}
                            </div>
                            <div className="p-4 mt-4 rounded-md shadow-md">

                                <TokenData TokenBalance={!isAuthenticated ? `0` : Tommybalance.toString()} TokenName="Tommy" TokenLogo={"./Tommy.JPG"} />
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
                                    ) : amountTommy} {/* Assuming amountMercx is a number, format it to four decimal places */}
                                </label>
                            </div>
                            {fetchingRateDown && amountTommy !== "" && (
                                <div className="text-sm font-medium text-center text-gray-200 pt-5">
                                    1 Bella = 10 Tommy
                                </div>
                            )}
                        </div>

                        <div className="flex justify-center">
                            <button
                                type="button"
                                className="place-content-center py-2 px-6 text-sm font-bold text-white bg-indigo-600 rounded-md bg-opacity-85 hover:bg-opacity-90 disabled:bg-indigo-400"
                                disabled={!isAuthenticated || inputBella === '0' || inputBella === '' || isloadingRate || !notSwapped}
                                onClick={() => handleIcpApprove()}
                            >
                                {!isAuthenticated ? "Connect your wallet" :
                                    inputBella === '0' || inputBella === '' ? "Enter an amount" :
                                        !notSwapped ? "Processing..." : // This line checks if notSwapped is false
                                            "Sell"}

                            </button>

                        </div>
                        {/*transfer fees*/}

                        <div className=" p-2 m-2 border-gray-700 bg-gray-800 flex justify-center ">

                            <dl className="flex items-center gap-4">
                                <dt className="text-sm font-normal text-gray-400">Network Fees</dt>
                                <dd className="text-sm font-medium text-white">0.0002 BELLA</dd>
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

export default Sell;
