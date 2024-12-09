import React, { useState, useEffect } from "react";
import { Principal } from "@dfinity/principal"; // Import Principal
import { useAuth } from "./use-auth-client";


function Transcations() {
  const [accountTransactions, setAccountTransactions] = useState([]);
  const { icrcIndexActor } = useAuth();
  const { principal } = useAuth();



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
  }, [principal, accountTransactions]);


  return (
   
    <div className="min-h-screen  justify-center bg-gray-900 flex">
    <div className="m-10 w-full max-w-screen-xl bg-slate-800 rounded-lg shadow-lg p-8 ">

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

                    if (transaction.transfer && transaction.transfer.length > 0) {
                      const transfer = transaction.transfer[0];
                      amount = Number(transfer.amount) / 1e8;
                      fromOwner = transfer.from.owner.toText();
                      toOwner = transfer.to.owner.toText();
                      timestamp = new Date(Number(transaction.timestamp / 1_000_000n)).toLocaleString();

                      // Check if the owner ID matches the specific ID and replace name
                      if (fromOwner === "b77ix-eeaaa-aaaaa-qaada-cai") fromOwner = "MercX Link";
                      if (toOwner === "b77ix-eeaaa-aaaaa-qaada-cai") toOwner = "MercX Link";

                    } else if (transaction.mint && transaction.mint.length > 0) {
                      const mint = transaction.mint[0];
                      amount = Number(mint.amount) / 1e8;
                      toOwner = mint.to.owner.toText();
                      timestamp = new Date(Number(transaction.timestamp / 1_000_000n)).toLocaleString();

                      // Check if the owner ID matches the specific ID and replace name
                      if (toOwner === "b77ix-eeaaa-aaaaa-qaada-cai") toOwner = "MercX Link";
                    }

                    return (
                      <tr key={index}>
                        <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-white sm:pl-0">
                          {(index + 1).toString()}
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
                    <td colSpan="6" className="text-center text-white py-4">
                      No account transactions found.
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

export default () => (
  <Transcations />
);