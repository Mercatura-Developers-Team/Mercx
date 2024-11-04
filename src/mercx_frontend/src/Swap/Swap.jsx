import React, { useEffect } from 'react';
import { Fragment, useState } from "react";
import { Dialog, Transition } from "@headlessui/react";
import TokenData from './TokenData';
import { useAuth } from '.././use-auth-client';
import { Principal } from "@dfinity/principal"; // Import Principal


const Swap = () => {
    const { whoamiActor, icpActor, mercx_Actor } = useAuth();
    const [Icpbalance, setIcpBalance] = useState(0n); // Keep balance as BigInt
    const { principal } = useAuth();
    const [balance, setBalance] = useState(0n);
    const [tokenName, setTokenName] = useState("");
    const [inputIcp, setInputIcp] = useState('');
    const [amountMercx, setAmountMercx] = useState('0.0');
    const [logoUrl, setLogoUrl] = useState("");//not used 

    const handleAmountChange = (e) => {
        setInputIcp(e.target.value);
       // setAmountMercx(Number(e.target.value) / 0.00125);
    setAmountMercx(Number(e.target.value) );

    }

    async function fetchData(principalId) {
        try {
            // Use principalId directly if it's a Principal object
            const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

            //   // Fetch token name
            const name = await whoamiActor.icrc1_name();
            setTokenName(name);

            // Fetch logo URL
            const logo = await whoamiActor.icrc1_metadata();
            setLogoUrl(logo);

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

        const icp_swap_canister_id = "b77ix-eeaaa-aaaaa-qaada-cai"; // Placeholder for actual canister ID
        let m = Math.floor(inputIcp * 1e8);
        let amount = Number(m); // Assume icpAmount is a string input from the user
        // Convert the user input into a Number, then multiply by 1e8 to convert ICP to e8s

        let amountFormatApprove = Math.floor(amount * 1e8); // Adding 10000 transferring fees if needed, and ensuring it's a Number
        try {

            const allowanceCheck = {
                account: { owner: principal, subaccount: [] },
                spender: { owner: Principal.fromText(icp_swap_canister_id), subaccount: [] }
            };
    
            console.log("Checking allowance with structure:", JSON.stringify(allowanceCheck));
            
            const currentAllowanceResult = await icpActor.icrc2_allowance(allowanceCheck);
            const currentAllowance = currentAllowanceResult.allowance;
    
            console.log("Current allowance:", currentAllowance);

            if ((BigInt(amountFormatApprove)) < Icpbalance ) {

            
            // Check if we already have enough allowance for the swap
            // const currentAllowance = await icpActor.icrc2_allowance({
            //     owner: principal,
            //     spender: Principal.fromText(icp_swap_canister_id),
            // });

             // Proceed with approval only if current allowance is less than needed
        // if (currentAllowance < BigInt(amountFormatApprove)){
            if (BigInt(currentAllowance) < BigInt(amountFormatApprove)) {

            const resultIcpApprove = await icpActor.icrc2_approve({
                spender: {
                    owner: Principal.fromText(icp_swap_canister_id),
                    subaccount: [],
                },
                amount: BigInt(amountFormatApprove),
                fee: [BigInt(10000)], // Optional fee, set as needed
                memo: [],  // Optional memo field
                from_subaccount: [],  // From subaccount, if any
                created_at_time: [],  // Specify if needed
                expected_allowance: [],  // Optional, specify expected allowance if needed
                expires_at: [],  // Specify if approval should expire
            });

            // If the approval was successful, call the backend function
            if (resultIcpApprove && "Ok" in resultIcpApprove) {
                alert('Approval successful!');
          //  }
            }
              
             else {
                console.error("Approval failed:", resultIcpApprove.Err);
                alert("Approval failed: " + resultIcpApprove.Err);
            }}
              // Call the backend function
              const backendResponse = await mercx_Actor.swap(amount);
              setInputIcp("0.0");
              setAmountMercx('0.0');
              console.log('Backend response:', backendResponse);
              fetchData(principal);
        } else {
            alert("Insufficient balance")
        }
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
        <div className="min-h-screen bg-white dark:bg-slate-750">
            <main>

                <div className="max-w-md mx-auto sm:px-6 lg:px-8 pt-8 lg:pt-14 2xl:pt-18">
                    <div className="shadow-xl rounded-3xl h-[480px] border-t-[1px] border-gray-100 dark:border-slate-800 dark:bg-slate-800">
                        <div className="border-b-[1px] border-gray-200 dark:border-gray-900 shadow-md p-3">
                            <p className="text-lg font-bold text-center dark:text-gray-200">
                                Swap
                            </p>
                            <p className="text-gray-500 dark:text-gray-300 text-center text-sm">
                                Swap ICP with MERCX
                            </p>
                        </div>
                        <div className="p-4">
                            <div className="p-4 mt-4 rounded-md shadow-md">
                                <TokenData TokenBalance={Icpbalance} TokenName="ICP" TokenLogo={"./favicon.ico"} />
                                <input
                                    type="number"
                                    min='0'
                                    inputMode="decimal"
                                    name="amount"
                                    id="amount"
                                    value={inputIcp}
                                    onFocus={(e) => e.target.select()}
                                    onChange={(e) => handleAmountChange(e)}
                                    className="block w-full text-right outline-0 text-gray-500 dark:text-gray-200 bg-inherit"
                                    //   disabled={token?.address ? false : true}
                                    placeholder="0.0"
                                />
                            </div>
                            <div className="p-4 mt-4 rounded-md shadow-md">
                                <TokenData TokenBalance={balance} TokenName={tokenName} TokenLogo={"/j.png"} />
                                <label
                                    type="number"
                                    min='0'
                                    inputMode="decimal"
                                    name="amount2"
                                    id="amount"
                                    //  value={amountMercx}
                                    onFocus={(e) => e.target.select()}
                                    //   onChange={(e) => onChangeInput(e.target.value, isTokenA)}
                                    className="block w-full pr-4 text-right outline-0 text-gray-500 dark:text-gray-200 bg-inherit"
                                    //   disabled={token?.address ? false : true}
                                    placeholder="0.0"
                                >
                                    {amountMercx}
                                </label>
                            </div>
                        </div>
                        <div className="flex justify-center">
                            <button
                                type="button"
                                className="place-content-center py-2 px-6 text-sm font-bold text-white bg-indigo-600 rounded-md bg-opacity-85 hover:bg-opacity-90 disabled:bg-indigo-400"
                                disabled={inputIcp === '0' || inputIcp === ''}
                                onClick={() => handleIcpApprove()}
                            >
                                {inputIcp === '0' || inputIcp === '' ? "Enter an amount" : "SWAP"}

                            </button>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    </>);
}

export default Swap;