import React, { useState, useEffect } from "react";

import { Principal } from "@dfinity/principal"; // Import Principal

import { useAuth } from "./use-auth-client";
import './index.css';
import Particles from './HomeBg';


function Home() {

  //const [logoUrl, setLogoUrl] = useState("");
  //token balances
  const [balance, setBalance] = useState(0n); // Keep balance as BigInt
  const [Icpbalance, setIcpBalance] = useState(0n); // Keep balance as BigInt
  const [Tommybalance, setTommyBalance] = useState(0n); // Keep balance as BigInt
  const [accountTransactions, setAccountTransactions] = useState([]);
  const { whoamiActor, icrcIndexActor, icpActor, tommy_Actor, isAuthenticated } = useAuth();
  const { principal } = useAuth();

  const features = [
    {
      icon: "⚡",
      title: "Real-Time Settlement",
      description: "Transactions on MercX are settled instantaneously, allowing seamless asset management without traditional banking delays"
    },
    {
      icon: "🌐",
      title: "Enhanced Accessibility", 
      description: "Fractional ownership makes institutional-grade products accessible to retail investors starting from $100"
    },
    {
      icon: "🛡️",
      title: "Regulatory Compliance",
      description: "MercX adheres to regulations set by the Financial Regulatory Authority of Egypt, ensuring secure and transparent environment"
    },
    {
      icon: "🤖",
      title: "AI Agent Advisors",
      description: "Smart advisory tools and business analytics dashboards to assist users in understanding investment options and market conditions"
    }
  ];


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

      <div className="fixed inset-0 w-full h-full z-0 bg-black" >
      <div style={{ width: '100%', height: '600px', position: 'relative' }}>
  <Particles
        particleColors={['#3B82F6', '#06B6D4', '#8B5CF6']} // Blue/cyan colors instead of white

    particleCount={800}
    particleSpread={10}
    speed={0.1}
    particleBaseSize={100}
    moveParticlesOnHover={true}
    alphaParticles={false}
    disableRotation={false}
  />
</div>


</div>



      <main className="relative z-10 min-h-screen text-white p-4">
      <section className="bg-slate-800 text-white rounded-lg shadow p-4 m-4 bg-opacity-20">

        {/* <section className="bg-slate-800 bg-opacity-80 text-white rounded-lg shadow p-4 m-4"> */}
         
    <div className="py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12">
        {/* <a href="#" className="inline-flex justify-between items-center py-1 px-1 pr-4 mb-7 text-sm text-gray-700 bg-gray-100 rounded-full dark:bg-gray-800 dark:text-white hover:bg-gray-200 dark:hover:bg-gray-700" role="alert">
            <span className="text-xs bg-primary-600 rounded-full text-white px-4 py-1.5 mr-3">New</span> <span className="text-sm font-medium">See what's new</span> 
            <svg className="ml-2 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clipRule="evenodd"></path></svg>
        </a> */}
     {/* Main Heading */}
     <h1 className="mb-8 text-4xl md:text-6xl lg:text-7xl font-extrabold tracking-tight leading-none">
              <span className="bg-gradient-to-r from-blue-400 via-purple-500 to-green-400 bg-clip-text text-transparent">
                Bringing Real-World
              </span>
              <br />
              <span className="text-white">Finance to Web3</span>
            </h1>
                    <p className="text-lg font-normal  lg:text-xl sm:px-16 xl:px-48 text-gray-400">Earn yield from tokenized T-Bills & Money Market Funds (FXMX)</p>
        <p className=" text-lg font-normal  lg:text-xl sm:px-16 xl:px-48 text-gray-400">Gain exposure to Egypt’s top 30 stocks with EX30</p>
        <p className="mb-8 text-lg font-normal  lg:text-xl sm:px-16 xl:px-48 text-gray-400">	Backed by real assets, secured on ICP</p> 
        <div className="flex flex-col mb-8 lg:mb-16 space-y-4 sm:flex-row sm:justify-center sm:space-y-0 sm:space-x-4">
            {/* <a href="#" className="inline-flex justify-center items-center py-3 px-5 text-base font-medium text-center text-white rounded-lg bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-900">
            Sign up now for early access
                <svg className="ml-2 -mr-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
            </a> */}
            <a href="/signup" className="inline-flex justify-center items-center py-3 px-5 text-base font-medium text-center  rounded-lg border  focus:ring-4  text-white bg-gradient-to-r from-blue-600 to-purple-600 hover:shadow-xl hover:scale-105 hover:border-white/20 focus:ring-gray-800">
            Sign up now for early access
            <svg className="ml-2 -mr-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
            </a>  
        </div>
       
        </div> 
    {/* </div> */}

    

        </section>

        {/* Features Section */}
        <section className="py-20 px-4">
          <div className="max-w-6xl mx-auto">
            <div className="text-center mb-16">
              <h2 className="text-3xl md:text-4xl font-bold mb-4 text-white">
                Key Features
              </h2>
              <p className="text-xl text-gray-300 max-w-3xl mx-auto">
                Revolutionary infrastructure bringing MENA capital markets into the digital age
              </p>
            </div>

            <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
              {features.map((feature, index) => (
                <div 
                  key={index} 
                  className="group backdrop-blur-md bg-white/10 border border-white/20 rounded-xl p-6 hover:bg-white/15 hover:scale-105 transition-all duration-200"
                >
                  <div className="text-4xl mb-4 group-hover:scale-110 transition-transform">{feature.icon}</div>
                  <h3 className="text-xl font-semibold mb-2 text-white">{feature.title}</h3>
                  <p className="text-gray-300">{feature.description}</p>
                </div>
              ))}
            </div>
          </div>
        </section>


      </main>

    </div>

  );
}

export default () => (

  <Home />

);



