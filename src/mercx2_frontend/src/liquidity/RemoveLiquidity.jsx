import React, { useState, useEffect } from "react";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "../use-auth-client";

export default function RemoveLiquidity() {
  const [searchParams] = useSearchParams();
  const token0 = searchParams.get("token0");
  const token1 = searchParams.get("token1");
  const [amount, setAmount] = useState("");
  const [userLpBalance, setUserLpBalance] = useState(null);
  const [isMaxAmount, setIsMaxAmount] = useState(false);
  const [reply, setReply] = useState(null);
  const [loading, setLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");

  const { mercx_Actor, principal } = useAuth();

  // Extract fetchLpBalance so it can be reused
  const fetchLpBalance = async () => {
    if (!mercx_Actor || !principal) return;
    
    try {
      const lpTokens = await mercx_Actor.get_lp_tokens_by_principal(principal);
      
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
      } else {
        setUserLpBalance(null);
      }
    } catch (e) {
      console.error("Failed to fetch LP balance:", e);
    }
  };

  // Fetch on mount
  useEffect(() => {
    fetchLpBalance();
  }, [mercx_Actor, principal, token0, token1]);

  const handleMaxClick = () => {
    if (userLpBalance) {
      setAmount(userLpBalance.display.toString());
      setIsMaxAmount(true);
    }
  };

  const handleRemove = async () => {
    if (!amount || parseFloat(amount) <= 0) return;
    
    setLoading(true);
    setErrorMsg("");

    try {
      let scaledAmount;

      if (isMaxAmount && userLpBalance) {
        scaledAmount = userLpBalance.raw;
        console.log("Using exact raw amount (MAX clicked):", scaledAmount);
      } else {
        const DECIMALS = 8;
        scaledAmount = Math.floor(parseFloat(amount) * Math.pow(10, DECIMALS));
        console.log("Using scaled amount:", scaledAmount);
      }

      const res = await mercx_Actor.remove_liquidity({
        token_0: token0,
        token_1: token1,
        remove_lp_token_amount: BigInt(scaledAmount),
      });
  
      if (res && res.Ok) {
        setReply(res.Ok);
        setAmount("");
        setIsMaxAmount(false);
        setErrorMsg("");
        
        // Refetch balance after successful removal
        await fetchLpBalance();
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

  const formatTokenAmount = (amount, decimals = 8) => {
    if (!amount) return "0.0";
    
    const amountStr = amount.toString().padStart(decimals + 1, '0');
    const integerPart = amountStr.slice(0, -decimals) || '0';
    const fractionalPart = amountStr.slice(-decimals).replace(/\.?0+$/, '');
    
    return fractionalPart.length > 0 
      ? `${integerPart}.${fractionalPart}` 
      : integerPart;
  };

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

        {/* Display available balance or no tokens message */}
        {userLpBalance && userLpBalance.display > 0 ? (
          <div className="mb-2 text-sm text-gray-400 text-right">
            Available: {userLpBalance.display.toFixed(8)} LP tokens
          </div>
        ) : (
          <div className="mb-2 text-sm text-yellow-400 text-center bg-yellow-900/20 p-2 rounded">
            No LP tokens available for this pool
          </div>
        )}

        <div className="relative mb-4">
          <input
            type="number"
            placeholder={userLpBalance && userLpBalance.display > 0 ? "Enter LP tokens to remove" : "No LP tokens available"}
            value={amount}
            onChange={(e) => {
              setAmount(e.target.value);
              setIsMaxAmount(false);
            }}
            disabled={!userLpBalance || userLpBalance.display === 0}
            className="w-full px-4 py-2 pr-16 rounded bg-gray-700 text-white border border-gray-600 placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          />
          <button
            onClick={handleMaxClick}
            disabled={!userLpBalance || userLpBalance.display === 0}
            className="absolute right-2 top-1/2 -translate-y-1/2 px-3 py-1 text-xs font-bold text-indigo-400 hover:text-indigo-300 bg-indigo-900/30 hover:bg-indigo-900/50 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            MAX
          </button>
        </div>

        <button
          onClick={handleRemove}
          disabled={loading || !amount || parseFloat(amount) <= 0 || !userLpBalance || userLpBalance.display === 0}
          className={`w-full py-2 px-4 font-bold rounded-lg text-sm flex items-center justify-center transition 
            ${loading || !amount || parseFloat(amount) <= 0 || !userLpBalance || userLpBalance.display === 0
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
              <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2 text-green-400" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
              </svg>
              Removal Successful
            </h3>

            <div className="flex flex-col gap-4">
              <div className="bg-gray-900 rounded-md p-4">
                <p className="text-gray-400 text-sm mb-1">You received:</p>
                <p className="text-white text-lg font-bold">
                  {formatTokenAmount(reply.amount_0)} {shortenPrincipal(reply.symbol_0)}
                </p>
                <p className="text-sm text-gray-400">
                  LP Fee: {formatTokenAmount(reply.lp_fee_0)} {shortenPrincipal(reply.symbol_0)}
                </p>
              </div>

              <div className="bg-gray-900 rounded-md p-4">
                <p className="text-gray-400 text-sm mb-1">You received:</p>
                <p className="text-white text-lg font-bold">
                  {formatTokenAmount(reply.amount_1)} {shortenPrincipal(reply.symbol_1)}
                </p>
                <p className="text-sm text-gray-400">
                  LP Fee: {formatTokenAmount(reply.lp_fee_1)} {shortenPrincipal(reply.symbol_1)}
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