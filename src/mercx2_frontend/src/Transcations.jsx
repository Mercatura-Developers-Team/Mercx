import React, { useState, useEffect } from "react";
import { Principal } from "@dfinity/principal"; // Import Principal
import { useAuth } from "./use-auth-client";

function Transactions() {
  const [accountTransactions, setAccountTransactions] = useState([]);
  const [usernameMap, setUsernameMap] = useState({}); // Map of Principal IDs to usernames
  const { icrcIndexActor, fxmxIndexActor, kycActor } = useAuth();
  const { principal } = useAuth();

  // Function to get username from principal using get_username_by_principal
  async function getUsername(principalId) {
    try {
      // Validate the Principal ID
      const principal = Principal.fromText(principalId);
      console.log("Valid Principal:", principal.toString());

      // Call the function that takes a Principal and returns a username
      const usernameResponse = await kycActor.get_username_by_principal(principal);
      console.log("usernameResponse", usernameResponse);

      // Extract the username from the response
      if (usernameResponse.Ok) {
        return usernameResponse.Ok; // Return the username
      } else if (usernameResponse.Err) {
        console.log("User not found for Principal:", principalId);
        return principalId; // Fallback to Principal ID
      }
    } catch (error) {
      console.error("Error fetching username:", error);
      return principalId; // Fallback to Principal ID in case of an error
    }
  }

  // Fetch token details (name and logo) and user balance on load
  async function fetchData(principalId) {
    try {
      // Use principalId directly if it's a Principal object
      const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

      // Fetch account transactions
      const accountTransactionsArgs = {
        account: {
          owner,  // Principal object directly
          subaccount: [],  // Optional subaccount
        },
        start: [],  // Adjust the starting point for pagination
        max_results: 30n,  // Pass max_results inside the same record
      };

      const accountTxResponse = await fxmxIndexActor.get_account_transactions(accountTransactionsArgs);

      if (accountTxResponse?.Ok?.transactions) {
        setAccountTransactions(accountTxResponse.Ok.transactions);

        // Extract unique Principals from transactions
        const uniquePrincipals = new Set();
        accountTxResponse.Ok.transactions.forEach((tx) => {
          if (tx.transaction.transfer && tx.transaction.transfer.length > 0) {
            const fromPrincipal = tx.transaction.transfer[0].from.owner.toText();
            const toPrincipal = tx.transaction.transfer[0].to.owner.toText();

            // Validate Principals before adding to the set
            try {
              Principal.fromText(fromPrincipal);
              uniquePrincipals.add(fromPrincipal);
            } catch (error) {
              console.error("Invalid fromPrincipal:", fromPrincipal, error);
            }

            try {
              Principal.fromText(toPrincipal);
              uniquePrincipals.add(toPrincipal);
            } catch (error) {
              console.error("Invalid toPrincipal:", toPrincipal, error);
            }

          } else if (tx.transaction.mint && tx.transaction.mint.length > 0) {
            const toPrincipal = tx.transaction.mint[0].to.owner.toText();

            // Validate Principal before adding to the set
            try {
              Principal.fromText(toPrincipal);
              uniquePrincipals.add(toPrincipal);
            } catch (error) {
              console.error("Invalid toPrincipal:", toPrincipal, error);
            }
          }
        });

        // Fetch usernames for all unique Principals
        const newUsernameMap = {};
        for (const principalId of uniquePrincipals) {
          const username = await getUsername(principalId);
          newUsernameMap[principalId] = username; // Store the username or fallback to Principal ID
        }

        // Update the username map state
        setUsernameMap(newUsernameMap);
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
  }, [principal]);

  return (
    <div className="min-h-screen justify-center bg-gray-900 flex">
      <div className="m-10 w-full max-w-screen-xl bg-slate-800 rounded-lg shadow-lg p-8">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-300">
            <thead>
              <tr>
                <th className="whitespace-nowrap py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-white sm:pl-0">
                  Transaction
                </th>
                <th className="px-3 py-3.5 text-left text-sm font-semibold text-white">
                  Type
                </th>
                <th className="whitespace-nowrap px-3 py-3.5 text-left text-sm font-semibold text-white">
                  Amount
                </th>
                <th className="whitespace-nowrap px-3 py-3.5 text-left text-sm font-semibold text-white">
                  Timestamp
                </th>
                <th className="whitespace-nowrap px-3 py-3.5 text-left text-sm font-semibold text-white">
                  From
                </th>
                <th className="whitespace-nowrap px-3 py-3.5 text-left text-sm font-semibold text-white">
                  To
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200">
              {accountTransactions.length > 0 ? (
                accountTransactions.map((tx, index) => {
                  const transaction = tx.transaction;
                  const kind = transaction.kind;
                  let amount = "N/A";
                  let fromOwner = "N/A";
                  let toOwner = "N/A";
                  let timestamp = "N/A";
                  const maxIndex = accountTransactions.length;

                  if (transaction.transfer && transaction.transfer.length > 0) {
                    const transfer = transaction.transfer[0];
                    amount = Number(transfer.amount) / 1e8;
                    fromOwner = transfer.from.owner.toText();
                    toOwner = transfer.to.owner.toText();
                    timestamp = new Date(Number(transaction.timestamp / 1_000_000n)).toLocaleString();

                    // Replace Principal IDs with usernames from the usernameMap
                    fromOwner = usernameMap[fromOwner] || fromOwner;
                    toOwner = usernameMap[toOwner] || toOwner;

                    // Check for specific Principal ID and replace with "MercX Link"
                    if (fromOwner === "a3shf-5eaaa-aaaaa-qaafa-cai") {
                      fromOwner = "MercX Link";
                    }
                    if (toOwner === "a3shf-5eaaa-aaaaa-qaafa-cai") {
                      toOwner = "MercX Link";
                    }

                  } else if (transaction.mint && transaction.mint.length > 0) {
                    const mint = transaction.mint[0];
                    amount = Number(mint.amount) / 1e8;
                    toOwner = mint.to.owner.toText();
                    timestamp = new Date(Number(transaction.timestamp / 1_000_000n)).toLocaleString();

                    // Replace Principal ID with username from the usernameMap
                    toOwner = usernameMap[toOwner] || toOwner;
                  }

                  return (
                    <tr key={index}>
                      <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-white sm:pl-0">
                        {maxIndex - index}
                      </td>
                      <td className="whitespace-nowrap px-3 py-4 text-sm text-white">
                        {kind || "N/A"}
                      </td>
                      <td className="whitespace-nowrap px-3 py-4 text-sm text-white">
                        {amount}
                      </td>
                      <td className="whitespace-nowrap px-3 py-4 text-sm text-white">
                        {timestamp}
                      </td>
                      <td className="whitespace-nowrap px-3 py-4 text-sm text-white">
                        {fromOwner}
                      </td>
                      <td className="whitespace-nowrap px-3 py-4 text-sm text-white">
                        {toOwner}
                      </td>
                    </tr>
                  );
                })
              ) : (
                <tr>
                  <td colSpan="6" className="text-center text-gray-500 py-4">
                    No account transactions found
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

export default Transactions;