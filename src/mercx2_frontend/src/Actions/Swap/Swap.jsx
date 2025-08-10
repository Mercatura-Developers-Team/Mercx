import React, { useEffect } from 'react';
import { useState } from 'react';
import { useAuth } from '../../use-auth-client';
import { Principal } from "@dfinity/principal";
import TokenSelector from '../../pool/TokenSelector';
import SuccessModal from './SuccessModel';

const Swap = () => {

  const { mercx_Actor, isAuthenticated, principal, createTokenActor } = useAuth();
  // Token state
  const [tokens, setTokens] = useState([]);
  const [fromToken, setFromToken] = useState(null);
  const [toToken, setToToken] = useState(null);
  const [openTokenSelect, setOpenTokenSelect] = useState(false);
  const [selectingFor, setSelectingFor] = useState(null);
  const [logos, setLogos] = useState({});

  // Swap state
  const [fromAmount, setFromAmount] = useState('');
  const [toAmount, setToAmount] = useState('');
  const [rate, setRate] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isSwapping, setIsSwapping] = useState(false);
  const [error, setError] = useState('');
  const [successModal, setSuccessModal] = useState(false);

  // Balances
  const [fromBalance, setFromBalance] = useState(0n);
  const [toBalance, setToBalance] = useState(0n);

  // Function to fetch token logos
  const fetchTokenLogos = async (tokenList) => {
    const logoMap = {};

    for (const token of tokenList) {
      if (!logoMap[token.symbol]) {
        try {
          const principal = Principal.fromText(token.canister_id.toText());
          const url = await mercx_Actor.get_logo_url(principal);
          logoMap[token.symbol] = url;
        } catch (e) {
          console.warn(`Failed to fetch logo for ${token.symbol}`, e);
          logoMap[token.symbol] = "/j.png"; // Default logo
        }
      }
    }
    return logoMap;
  };

  // Fetch tokens with logos
  useEffect(() => {
    const fetchTokens = async () => {
      if (!mercx_Actor) return;
      try {
        const tokenList = await mercx_Actor.get_all_tokens();
        setTokens(tokenList);

        // Fetch logos for all tokens
        const logoMap = await fetchTokenLogos(tokenList);
        setLogos(logoMap);
        // // Set defaults if available
        // const icp = tokenList.find(t => t.symbol === "ICP");
        // const bella = tokenList.find(t => t.symbol === "BELLA");
        // if (icp) setFromToken(icp);
        // if (bella) setToToken(bella);
      } catch (err) {
        console.error("Failed to fetch tokens:", err);
      }
    };
    fetchTokens();
  }, [mercx_Actor]);

  // Fetch balances when tokens change
  useEffect(() => {
    const fetchBalances = async () => {
      if (!principal || !fromToken || !toToken) return;

      try {
        const fromActor = await createTokenActor(fromToken.canister_id.toText());
        const fromBal = await fromActor.icrc1_balance_of({ owner: principal, subaccount: [] });
        setFromBalance(fromBal);

        const toActor = await createTokenActor(toToken.canister_id.toText());
        const toBal = await toActor.icrc1_balance_of({ owner: principal, subaccount: [] });
        setToBalance(toBal);
      } catch (err) {
        console.error("Error fetching balances:", err);
      }
    };
    fetchBalances();
  }, [principal, fromToken, toToken, createTokenActor]);

  // Handle token selection
  const handleTokenSelect = (token) => {
    setError(''); // ✅ Clear error
    if (selectingFor === 'from') {
      if (toToken && token.canister_id.toText() === toToken.canister_id.toText()) {
        setError("Cannot select the same token");
        return;
      }
      setFromToken(token);
    } else {
      if (fromToken && token.canister_id.toText() === fromToken.canister_id.toText()) {
        setError("Cannot select the same token");
        return;
      }
      setToToken(token);
    }
    setOpenTokenSelect(false);
    setFromAmount('');
    setToAmount('');
  };

  // Calculate swap rate
  const calculateRate = async (amount) => {
    const numericAmount = parseFloat(amount);
    if (!fromToken || !toToken || isNaN(numericAmount) || numericAmount <= 0) return;

    setIsLoading(true);
    setError('');

    try {
      const amountIn = parseAmount(amount, fromToken.decimals);
      const rateResponse = await mercx_Actor.swap_amounts(
        fromToken.canister_id.toText(),  // ✅ convert to string
        amountIn,
        toToken.canister_id.toText()     // ✅ convert to string
      );


      if ("Err" in rateResponse) throw new Error(rateResponse.Err);

      const { receive_amount, price } = rateResponse.Ok;
      setRate(normalizeAmount(price, toToken.decimals));
      setToAmount(normalizeAmount(receive_amount, toToken.decimals));

    } catch (err) {
      console.error("Rate calculation failed:", err);
      setError(err.message || "Failed to calculate rate");
      setToAmount('');
    } finally {
      setIsLoading(false);
    }
  };

  // Handle amount input
  const handleFromAmountChange = (e) => {
    const value = e.target.value;
    setFromAmount(value);
    setError(''); // ✅ Clear error
    if (value === '') {
      setToAmount('');
      return;
    }
    calculateRate(value);
  };

  // Execute swap
  const handleSwap = async () => {
    if (!fromToken || !toToken || !fromAmount || !toAmount) return;

    setIsSwapping(true);
    setError('');

    try {
      const spenderId = "ahw5u-keaaa-aaaaa-qaaha-cai"; // Swap canister ID
      const amountIn = parseAmount(fromAmount, fromToken.decimals);
      const amountOut = parseAmount(toAmount, toToken.decimals);

      // Check and approve allowance
      const fromActor = await createTokenActor(fromToken.canister_id.toText());
      const allowanceCheck = {
        account: { owner: principal, subaccount: [] },
        spender: { owner: Principal.fromText(spenderId), subaccount: [] }
      };

      const currentAllowance = (await fromActor.icrc2_allowance(allowanceCheck)).allowance;

      if (currentAllowance < amountIn) {
        const approveResult = await fromActor.icrc2_approve({
          spender: { owner: Principal.fromText(spenderId), subaccount: [] },
          amount: amountIn,
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          expires_at: [],
          expected_allowance: [],
        });

        if ("Err" in approveResult) {
          throw new Error(`Approval failed: ${JSON.stringify(approveResult.Err)}`);
        }
      }

      // Execute swap
      const swapResult = await mercx_Actor.swap_tokens({
        pay_token: fromToken.canister_id.toText(),
        receive_token: toToken.canister_id.toText(),
        pay_amount: amountIn,
        pay_tx_id: [], // Or provide TransactionHash/BlockIndex variant
        receive_amount: [], // Optional: can be left empty if not protecting with limit
        receive_address: [], // Optional
        max_slippage: [], // ✅ Add this! 1% slippage (wrap in [])
      });
      console.log(swapResult);

      if ("Err" in swapResult) throw new Error(swapResult.Err);

      // Success
      setSuccessModal(true);
      setFromAmount('');
      setToAmount('');
    } catch (err) {
      console.error("Swap failed:", err);
      setError(err.message || "Swap failed. Please try again.");
    } finally {
      setIsSwapping(false);
    }
  };

  // Helper to format balance for display
  const formatBalance = (balance, decimals) => {
    if (!balance) return "0";
    const normalized = normalizeAmount(balance, decimals);
    return parseFloat(normalized).toFixed(4);
  };

  return (
    <div className="shadow-xl rounded-3xl h-[480px] border-t-[1px] border-slate-800 bg-slate-800">
      <div className=" border-gray-900 shadow-md p-3">

        {/* Header */}
        <p className="text-gray-300 text-center text-sm">
          Swap Tokens
        </p>

        {/* Main Content */}
        <div className="p-5">
          {/* From Token */}
          <div className="mb-4 bg-gradient-to-br from-gray-800 to-gray-900 rounded-xl p-4  border border-gray-700 hover:border-indigo-500/50 transition-all duration-300">
             
            <div className="flex justify-between items-center mb-3">
              <button
                onClick={() => { setSelectingFor('from'); setOpenTokenSelect(true); }}
                className="flex items-center gap-2 shadow-md rounded-md hover:bg-gray-500 px-3 py-1 transition"
              >
                {fromToken ? (
                  <>
                  
                    <img
                      src={logos[fromToken.symbol] || "./favicon.ico"}
                      alt={fromToken.symbol}
                      className="w-8 h-8 rounded-full shadow-md border-2 border-indigo-400/50"
                    />
                    <span className="font-medium text-white">{fromToken.symbol}</span>
                 
                  </>
                ) : (
                  <span className="text-white  ">Select Token</span>
                )}
                <svg className="w-4 h-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                </svg>
              </button>
              <div className="flex items-center gap-1">
                <span className="text-sm text-gray-400">Balance:</span>
                {fromToken && (
                  <img
                    src={logos[fromToken.symbol] || "./favicon.ico"}
                    alt={fromToken.symbol}
                    className="w-4 h-4 rounded-full"
                  />
                )}
                <span className="text-sm text-gray-400">
                  {fromToken ? formatBalance(fromBalance, fromToken.decimals) : '0'}
                </span>
              </div>
            </div>
            <div className="flex items-center">
              <input
                type="number"
                value={fromAmount}
                onChange={handleFromAmountChange}
                placeholder="0.0"
                className="w-full bg-transparent text-white text-2xl outline-none placeholder-gray-500 font-medium"
                disabled={!fromToken || !toToken}
              />
              {fromToken && (
                <button
                  onClick={() => {
                    const v = formatBalance(fromBalance, fromToken.decimals);
                    setFromAmount(v);     // ← already sets the input
                    calculateRate(v);     // ← add this line ─ recompute the other field
                  }}
                  className="text-xs bg-indigo-600 hover:bg-indigo-700 text-white px-2 py-1 rounded"
                >
                  MAX
                </button>
              )}
            </div>
          </div>

          {/* Swap Direction Arrow */}
          <div className="flex justify-center my-3">
            <button
              onClick={() => {
                const temp = fromToken;
                setFromToken(toToken);
                setToToken(temp);
                setFromAmount('');
                setToAmount('');
              }}
              className="p-3 bg-gray-700 hover:bg-indigo-700 rounded-full text-gray-300 transition-all duration-300 hover:-translate-y-0.5 shadow-md"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" />
              </svg>
            </button>
          </div>

          {/* To Token */}
          <div className="mb-6 bg-gradient-to-br from-gray-800 to-gray-900 rounded-xl p-4 border border-gray-700 hover:border-indigo-500/50 transition-all duration-300">
            <div className="flex justify-between items-center mb-3">
              <button
                onClick={() => { setSelectingFor('to'); setOpenTokenSelect(true); }}
                className="flex items-center gap-2 shadow-md rounded-md hover:bg-gray-500  px-3 py-1 transition"
              >
                {toToken ? (
                  <>
                    <img
                      src={logos[toToken.symbol] || "./Bella.jpeg"}
                      alt={toToken.symbol}
                      className="w-5 h-5 rounded-full"
                    />
                    <span className="font-medium text-white">{toToken.symbol}</span>
                  </>
                ) : (
                  <span className="text-white">Select Token</span>
                )}
                <svg className="w-4 h-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                </svg>
              </button>
              <div className="flex items-center gap-1">
                <span className="text-sm text-gray-400">Balance:</span>
                {toToken && (
                  <img
                    src={logos[toToken.symbol] || "./Bella.jpeg"}
                    alt={toToken.symbol}
                    className="w-4 h-4 rounded-full"
                  />
                )}
                <span className="text-sm text-gray-400">
                  {toToken ? formatBalance(toBalance, toToken.decimals) : '0'}
                </span>
              </div>
            </div>
            <div className="flex items-center">
              {toToken && (
                <img
                  src={logos[toToken.symbol] || "./Bella.jpeg"}
                  alt={toToken.symbol}
                  className="w-6 h-6 rounded-full mr-2"
                />
              )}
              <span className="text-white text-2xl">
                {isLoading ? (
                  <svg className="animate-spin h-6 w-6 text-gray-400" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.86 1.861 7.298 4.708 9.291l1.292-1.292z"></path>
                  </svg>
                ) : (
                  toAmount || '0.0'
                )}
              </span>
            </div>
          </div>

          {/* Rate Info */}
          {rate && fromAmount && (
            <div className="flex items-center justify-center gap-2 text-sm text-gray-400 mb-4">
              {fromToken && (
                <img
                  src={logos[fromToken.symbol] || "./favicon.ico"}
                  alt={fromToken.symbol}
                  className="w-4 h-4 rounded-full"
                />
              )}
              <span>1 {fromToken?.symbol} = {formatRate(rate)} {toToken?.symbol}</span>
              {toToken && (
                <img
                  src={logos[toToken.symbol] || "./Bella.jpeg"}
                  alt={toToken.symbol}
                  className="w-4 h-4 rounded-full"
                />
              )}
            </div>
          )}

          {/* Error Message */}
          {error && (
            <div className="mb-4 p-2 bg-red-900/30 border border-red-700 text-red-400 text-sm rounded text-center">
              {error}
            </div>
          )}

          {/* Swap Button */}
          {/* <button
            onClick={handleSwap}
            disabled={!isAuthenticated || !fromAmount || isLoading || isSwapping || !fromToken || !toToken}
            className={`w-full py-3 rounded-lg font-bold text-white transition-colors ${isSwapping ? 'bg-indigo-400 cursor-not-allowed' :
                !isAuthenticated ? 'bg-gray-600 cursor-not-allowed' :
                  !fromAmount || !fromToken || !toToken ? 'bg-gray-600 cursor-not-allowed' :
                    'bg-indigo-600 hover:bg-indigo-700'
              }`}
          > */}
          <div className="flex justify-center">
            <button
              onClick={handleSwap}
              disabled={!isAuthenticated || !fromAmount || isLoading || isSwapping || !fromToken || !toToken}
              className="place-content-center py-2 px-6 text-sm font-bold text-white bg-indigo-600 rounded-md bg-opacity-85 hover:bg-opacity-90 disabled:bg-indigo-400 disabled:cursor-not-allowed transition"
            >


              {!isAuthenticated ? 'Connect Wallet' :
                isSwapping ? 'Swapping...' :
                  !fromToken || !toToken ? 'Select Tokens' :
                    !fromAmount ? 'Enter Amount' :
                      'Swap'}
            </button>
          </div>
          {/* Network Info */}
          <div className="flex items-center justify-center gap-2 mt-4 text-xs text-gray-500">
            {/* <img 
              src={logos['ICP'] || "./favicon.ico"} 
              alt="ICP" 
              className="w-3 h-3 rounded-full" 
            /> */}
            {/* <span>Estimated network fee: ~0.0002 ICP</span> */}
          </div>
        </div>

        {/* Token Selector Modal */}
        {openTokenSelect && (
          <TokenSelector
            tokens={tokens}
            selectingFor={selectingFor}
            token0={fromToken}
            token1={toToken}
            onSelect={handleTokenSelect}
            onCancel={() => setOpenTokenSelect(false)}
          />
        )}

        {/* Success Modal */}
        <SuccessModal
          isVisible={successModal}
          onClose={() => setSuccessModal(false)}
          action="swap"
        />
      </div>
    </div>
  );
};

// Helper functions
function parseAmount(amount, decimals) {
  if (!amount || isNaN(amount)) return 0n;
  const [whole, fraction = ''] = amount.split('.');
  const paddedFraction = fraction.padEnd(decimals, '0').slice(0, decimals);
  return BigInt(whole + paddedFraction);
}

function normalizeAmount(amount, decimals) {
  if (!amount) return "0";
  const amountStr = amount.toString().padStart(decimals + 1, '0');
  const whole = amountStr.slice(0, -decimals) || '0';
  const fraction = amountStr.slice(-decimals).replace(/0+$/, '');

  let result = fraction ? `${whole}.${fraction}` : whole;

  // Fix any case like "3..1234"
  result = result.replace(/\.\./g, '.');

  return result;
}

// Add this new helper function for formatting rates
function formatRate(rateValue, decimals = 6) {
  if (!rateValue) return "0";
  const num = parseFloat(rateValue.toString());
  return num.toFixed(decimals).replace(/\.?0+$/, ''); // Remove trailing zeros
}


export default Swap;