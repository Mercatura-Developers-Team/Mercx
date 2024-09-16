import { useState, useEffect } from "react";
import { mercx_backend } from "declarations/mercx_backend";
import { Principal } from "@dfinity/principal"; // Import Principal

function App() {
  const [tokenName, setTokenName] = useState("");
  const [logoUrl, setLogoUrl] = useState("");
  const [balance, setBalance] = useState(0n); // Keep balance as BigInt
  const [transactions, setTransactions] = useState([]); // Initialize as empty array

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
          owner: Principal.fromText("cy4ps-czgf3-cr64v-goyxx-vzgbp-2ilng-q4k4s-7hex3-zqaer-yq4md-3qe"),
          subaccount: [],
        });
        setBalance(balanceResult);

        // Fetch latest transactions
        const txResponse = await mercx_backend.get_transactions(0, 10); // Adjust length as needed
        if (txResponse && txResponse.transactions) {
          setTransactions(txResponse.transactions);
        }
      } catch (error) {
        console.error("Error fetching data: ", error);
      }
    }
    fetchData();
  }, []);

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

      <section>
        <h2>Transaction History</h2>
        <ul>
          {transactions.length > 0 ? (
            transactions.map((tx, index) => (
              <li key={index}>
                <p>Type: {tx.kind}</p>
                <p>Amount: {tx.amount ? tx.amount.toString() : "N/A"}</p>
                <p>Timestamp: {new Date(Number(tx.timestamp / 1000000)).toLocaleString()}</p>
                <p>From: {tx.from_account ? tx.from_account.owner.toText() : "N/A"}</p>
                <p>To: {tx.to_account ? tx.to_account.owner.toText() : "N/A"}</p>
              </li>
            ))
          ) : (
            <p>No transactions found.</p>
          )}
        </ul>
      </section>

      <section>
        <h2>Transfer Tokens</h2>
        <form
          onSubmit={async (event) => {
            event.preventDefault();
            const toAccount = event.target.elements.to.value;
            const amount = BigInt(event.target.elements.amount.value);

            try {
              const transferResult = await mercx_backend.transfer({
                amount,
                to_account: {
                  owner: Principal.fromText(toAccount),
                  subaccount: [],
                },
              });
              alert("Transfer successful: Block Index " + transferResult);
            } catch (error) {
              console.error("Transfer failed: ", error);
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
