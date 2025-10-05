import React, { useState, useEffect } from "react";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "../use-auth-client";

export default function RemoveLiquidity() {
  const [searchParams] = useSearchParams();
  const token0 = searchParams.get("token0");
  const token1 = searchParams.get("token1");
  const [amount, setAmount] = useState("");
  const [userLpBalance, setUserLpBalance] = useState(null);
  const [isMaxAmount, setIsMaxAmount] = useState(false); // NEW FLAG
  const [reply, setReply] = useState(null);
  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");

  const { mercx_Actor, principal } = useAuth();

  // Fetch user's LP balance on mount
  useEffect(() => {
    const fetchLpBalance = async () => {
      if (!mercx_Actor || !principal) return;
      
      try {
        const lpTokens = await mercx_Actor.get_lp_tokens_by_principal(principal);
        
        // Find the matching LP token
        const normalize = (s) => (s || "").toUpperCase();
        const match = lpTokens.find(t => {
          const sym = normalize(t.symbol);
          return sym === `${normalize(token0)}_${normalize(token1)}` ||
                 sym === `${normalize(token1)}_${normalize(token0)}` ||
                 sym === `${normalize(token0)}_${normalize(token1)} LP` ||
                 sym === `${normalize(token1)}_${normalize(token0)} LP`;
        });

        if (match) {
          setUserLpBalance({
            raw: match.amount,
            display: Number(match.amount) / 10**8
          });
        }
      } catch (e) {
        console.error("Failed to fetch LP balance:", e);
      }
    };

    fetchLpBalance();
  }, [mercx_Actor, principal, token0, token1]);

  // Set max amount
  const handleMaxClick = () => {
    if (userLpBalance) {
      setAmount(userLpBalance.display.toString());
      setIsMaxAmount(true); // Mark that MAX was clicked
    }
  };

  const handleRemove = async () => {
    if (!amount || parseFloat(amount) <= 0) return;
    
    setLoading(true);
    setErrorMsg("");

    try {
      let scaledAmount;

      // If MAX was clicked, use exact raw amount
      if (isMaxAmount && userLpBalance) {
        scaledAmount = userLpBalance.raw;
        console.log("Using exact raw amount (MAX clicked):", scaledAmount);
      } else {
        // Otherwise, scale the input amount
        const DECIMALS = 8;
        scaledAmount = Math.floor(parseFloat(amount) * Math.pow(10, DECIMALS));
        console.log("Using scaled amount:", scaledAmount);
      }

      console.log("Scaled LP amount sent:", scaledAmount);

      const res = await mercx_Actor.remove_liquidity({
        token_0: token0,
        token_1: token1,
        remove_lp_token_amount: BigInt(scaledAmount),
      });
  
      console.log("Liquidity removed:", res);
      
      if (res && res.Ok) {
        setReply(res.Ok);
        setAmount("");
        setIsMaxAmount(false); // Reset flag
        setErrorMsg("");
      } else if (res && res.Err) {
        setReply(null);
        setErrorMsg(res.Err);
      } else {
        setReply(null);
        setErrorMsg("Unknown error occurred.");
      }
    } catch (e) {
      console.error("Failed to remove liquidity", e);
      setErrorMsg("Failed to remove liquidity. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  // Helper function to format BigInt values with decimals
  const formatTokenAmount = (amount, decimals = 8) => {
    if (!amount) return "0.0";
    
    const amountStr = amount.toString().padStart(decimals + 1, '0');
    const integerPart = amountStr.slice(0, -decimals) || '0';
    const fractionalPart = amountStr.slice(-decimals).replace(/\.?0+$/, '');
    
    return fractionalPart.length > 0 
      ? `${integerPart}.${fractionalPart}` 
      : integerPart;
  };

  // Function to shorten principal ID for display
  const shortenPrincipal = (principal) => {
    if (!principal) return "";
    if (principal.length <= 15) return principal;
    return `${principal.slice(0, 5)}...${principal.slice(-5)}`;
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-b bg-gray-900 px-4 py-12">
      <div className="w-full max-w-lg bg-gray-800 border border-gray-700 shadow-lg rounded-xl p-8">
        <h2 className="text-xl sm:text-2xl font-bold text-white text-center mb-6">
          Remove Liquidity <br />
          <span className="text-sm font-medium text-gray-400">
            {token0}/{token1}
          </span>
        </h2>

        {/* Display available balance */}
        {userLpBalance && (
          <div className="mb-2 text-sm text-gray-400 text-right">
            Available: {userLpBalance.display.toFixed(8)} LP tokens
          </div>
        )}

        {/* Input with MAX button */}
        <div className="relative mb-4">
          <input
            type="number"
            placeholder="Enter LP tokens to remove"
            value={amount}
            onChange={(e) => {
              setAmount(e.target.value);
              setIsMaxAmount(false); // Reset flag when user manually types
            }}
            className="w-full px-4 py-2 pr-16 rounded bg-gray-700 text-white border border-gray-600 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-700"
          />
          <button
            onClick={handleMaxClick}
            className="absolute right-2 top-1/2 -translate-y-1/2 px-3 py-1 text-xs font-bold text-indigo-400 hover:text-indigo-300 bg-indigo-900/30 hover:bg-indigo-900/50 rounded transition-colors"
          >
            MAX
          </button>
        </div>

        <button
          onClick={handleRemove}
          disabled={loading || !amount || parseFloat(amount) <= 0}
          className={`w-full py-2 px-4 font-bold rounded-lg text-sm flex items-center justify-center transition 
            ${loading || !amount || parseFloat(amount) <= 0 
              ? "bg-gray-500 cursor-not-allowed" 
              : "bg-gradient-to-r from-indigo-500 to-indigo-700 hover:from-indigo-700 hover:to-indigo-900"} 
            text-white`}
        >
          {loading ? (
            <div className="flex items-center justify-center">
              <svg className="animate-spin -ml-1 mr-2 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Processing...
            </div>
          ) : "Confirm Removal"}
        </button>

        
        <div className="bg-gray-750 px-6 py-4 border-t border-gray-700 mt-4 rounded-lg">
          <div className="flex items-start">
            <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 text-red-400 mr-2 mt-0.5" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
            </svg>
            <p className="text-sm text-gray-400">
              Removing liquidity will convert your LP tokens back into the underlying assets at the current pool ratio.
            </p>
          </div>
        </div>

        {errorMsg && (
          <div className="mt-4 p-4 bg-red-800 border border-red-600 text-white rounded-md">
            ⚠️ {errorMsg}
          </div>
        )}

        {reply && (
          <div className="mt-6 bg-gray-800 border border-gray-700 rounded-lg p-5 shadow-md">
            <h3 className="text-white text-xl font-semibold mb-4 flex items-center">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
              </svg>
              Remove Summary
            </h3>

            <div className="flex flex-col gap-4">
              <div className="bg-gray-900 rounded-md p-4">
                <p className="text-gray-400 text-sm mb-1">You will receive:</p>
                <p className="text-white text-lg font-bold">
                  {formatTokenAmount(reply.amount_0)}{" "}
                  <span className="text-red-400" title={reply.symbol_0}>
                    {shortenPrincipal(reply.symbol_0)}
                  </span>
                </p>
                <p className="text-sm text-gray-400">
                  LP Fee:{" "}
                  <span className="text-red-400 font-medium">
                    {formatTokenAmount(reply.lp_fee_0)}{" "}
                    <span title={reply.symbol_0}>
                      {shortenPrincipal(reply.symbol_0)}
                    </span>
                  </span>
                </p>
              </div>

              <div className="bg-gray-900 rounded-md p-4">
                <p className="text-gray-400 text-sm mb-1">You will receive:</p>
                <p className="text-white text-lg font-bold">
                  {formatTokenAmount(reply.amount_1)}{" "}
                  <span className="text-red-400" title={reply.symbol_1}>
                    {shortenPrincipal(reply.symbol_1)}
                  </span>
                </p>
                <p className="text-sm text-gray-400">
                  LP Fee:{" "}
                  <span className="text-red-400 font-medium">
                    {formatTokenAmount(reply.lp_fee_1)}{" "}
                    <span title={reply.symbol_1}>
                      {shortenPrincipal(reply.symbol_1)}
                    </span>
                  </span>
                </p>
              </div>
              
              <div className="bg-gray-900 rounded-md p-4">
                <p className="text-gray-400 text-sm mb-1">LP tokens removed:</p>
                <p className="text-white text-lg font-bold">
                  {formatTokenAmount(reply.remove_lp_token_amount)}
                </p>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}