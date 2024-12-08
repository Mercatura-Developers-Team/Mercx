import React, { useState, useEffect } from "react";

import { Principal } from "@dfinity/principal"; // Import Principal


import { useAuth } from "./use-auth-client";
import './index.css';

function Transcations() {
  const [tokenName, setTokenName] = useState("");
  const [icptokenName, setIcpTokenName] = useState("");
  const [logoUrl, setLogoUrl] = useState("");
  const [balance, setBalance] = useState(0n); // Keep balance as BigInt
  const [Icpbalance, setIcpBalance] = useState(0n); // Keep balance as BigInt
  const [accountTransactions, setAccountTransactions] = useState([]);
  const { whoamiActor, icrcIndexActor, icpActor, isAuthenticated } = useAuth();
  const { principal } = useAuth();



  // Fetch token details (name and logo) and user balance on load
  async function fetchData(principalId) {
    try {
      // Use principalId directly if it's a Principal object
      const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

      // Fetch token name
      const name = await whoamiActor.icrc1_name();
      setTokenName(name);

      const icptokenname = await icpActor.icrc1_symbol();
      setIcpTokenName(icptokenname);

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

      // Fetch latest transactions
      // const txResponse = await whoamiActor.get_transactions(0, 50);
      // if (txResponse?.Ok?.transactions) {
      //   setTransactions(txResponse.Ok.transactions);
      // }

      // Fetch account transactions
      const accountTransactionsArgs = {
        account: {
          owner,  // Principal object directly
          subaccount: [],  // Optional subaccount
        },
        start: [],  // Adjust the starting point for pagination
        max_results: 30n,  // Pass max_results inside the same record
      };

      const accountTxResponse = await icrcIndexActor.get_account_transactions(accountTransactionsArgs);

      if (accountTxResponse?.Ok?.transactions) {
        setAccountTransactions(accountTxResponse.Ok.transactions);
      }
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }

  // Fetch data once the principal ID is available
  useEffect(() => {
    if (principal) {
      fetchData(principal);  // Fetch data when principal and actor are available
    }
  }, [principal, balance, Icpbalance, accountTransactions]);




  return (

    <div className="min-h-screen bg-gray-100">
      {/* <MyNavbar /> */}
      <main className="bg-blue-900 text-white p-4">
        <section className="bg-white text-gray-900 rounded-lg shadow p-4 m-4">
          <h2 className="text-lg font-bold">Your Balance</h2>
          <p className="text-xl">{!isAuthenticated ? `0 ${tokenName}` : `${balance.toString()} ${tokenName}`}</p>
          <p className="text-xl"> {!isAuthenticated ? `0 ${icptokenName}` : `${Icpbalance.toString()} ${icptokenName}`}</p>
        </section>
 

        <section className="p-4 m-4 bg-white rounded-lg shadow text-gray-900">
          <h2 className="text-lg font-bold">Account Transaction History</h2>
          <ul>
            {accountTransactions.length > 0 ? (
              accountTransactions.map((tx, index) => {
                //  console.log("Transaction data:", tx);

                const { transaction } = tx;
                const { kind, timestamp } = transaction;

                let amount = "N/A";
                let fromOwner = "N/A";
                let toOwner = "N/A";

                if (transaction.transfer && transaction.transfer.length > 0) {
                  const transfer = transaction.transfer[0];
                  amount = transfer.amount.toString();
                  fromOwner = transfer.from.owner.toText();
                  toOwner = transfer.to.owner.toText();
                } else if (transaction.mint && transaction.mint.length > 0) {
                  const mint = transaction.mint[0];
                  amount = mint.amount.toString();
                  toOwner = mint.to.owner.toText();
                }

                return (
                  <li key={index} className="p-2 border-b border-gray-200">
                    <p>Transaction ID: {tx.id.toString()}</p>
                    <p>Type: {kind || "N/A"}</p>
                    <p>Amount: {amount}</p>
                    <p>
                      Timestamp:{" "}
                      {timestamp
                        ? new Date(
                          Number(timestamp / 1_000_000n)
                        ).toLocaleString()
                        : "N/A"}
                    </p>
                    <p>From: {fromOwner}</p>
                    <p>To: {toOwner}</p>
                  </li>
                );
              })
            ) : (
              <p>No account transactions found.</p>
            )}
          </ul>
        </section>



        <section className="p-4 m-4 bg-white rounded-lg shadow  text-gray-900">
          <h2 className="font-bold text-lg">Transfer Bella</h2>
          <form className="flex flex-col gap-4"
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
            <label>
              To Account (Principal ID):
              <input
                type="text"
                name="to"
                placeholder="Enter recipient's principal"
                required
                className="p-2 border rounded"
              />
            </label>
            <label>
              Amount:
              <input
                type="number"
                name="amount"
                placeholder="Amount to transfer"
                required
                className="p-2 border rounded"
              />
            </label>
            <button type="submit" className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">Send</button>
          </form>
        </section>

        <section className="p-4 m-4 bg-white rounded-lg shadow  text-gray-900">
          <h2 className="font-bold text-lg">Transfer ICP</h2>
          <form className="flex flex-col gap-4"
            onSubmit={async (event) => {
              event.preventDefault();

              try {
                const toAccount = event.target.elements.to.value.trim();
                const amount = BigInt(event.target.elements.amount.value);

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
                  fetchData(principal);  // Refresh balance and transactions
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
            <label>
              To Account (Principal ID):
              <input
                type="text"
                name="to"
                placeholder="Enter recipient's principal"
                required
                className="p-2 border rounded"
              />
            </label>
            <label>
              Amount:
              <input
                type="number"
                name="amount"
                placeholder="Amount to transfer"
                required
                className="p-2 border rounded"
              />
            </label>
            <button type="submit" className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">Send</button>
          </form>
        </section>





      </main>

    </div>

  );
}

export default () => (

  <Transcations />

);


