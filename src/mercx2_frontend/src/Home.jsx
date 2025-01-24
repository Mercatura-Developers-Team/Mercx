import React, { useState, useEffect } from "react";

import { Principal } from "@dfinity/principal"; // Import Principal

import { useAuth } from "./use-auth-client";
import './index.css';

function Home() {
  //token names
  const [tokenName, setTokenName] = useState("");
 //const [icptokenName, setIcpTokenName] = useState("");
  const [TommytokenName, setTommyName] = useState("");
  //const [logoUrl, setLogoUrl] = useState("");
  //token balances
  const [balance, setBalance] = useState(0n); // Keep balance as BigInt
  const [Icpbalance, setIcpBalance] = useState(0n); // Keep balance as BigInt
  const [Tommybalance, setTommyBalance] = useState(0n); // Keep balance as BigInt
  const [accountTransactions, setAccountTransactions] = useState([]);
  const { whoamiActor, icrcIndexActor, icpActor, tommy_Actor, isAuthenticated } = useAuth();
  const { principal } = useAuth();



  // Fetch token details (name and logo) and user balance on load
  async function fetchData(principalId) {
    try {
      // Use principalId directly if it's a Principal object
      const owner = typeof principalId === 'string' ? Principal.fromText(principalId) : principalId;

      // Fetch token name
      const name = await whoamiActor.icrc1_name();
      setTokenName(name);

      // const icptokenname = await icpActor.icrc1_symbol();
      // setIcpTokenName(icptokenname);

      const tommyTokenname = await tommy_Actor.icrc1_name();
      setTommyName(tommyTokenname);

      // Fetch logo URL
      // const logo = await whoamiActor.icrc1_metadata();
      // setLogoUrl(logo);

      // Fetch user token balances
      const balanceResult = await whoamiActor.icrc1_balance_of({
        owner, // Use the Principal object directly
        subaccount: [],
      });
      const numericBalanceMercx = Number(balanceResult);
      const after_ap = numericBalanceMercx / 1e8;
      setBalance(after_ap);

      // Fetch icp balance
      // const balanceicp = await icpActor.icrc1_balance_of({
      //   owner, // Use the Principal object directly
      //   subaccount: [],
      // });
      // const numericBalanceIcp = Number(balanceicp);
      // const after_app = numericBalanceIcp / 1e8;
      // setIcpBalance(after_app);

      const balanceTommy = await tommy_Actor.icrc1_balance_of({
        owner, // Use the Principal object directly
        subaccount: [],
      });
      const numericBalanceTommy = Number(balanceTommy);
      const after_ap_tommy = numericBalanceTommy / 1e8;
      setTommyBalance(after_ap_tommy);

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
  }, [principal, balance, accountTransactions]);




  return (
 
    <div>
      {/* <MyNavbar /> */}
      <main className="min-h-screen bg-gray-900 text-white p-4">
        <section className="bg-slate-800 text-white rounded-lg shadow p-4 m-4">
          <h2 className="text-lg font-bold">Your Balance</h2>
          <p className="text-xl">{!isAuthenticated ? `0 ${tokenName}` : `${balance.toString()} ${tokenName}`}</p>
          <p className="text-xl">{!isAuthenticated ? `0 ${TommytokenName}` : `${Tommybalance.toString()} ${TommytokenName}`}</p>
          {/* <p className="text-xl"> {!isAuthenticated ? `0 ${icptokenName}` : `${Icpbalance.toString()} ${icptokenName}`}</p> */}

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
      </main>

    </div>

  );
}

export default () => (

  <Home />

);


