import React from 'react'

import { useAuth } from "./use-auth-client";
import { Principal } from "@dfinity/principal";


const Transfer = () => {
    const { whoamiActor, icpActor } = useAuth();
    const { principal } = useAuth();
    return (
        <div className='md:py-10 bg-gray-900 py-8'>

            <section className=" p-4 m-4  rounded-lg shadow  bg-slate-800 text-gray-900 border border-gray-700   ">
                <h2 className="font-bold text-lg text-white">Transfer Bella</h2>
                <form className="  flex flex-col gap-4  "
                    onSubmit={async (event) => {
                        event.preventDefault();

                        try {
                            const toAccount = event.target.elements.to.value.trim();
                            const amount = BigInt(event.target.elements.amount.value * 1e8);

                            if (!toAccount || amount <= 0n) {
                                alert("Please provide valid inputs");
                                return;
                            }

                            // Validate recipient's Principal ID format
                            let recipientPrincipal;
                            try {
                                recipientPrincipal = Principal.fromText(toAccount);
                            } catch (err) {
                                alert("Invalid Principal ID format. Please provide a valid Principal ID.");
                                return;
                            }

                            // Use authenticated user's principal as the sender (from_account)
                            const senderPrincipal = principal;  // principal should be available from useAuthClient()

                            // Call the backend transfer function
                            const transferResult = await whoamiActor.icrc1_transfer({
                                to: {
                                    owner: recipientPrincipal,   // Use `to` instead of `to_account`
                                    subaccount: [],              // Optional, set subaccount if required
                                },
                                fee: [],                        // Optional fee, use [] if not needed
                                memo: [],                       // Optional memo, use [] if not needed
                                from_subaccount: [],            // Optional, set sender's subaccount if required
                                created_at_time: [],            // Optional, use [] if no specific timestamp is provided
                                amount,                         // The amount to transfer
                            });

                            // Check if the transfer was successful
                            if ("Ok" in transferResult) {
                                alert("Transfer successful: Block Index " + transferResult.Ok);
                                // fetchData(principal);  // Refresh balance and transactions
                            } else {
                                console.error("Transfer failed: ", transferResult.Err);
                                alert("Transfer failed: " + transferResult.Err);
                            }
                        } catch (error) {
                            console.error("Transfer failed: ", error);
                            alert("Transfer failed: " + error.message);
                        }
                    }}
                >
                    <label className="m-4 block text-sm font-medium text-white dark:text-red w-full ">
                        To Account (Principal ID):
                        <input
                            type="text"
                            name="to"
                            placeholder="Enter recipient's principal"
                            required
                            className=" w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-primary-500 focus:ring-primary-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder:text-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500"
                        />
                    </label>
                  
                        <label className="mb-2 block text-sm font-medium text-gray-900 dark:text-white w-full">
                            Amount:
                            <input
                                type="number"
                                name="amount"
                                step="any"
                                placeholder="Amount to transfer"
                                required
                                className="w-full rounded-lg border border-gray-300 p-2.5 bg-gray-50 text-sm text-gray-900 focus:border-primary-500 focus:ring-primary-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder:text-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500"
                            />
                        </label>
                    
                    <button type="submit" className="flex w-full items-center justify-center rounded-lg bg-primary-700 px-5 py-2.5 text-sm font-medium text-white hover:bg-primary-800 focus:outline-none focus:ring-4  focus:ring-primary-300 dark:bg-primary-600 dark:hover:bg-primary-700 dark:focus:ring-primary-800">Send</button>
                </form>
            </section>



            <section className="p-4 m-4  rounded-lg shadow  bg-slate-800 text-gray-900 border border-gray-700  ">
                <h2 className="font-bold text-lg text-white">Transfer ICP</h2>
                <form className="flex flex-col gap-4 bg-slate-800"
                    onSubmit={async (event) => {
                        event.preventDefault();

                        try {
                            const toAccount = event.target.elements.to.value.trim();

                            const amount = BigInt(event.target.elements.amount.value * 1e8);

                            if (!toAccount || amount <= 0n) {
                                alert("Please provide valid inputs");
                                return;
                            }

                            // Validate recipient's Principal ID format
                            let recipientPrincipal;
                            try {
                                recipientPrincipal = Principal.fromText(toAccount);
                            } catch (err) {
                                alert("Invalid Principal ID format. Please provide a valid Principal ID.");
                                return;
                            }

                            // Use authenticated user's principal as the sender (from_account)
                            const senderPrincipal = principal;  // principal should be available from useAuthClient()

                            // Call the backend transfer function
                            const transferResult = await icpActor.icrc1_transfer({
                                to: {
                                    owner: recipientPrincipal,   // Use `to` instead of `to_account`
                                    subaccount: [],              // Optional, set subaccount if required
                                },
                                fee: [],                        // Optional fee, use [] if not needed
                                memo: [],                       // Optional memo, use [] if not needed
                                from_subaccount: [],            // Optional, set sender's subaccount if required
                                created_at_time: [],            // Optional, use [] if no specific timestamp is provided
                                amount,                         // The amount to transfer
                            });

                            // Check if the transfer was successful
                            if ("Ok" in transferResult) {
                                alert("Transfer successful: Block Index " + transferResult.Ok);

                            } else {
                                console.error("Transfer failed: ", transferResult.Err);
                                alert("Transfer failed: " + transferResult.Err);
                            }
                        } catch (error) {
                            console.error("Transfer failed: ", error);
                            alert("Transfer failed: " + error.message);
                        }
                    }}
                >
                    <label className='mb-2 block text-sm font-medium text-gray-900 dark:text-white w-full'>
                        To Account (Principal ID):
                        <input
                            type="text"
                            name="to"
                            placeholder="Enter recipient's principal"
                            required
                            className="w-full rounded-lg border border-gray-300 p-2.5 bg-gray-50 text-sm text-gray-900 focus:border-primary-500 focus:ring-primary-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder:text-gray-400 dark:focus:border-primary-500 dark:focus:ring-primary-500 "
                        />
                    </label>
                    <label className='mb-2 block text-sm font-medium text-gray-900 dark:text-white w-full '>
                        Amount:
                        <input
                            type="number"
                            name="amount"
                            step="any"
                            placeholder="Amount to transfer"
                            required
                            className="w-full rounded-lg border p-2.5 text-sm  border-gray-600 bg-gray-700 text-white placeholder:text-gray-400 focus:border-primary-500 focus:ring-primary-500"
                        />
                    </label>
                    <button type="submit" className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">Send</button>
                </form>
            </section>




        </div>
        );
}

export default Transfer;