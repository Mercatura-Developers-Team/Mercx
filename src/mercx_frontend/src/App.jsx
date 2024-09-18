import { useState, useEffect } from "react";
import { mercx_backend } from "declarations/mercx_backend";
import { Principal } from "@dfinity/principal"; // Import Principal
//import { AuthClient } from "@dfinity/auth-client";
import { fetchTransactions } from './api/icrc1Api'; // adjust


function App() {
  const [tokenName, setTokenName] = useState("");
  const [logoUrl, setLogoUrl] = useState("");
  const [balance, setBalance] = useState(0n); // Keep balance as BigInt
  const [transactions, setTransactions] = useState([]); // Initialize as empty array


  const principalId = 'be2us-64aaa-aaaaa-qaabq-cai'; // Set the appropriate principal ID

// useEffect(() => {
//   async function authenticateUser() {
//     const authClient = await AuthClient.create();
    
//     if (await authClient.isAuthenticated()) {
//       const identity = authClient.getIdentity();
//       const principal = identity.getPrincipal().toText();
//       console.log("Authenticated user principal:", principal);
//     } else {
//       await authClient.login({
//         identityProvider: "https://identity.ic0.app/#authorize",
//         onSuccess: () => {
//           window.location.reload(); // Reload the page to get authenticated state
//         }
//       });
//     }
//   }
//   authenticateUser();
// }, []);

  // Fetch token details (name and logo) and user balance on load
  useEffect(() => {
    async function fetchData() {
      try {
        // Fetch token name
        const name = await mercx_backend.get_token_name();
        setTokenName(name);

        // Fetch logo URL
        const logo = await mercx_backend.get_logo_url();
        setLogoUrl(logo);

        // Fetch user balance (replace <YourPrincipalHere> with actual principal)
        const balanceResult = await mercx_backend.check_balance({
          owner: Principal.fromText("bd3sg-teaaa-aaaaa-qaaba-cai"),
          subaccount: [],
        });
        setBalance(balanceResult);

                // Fetch latest transactions
      const txResponse = await mercx_backend.get_transactions(0, 10); // Adjust length as needed
      if (txResponse?.Ok?.transactions) {
        setTransactions(txResponse.Ok.transactions); // Correctly access transactions
        //console.log(txResponse.Ok.transactions);
      } else {
        console.error("No transactions in response:", txResponse);
      }
    } catch (error) {
      console.error("Error fetching data: ", error);
    }
  }
  fetchData();
// console.log(transactions)
  }, []);

  useEffect(() => {
    fetchTransactions(Principal.fromText(principalId), 10)
        .then(setTransactions)
        .catch(error => console.error('Failed to fetch transactions:', error));
}, [principalId]);

  return (
    <main
      style={{
        height: "100%",
        minHeight: "100vh",
        backgroundColor: "#001f3f", // Dark blue background
        color: "white", // White text
      }}
    >
      <header>
        <img src={logoUrl} alt={`${tokenName} Logo`} width="100" />
        <h1>{tokenName} - Token Dashboard</h1>
      </header>

      <section>
        <h2>Your Balance</h2>
        <p>{balance.toString()} {tokenName}</p> {/* Convert BigInt to string */}
      </section>
      <div>
            <h1>Transactions</h1>
            {transactions.map((tx, index) => (
                <div key={index}>
                    <p>Transaction ID: {tx.id}</p>
                    <p>Type: {tx.kind}</p>
                    <p>Timestamp: {new Date(Number(tx.timestamp) / 1000000).toLocaleString()}</p>
                    {/* Add more details as needed */}
                </div>
            ))}
        </div>
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
</section> */}






      <section>
        <h2>Transfer Tokens</h2>
        <form
          onSubmit={async (event) => {
            event.preventDefault();

            try {
              // Retrieve and validate the Principal ID
              const toAccount = event.target.elements.to.value.trim(); // Trim to remove extra spaces
              const amount = BigInt(event.target.elements.amount.value); // Convert amount to BigInt

              if (!toAccount || !amount) {
                alert("Please provide valid inputs");
                return;
              }

              // Validate Principal ID format
              let principal;
              try {
                principal = Principal.fromText(toAccount);
              } catch (err) {
                alert("Invalid Principal ID format. Please provide a valid Principal ID.");
                return;
              }

              // Call the backend transfer function
              const transferResult = await mercx_backend.transfer({
                amount,
                to_account: {
                  owner: principal, // Use the properly formatted Principal here
                  subaccount: [], // Assuming no subaccount for simplicity
                },
              });

              alert("Transfer successful: Block Index " + transferResult);
            } catch (error) {
              console.error("Transfer failed: ", error);
              alert("Transfer failed: " + error.message); // Display the error message
            }
          }}
        >
          <label>
            To Account (Principal ID):
            <input type="text" name="to" placeholder="Enter recipient's principal" required />
          </label>
          <label>
            Amount:
            <input type="number" name="amount" placeholder="Amount to transfer" required />
          </label>
          <button type="submit">Send</button>
        </form>


      </section>
    </main>
  );
}

export default App;
