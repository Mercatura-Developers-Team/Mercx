import React, { useState, useEffect } from "react";
//import { mercx_backend } from "declarations/mercx_backend";
import { Principal } from "@dfinity/principal"; // Import Principal
//import { AuthClient } from "@dfinity/auth-client";
import MyNavbar from "./Navbar";
import { useAuth, AuthProvider } from "./use-auth-client";
import './index.css';

function App() {
  const [tokenName, setTokenName] = useState("");
  const [icptokenName, setIcpTokenName] = useState("");
  const [logoUrl, setLogoUrl] = useState("");
  const [balance, setBalance] = useState(0n); // Keep balance as BigInt
  const [Icpbalance, setIcpBalance] = useState(0n); // Keep balance as BigInt
  const [transactions, setTransactions] = useState([]); // Initialize as empty array
  const [accountTransactions, setAccountTransactions] = useState([]);
 // const { isAuthenticated, identity } = useAuth(); //isAuthenticated, which is a boolean indicating whether the user is currently authenticated, and identity, which likely contains data about the authenticated user.
  //const { login } = useAuth();
  const { whoamiActor ,icrcIndexActor,icpActor,mercx_Actor} = useAuth(); 
  const { principal } = useAuth(); 
  const [icpAmount, setIcpAmount] = useState(''); //approval amount 

 

  // Fetch token details (name and logo) and user balance on load
  async function fetchData(principalId) {
    try {
      // Use principalId directly if it's a Principal object
      const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

      // Fetch token name
      const name = await whoamiActor.icrc1_name();
      setTokenName(name);

      const icptokenname=await icpActor.icrc1_name();
      setIcpTokenName(icptokenname);

      // Fetch logo URL
      const logo = await whoamiActor.icrc1_metadata();
      setLogoUrl(logo);

      // Fetch user balance
      const balanceResult = await whoamiActor.icrc1_balance_of({
        owner, // Use the Principal object directly
        subaccount: [],
      });
      setBalance(balanceResult);

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
      if (principal ) {
        fetchData(principal);  // Fetch data when principal and actor are available
      }
    }, [principal,balance,Icpbalance,accountTransactions]); 

  // useEffect(() => {
  //   fetchData();
  //   console.log(transactions)
  // }, []);


  const handleIcpApprove = async (e) => {
    e.preventDefault();
    const icp_swap_canister_id = "b77ix-eeaaa-aaaaa-qaada-cai"; // Placeholder for actual canister ID
    let m = Math.floor(icpAmount * 1e8); 
    let amount = Number(m); // Assume icpAmount is a string input from the user

    // Convert the user input into a Number, then multiply by 1e8 to convert ICP to e8s
    let amountFormatApprove= Math.floor(amount * 1e8); // Adding 10000 transferring fees if needed, and ensuring it's a Number

    try {
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

            // Call the backend function
            const backendResponse = await mercx_Actor.swap(amount);

            console.log('Backend response:', backendResponse);
        } else {
            console.error("Approval failed:", resultIcpApprove.Err);
            alert("Approval failed: " + resultIcpApprove.Err);
        }
    } catch (error) {
        console.error("Approval process failed:", error);
        alert('Approval failed: ' + error.message);
    }
};

  return (

    <div className="min-h-screen bg-gray-100">
      <MyNavbar />
      <main className="bg-blue-900 text-white p-4">
        <section className="bg-white text-gray-900 rounded-lg shadow p-4 m-4">
          <h2 className="text-lg font-bold">Your Balance</h2>
          <p className="text-xl">{balance.toString()} {tokenName}</p>
          <p className="text-xl">{Icpbalance.toString()} {icptokenName}</p>
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

      <form onSubmit={handleIcpApprove}>
                  <input
                    type="number"
                    placeholder="Enter ICP amount"
                    value={icpAmount}
                    onChange={e => setIcpAmount(e.target.value)}
                    className="text-gray-900 p-2 border rounded-lg"
                  />
                  <button type="submit" className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                   Exchange
                  </button>
                </form>

      <section className="p-4 m-4 bg-white rounded-lg shadow  text-gray-900">
  <h2 className="font-bold text-lg">Transfer Mercx</h2>
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
  <AuthProvider>
    <App />
  </AuthProvider>
);


