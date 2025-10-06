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
    //  setTokenName(name);

      const icptokenname = await icpActor.icrc1_symbol();
     // setIcpTokenName(icptokenname);

      const tommyTokenname = await tommy_Actor.icrc1_name();
    //  setTommyName(tommyTokenname);

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
            <a href="/signup" className="inline-flex justify-center items-center py-3 px-5 text-base font-medium text-center  rounded-lg border focus:ring-4  text-white bg-gradient-to-r from-blue-600 to-purple-600 hover:shadow-xl hover:scale-105 hover:border-white/20 focus:ring-gray-800">
            Sign up now for early access
            <svg className="ml-2 -mr-1 w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z" clipRule="evenodd"></path></svg>
            </a>  
        </div>
       
        </div> 
    {/* </div> */}

    

        </section>

        {/* Features Section */}
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

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
              {features.map((feature, index) => (
                <div 
                  key={index} 
                  className="bg-slate-800 bg-opacity-40 backdrop-blur-sm border border-slate-600 border-opacity-50 rounded-none p-6 hover:bg-opacity-60 transition-all duration-200 flex flex-col items-center text-center"
                >
                  <div className="text-4xl mb-4">{feature.icon}</div>
                  <h3 className="text-xl font-semibold mb-3 text-white">{feature.title}</h3>
                  <p className="text-gray-300 text-sm leading-relaxed">{feature.description}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

  {/* Platform Overview Section */}
  <section className="py-16 px-4">
          <div className="max-w-4xl mx-auto text-center">
            <div className="bg-slate-800 bg-opacity-40 backdrop-blur-sm border border-slate-600 border-opacity-50 rounded-2xl p-8">
              <div className="flex items-center justify-center mb-4">
                <div className="flex items-center space-x-2">
                  <div className="w-3 h-3 bg-blue-400 rounded-full animate-pulse"></div>
                  <span className="text-blue-400 font-semibold">Decentralized Tokenization Platform</span>
                </div>
              </div>
              <h3 className="text-2xl font-bold text-white mb-4">Digital Infrastructure for MENA Capital Markets</h3>
              <p className="text-gray-300 mb-6">
                MercX is a decentralized tokenization platform built on Internet Computer Protocol (ICP), 
                designed to bring Real World Assets (RWA) to the blockchain. Starting with tokenized exposure 
                to Egypt's top 30 companies via the EGX30 index.
              </p>
              <div className="grid md:grid-cols-2 gap-6 text-left">
                <div className="space-y-3">
                  <h4 className="text-lg font-semibold text-blue-300">Platform Capabilities:</h4>
                  <ul className="space-y-2 text-gray-300">
                    <li>• Real-time Price Feeds</li>
                    <li>• Built-in Liquidity Pools</li>
                    <li>• Swap Functions</li>
                    <li>• Business Analytics Dashboards</li>
                  </ul>
                </div>
                <div className="space-y-3">
                  <h4 className="text-lg font-semibold text-cyan-300">Business Impact:</h4>
                  <ul className="space-y-2 text-gray-300">
                    <li>• Faster transactions</li>
                    <li>• Wider investor access</li>
                    <li>• Borrowing and Lending Systems</li>
                    <li>• Cutting-edge fund management</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </section>


        {/* Footer */}
        <footer className="bg-slate-900 bg-opacity-60 backdrop-blur-sm border-t border-slate-600 border-opacity-50 py-12 px-4">
          <div className="max-w-6xl mx-auto">
            <div className="grid md:grid-cols-4 gap-8 mb-8">
              
              {/* Company Info */}
              <div className="space-y-4">
                <div className="flex items-center space-x-2">
                  {/* <div className="w-10 h-10 bg-gradient-to-r from-blue-500 to-cyan-400 rounded-lg flex items-center justify-center text-xl font-bold">
                    M
                  </div> */}
                  <h3 className="text-xl font-bold text-white">MercX</h3>
                </div>
                <p className="text-gray-400 text-sm">
                  Bringing real-world assets to Web3. Tokenizing Egypt's financial markets on the Internet Computer.
                </p>
              </div>

              {/* Products */}
              <div>
                <h4 className="text-white font-semibold mb-4">Products</h4>
                <ul className="space-y-2 text-gray-400 text-sm">
               
                  <li><a href="/trade" className="hover:text-blue-400 transition-colors">Swap</a></li>
                  <li><a href="/pools" className="hover:text-blue-400 transition-colors">Liquidity Pools</a></li>
                </ul>
              </div>

              {/* Company */}
              <div>
                <h4 className="text-white font-semibold mb-4">Company</h4>
                <ul className="space-y-2 text-gray-400 text-sm">
                  <li><a href="/about" className="hover:text-blue-400 transition-colors">About Us</a></li>
                  {/* <li><a href="/docs" className="hover:text-blue-400 transition-colors">Documentation</a></li>
                  <li><a href="/compliance" className="hover:text-blue-400 transition-colors">Compliance</a></li> */}
                  <li><a href="/contact" className="hover:text-blue-400 transition-colors">Contact</a></li>
                </ul>
              </div>

              {/* Resources */}
              <div>
                <h4 className="text-white font-semibold mb-4">Resources</h4>
                <ul className="space-y-2 text-gray-400 text-sm">
                  <li><a href="/whitepaper" className="hover:text-blue-400 transition-colors">Whitepaper</a></li>
                  <li><a href="/faq" className="hover:text-blue-400 transition-colors">FAQ</a></li>
                  <li><a href="/blog" className="hover:text-blue-400 transition-colors">Blog</a></li>
                  <li><a href="/support" className="hover:text-blue-400 transition-colors">Support</a></li>
                </ul>
              </div>
            </div>

            {/* Social Links */}
            <div className="flex justify-center space-x-6 mb-8 pb-8 border-b border-slate-600 border-opacity-50">
            <a href="https://x.com/MercX__" className="text-gray-400 hover:text-blue-400 transition-colors">
                <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"></path>
                </svg>
              </a>
              {/* <a href="https://discord.gg/mercx" className="text-gray-400 hover:text-blue-400 transition-colors">
                <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M20.317 4.37a19.791 19.791 0 00-4.885-1.515.074.074 0 00-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 00-5.487 0 12.64 12.64 0 00-.617-1.25.077.077 0 00-.079-.037A19.736 19.736 0 003.677 4.37a.07.07 0 00-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 00.031.057 19.9 19.9 0 005.993 3.03.078.078 0 00.084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 00-.041-.106 13.107 13.107 0 01-1.872-.892.077.077 0 01-.008-.128 10.2 10.2 0 00.372-.292.074.074 0 01.077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 01.078.01c.12.098.246.198.373.292a.077.077 0 01-.006.127 12.299 12.299 0 01-1.873.892.077.077 0 00-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 00.084.028 19.839 19.839 0 006.002-3.03.077.077 0 00.032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 00-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"></path>
                </svg>
              </a> */}
              <a href="https://t.me/mercx" className="text-gray-400 hover:text-blue-400 transition-colors">
                <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z"></path>
                </svg>
              </a>
              <a href="https://github.com/Mercatura-Developers-Team/Mercx.git" className="text-gray-400 hover:text-blue-400 transition-colors">
                <svg className="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"></path>
                </svg>
              </a>
            </div>

            {/* Bottom Section */}
            <div className="flex flex-col md:flex-row justify-between items-center text-gray-400 text-sm">
              <p>&copy; 2025 MercX. All rights reserved.</p>
              <div className="flex space-x-6 mt-4 md:mt-0">
                {/* <a href="/privacy" className="hover:text-blue-400 transition-colors">Privacy Policy</a>
                <a href="/terms" className="hover:text-blue-400 transition-colors">Terms of Service</a> */}
                {/* <a href="/cookies" className="hover:text-blue-400 transition-colors">Cookie Policy</a> */}
              </div>
            </div>

            {/* Regulatory Notice
            <div className="mt-8 pt-8 border-t border-slate-600 border-opacity-50">
              <p className="text-gray-500 text-xs text-center">
                MercX operates in compliance with the Egyptian Financial Regulatory Authority (FRA). 
                All tokenized assets are backed by real-world reserves. Cryptocurrency investments carry risk. 
                Please invest responsibly.
              </p>
            </div> */}
          </div>
        </footer>
      </main>
    </div>

  );
}

export default () => (

  <Home />

);



