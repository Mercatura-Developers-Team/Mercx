import React, { useState, useEffect } from "react";
import { mercx_backend } from "declarations/mercx_backend";
import { Principal } from "@dfinity/principal"; // Import Principal
//import { AuthClient } from "@dfinity/auth-client";
import MyNavbar from "./Navbar";
//import LoggedOut from "./LoggedOut";
import { useAuth, AuthProvider } from "./use-auth-client";
//import LoggedIn from "./LoggedIn";

import './index.css';
function App() {
  const [tokenName, setTokenName] = useState("");
  const [logoUrl, setLogoUrl] = useState("");
  const [balance, setBalance] = useState(0n); // Keep balance as BigInt
  const [transactions, setTransactions] = useState([]); // Initialize as empty array
  const [accountTransactions, setAccountTransactions] = useState([]);
  const { isAuthenticated, identity } = useAuth(); //isAuthenticated, which is a boolean indicating whether the user is currently authenticated, and identity, which likely contains data about the authenticated user.
  //const { login } = useAuth();
  const { whoamiActor, logout } = useAuth(); 
  const { principal } = useAuth(); 
 

  // Fetch token details (name and logo) and user balance on load
  async function fetchData(principalId) {
    try {
      // Use principalId directly if it's a Principal object
      const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

      // Fetch token name
      const name = await whoamiActor.get_token_name();
      setTokenName(name);

      // Fetch logo URL
      const logo = await whoamiActor.get_logo_url();
      setLogoUrl(logo);

      // Fetch user balance
      const balanceResult = await whoamiActor.check_balance({
        owner, // Use the Principal object directly
        subaccount: [],
      });
      setBalance(balanceResult);

      // Fetch latest transactions
      const txResponse = await whoamiActor.get_transactions(0, 50);
      if (txResponse?.Ok?.transactions) {
        setTransactions(txResponse.Ok.transactions);
      }

      // Fetch account transactions
      const accountTransactionsArgs = {
        max_results: 10n,
        start: [],
        account: {
          owner, // Use the Principal object directly
          subaccount: [],
        },
      };

      const accountTxResponse = await whoamiActor.get_account_transactions(
        accountTransactionsArgs.account,
        accountTransactionsArgs.start,
        accountTransactionsArgs.max_results
      );

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
        fetchData(principal); // Call fetchData only when 'result' (principalId) is available
      }
    }, [principal]);

  // useEffect(() => {
  //   fetchData();
  //   console.log(transactions)
  // }, []);

  return (

    <div className="min-h-screen bg-gray-100">
      <MyNavbar />
      <main className="bg-blue-900 text-white p-4">
        <section className="bg-white text-gray-900 rounded-lg shadow p-4 m-4">
          <h2 className="text-lg font-bold">Your Balance</h2>
          <p className="text-xl">{balance.toString()} {tokenName}</p>
        </section>
  {/* <section>
  <h2>Transaction History</h2>
  <ul>
    {transactions.length > 0 ? (
      transactions.map((tx, index) => (
        <li key={index}>
          <p>Type: {tx.kind || "N/A"}</p>

          <p>
            Amount: 
            {tx.transfer?.[0]?.amount 
              ? tx.transfer[0].amount.toString() // Accessing the first transfer's amount
              : tx.mint?.[0]?.amount 
              ? tx.mint[0].amount.toString() // Accessing the first mint's amount
              : "N/A"}
          </p>

          <p>
            Timestamp: 
            {tx.timestamp 
              ? new Date(Number(tx.timestamp) / 1000000).toLocaleString() 
              : "N/A"}
          </p>

          <p>
            From: 
            {tx.transfer?.[0]?.from?.owner 
              ? tx.transfer[0].from.owner.toText() 
              : "N/A"}
          </p>

          <p>
            To: 
            {tx.transfer?.[0]?.to?.owner 
              ? tx.transfer[0].to.owner.toText() 
              : tx.mint?.[0]?.to?.owner 
              ? tx.mint[0].to.owner.toText() 
              : "N/A"}
          </p>
        </li>
      ))
    ) : (
      <p>No transactions found.</p>
    )}
  </ul>
</section>


 */}

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
  <h2 className="font-bold text-lg">Transfer Tokens</h2>
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
        const transferResult = await whoamiActor.transfer({
          amount,
          to_account: {
            owner: recipientPrincipal,
            subaccount: [],
          },
          from_account: {
            owner: senderPrincipal,  // Use the user's principal here
            subaccount: [],  // Adjust if using subaccounts
          },
        });

        // Check if the transfer was successful
        if ("Ok" in transferResult) {
          alert("Transfer successful: Block Index " + transferResult.Ok);
          fetchData(principal);
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
  <AuthProvider>
    <App />
  </AuthProvider>
);


