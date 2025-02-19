import React, { useState, useEffect } from "react";

import { Principal } from "@dfinity/principal"; // Import Principal

import { useAuth } from "./use-auth-client";
import './index.css';

function Home() {
  //token names
  const [tokenName, setTokenName] = useState("");
 const [icptokenName, setIcpTokenName] = useState("");
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

      const icptokenname = await icpActor.icrc1_symbol();
      setIcpTokenName(icptokenname);

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
      const balanceicp = await icpActor.icrc1_balance_of({
        owner, // Use the Principal object directly
        subaccount: [],
      });
      const numericBalanceIcp = Number(balanceicp);
      const after_app = numericBalanceIcp / 1e8;
      setIcpBalance(after_app);

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
 
    <div className="relative overflow-hidden min-h-screen">
      {/* <MyNavbar /> */}

      <video className="absolute top-0 left-0 w-full h-full object-cover z-0" autoPlay loop muted playsInline poster="bg.png">
      <source src="/output.mp4" type="video/mp4" />


</video>
  
     

      <main className="relative z-10 min-h-screen text-white p-4">
      <section className="bg-slate-800 text-white rounded-lg shadow p-4 m-4 bg-opacity-20">

        {/* <section className="bg-slate-800 bg-opacity-80 text-white rounded-lg shadow p-4 m-4"> */}
         
    <div className="py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12">
        {/* <a href="#" className="inline-flex justify-between items-center py-1 px-1 pr-4 mb-7 text-sm text-gray-700 bg-gray-100 rounded-full dark:bg-gray-800 dark:text-white hover:bg-gray-200 dark:hover:bg-gray-700" role="alert">
            <span className="text-xs bg-primary-600 rounded-full text-white px-4 py-1.5 mr-3">New</span> <span className="text-sm font-medium">See what's new</span> 
            <svg className="ml-2 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clipRule="evenodd"></path></svg>
        </a> */}
        <h1 className="mb-4 mt-7 text-4xl font-extrabold tracking-tight leading-none md:text-5xl lg:text-6xl text-white">Bringing real-world finance to Web3</h1>
        <p className="text-lg font-normal  lg:text-xl sm:px-16 xl:px-48 text-gray-400">Earn yield from tokenized T-Bills & Money Market Funds (FXMX)</p>
        <p className=" text-lg font-normal  lg:text-xl sm:px-16 xl:px-48 text-gray-400">Gain exposure to Egypt’s top 30 stocks with EX30</p>
        <p className="mb-8 text-lg font-normal  lg:text-xl sm:px-16 xl:px-48 text-gray-400">	Backed by real assets, secured on ICP</p> 
        <div className="flex flex-col mb-8 lg:mb-16 space-y-4 sm:flex-row sm:justify-center sm:space-y-0 sm:space-x-4">
            {/* <a href="#" className="inline-flex justify-center items-center py-3 px-5 text-base font-medium text-center text-white rounded-lg bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-900">
            Sign up now for early access
                <svg className="ml-2 -mr-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
            </a> */}
            <a href="/signup" className="inline-flex justify-center items-center py-3 px-5 text-base font-medium text-center  rounded-lg border  focus:ring-4  text-white border-gray-700 hover:bg-gray-700 focus:ring-gray-800">
            Sign up now for early access
            <svg className="ml-2 -mr-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
            </a>  
        </div>
       
        </div> 
    {/* </div> */}

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

   
        {/* <section className="bg-slate-800 text-white rounded-lg shadow p-4 m-4 bg-opacity-20" >
          <h2 className="text-lg font-bold">Your Balance</h2>
          <p className="text-xl">{!isAuthenticated ? `0 ${tokenName}` : `${balance.toString()} ${tokenName}`}</p>
          <p className="text-xl">{!isAuthenticated ? `0 ${TommytokenName}` : `${Tommybalance.toString()} ${TommytokenName}`}</p>
          <p className="text-xl"> {!isAuthenticated ? `0 ${icptokenName}` : `${Icpbalance.toString()} ${icptokenName}`}</p>

        </section> */}

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



