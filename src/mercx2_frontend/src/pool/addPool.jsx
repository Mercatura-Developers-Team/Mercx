import React, { useState, useEffect } from "react";
import { useAuth } from "../use-auth-client";

export default function CreatePool() {
  const { whoamiActor, icpActor, mercx_Actor, tommy_Actor, fxmxActor, kycActor, isAuthenticated, ckUSDTActor } = useAuth();
  const [token0, setToken0] = useState(null);
  const [token1, setToken1] = useState(null);
  // const tokens = [
  //     { name: "ICP", actor: icpActor },
  //     { name: "MERCX", actor: whoamiActor },
  //     { name: "TOMMY", actor: tommy_Actor },
  //     { name: "FXMX", actor: fxmxActor },
  //     { name: "ckUSDT", actor: ckUSDTActor },
  //   ];      
  const [tokens, setTokens] = useState([]);
  const [initialPrice, setInitialPrice] = useState("");
  const [amountToken0, setAmountToken0] = useState("");
  const [amountToken1, setAmountToken1] = useState("");

  const [openTokenSelect, setOpenTokenSelect] = useState(false);
  const [selectingFor, setSelectingFor] = useState(null); // "token0" or "token1"

  useEffect(() => {
    if (initialPrice && amountToken0) {
      const calculatedAmount1 = parseFloat(amountToken0) * parseFloat(initialPrice);
      setAmountToken1(calculatedAmount1 ? calculatedAmount1.toString() : "");
    } else {
      setAmountToken1("");
    }
  }, [initialPrice, amountToken0]);

  useEffect(() => {
    const fetchTokens = async () => {
      if (!mercx_Actor) return; // 🛑 Exit early if actor not ready
      try {
        const tokenList = await mercx_Actor.get_all_tokens();
        setTokens(tokenList);
      } catch (err) {
        console.error("Failed to fetch tokens:", err);
      }
    };

    fetchTokens();
  }, [mercx_Actor]);

  const handleCreatePool = async () => {
    const args = {
      token_0: token0.canister_id.toText(),
      token_1: token1.canister_id.toText(),
      // initial_price: Number(initialPrice),
      amount_0: Number(amountToken0),
      amount_1: Number(amountToken1),
      lp_fee_bps: [], // ← This is how you pass `None` in Candid for Option<u8>
    };
    console.log("Sending args:", args);

    try {
      const result = await mercx_Actor.add_pool(args);
      console.log("Result:", result);
    } catch (err) {
      console.error("Error adding pool:", err);
      alert("❌ Failed to add pool: " + err);
    }

  };

  const handleTokenSelect = (token) => {
    if (
      (selectingFor === "token0" && token1 && token.canister_id.toText() === token1.canister_id.toText()) ||
      (selectingFor === "token1" && token0 && token.canister_id.toText() === token0.canister_id.toText())
    ) {
      alert("❌ You cannot select the same token for both Token 0 and Token 1.");
      return;
    }

    if (selectingFor === "token0") {
      setToken0(token);
    } else {
      setToken1(token);
    }
    setOpenTokenSelect(false);
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-[#0f0f23]">
      <div className="bg-[#1a1a2e] p-8 rounded-2xl shadow-xl w-full max-w-md space-y-6">

        {/* Token Select */}
        <div className="flex flex-col space-y-4">
          <button
            onClick={() => { setOpenTokenSelect(true); setSelectingFor("token0"); }}
            className="w-full p-3 bg-gray-700 text-white rounded-lg"
          >
            {token0 ? `Token 0: ${token0.name}` : "Select Token 0"}
          </button>
          <button
            onClick={() => { setOpenTokenSelect(true); setSelectingFor("token1"); }}
            className="w-full p-3 bg-gray-700 text-white rounded-lg"
          >
            {token1 ? `Token 1: ${token1.name}` : "Select Token 1"}
          </button>
        </div>

        {/* Set Initial Price */}
        <div>
          <label className="text-sm text-gray-300 mb-2 block">Initial Price</label>
          <input
            type="number"
            value={initialPrice}
            onChange={(e) => setInitialPrice(e.target.value)}
            placeholder="Enter initial price"
            className="w-full p-3 bg-gray-800 text-white rounded-lg"
          />
        </div>

        {/* Amounts */}
        <div>
          <label className="text-sm text-gray-300 mb-2 block">Amount of {token0?.name || "Token 0"}</label>
          <input
            type="number"
            value={amountToken0}
            onChange={(e) => setAmountToken0(e.target.value)}
            placeholder="Enter amount of token 0"
            className="w-full p-3 bg-gray-800 text-white rounded-lg"
          />
        </div>

        <div>
          <label className="text-sm text-gray-300 mb-2 block">Amount of {token1?.name || "Token 1"}</label>
          <input
            type="number"
            value={amountToken1}
            readOnly
            placeholder="Auto-calculated"
            className="w-full p-3 bg-gray-700 text-white rounded-lg"
          />
        </div>

        {/* Create Pool */}
        <button
          onClick={handleCreatePool}
          disabled={!token0 || !token1 || !initialPrice || !amountToken0}
          className="w-full py-3 bg-green-400 hover:bg-green-500 text-black font-bold rounded-lg disabled:bg-gray-500 transition-all"
        >
          Create Pool
        </button>

        {/* Token Selection Modal */}
        {openTokenSelect && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-[#1a1a2e] p-6 rounded-lg space-y-4 w-80">
              <h2 className="text-white text-lg mb-4">Select a Token</h2>
              {tokens.map((token) => {
                const isDisabled =
                  (selectingFor === "token0" && token1 && token.canister_id.toText() === token1.canister_id.toText()) ||
                  (selectingFor === "token1" && token0 && token.canister_id.toText() === token0.canister_id.toText());

                return (
                  <button
                    key={token.canister_id.toText()}
                    onClick={() => handleTokenSelect(token)}
                    disabled={isDisabled}
                    className={`block w-full p-3 rounded-lg mb-2 ${isDisabled
                        ? "bg-gray-600 text-gray-400 cursor-not-allowed"
                        : "bg-gray-700 text-white hover:bg-gray-600"
                      }`}
                  >
                    {token.symbol || token.name}
                  </button>
                );
              })}


              <button
                onClick={() => setOpenTokenSelect(false)}
                className="w-full p-3 bg-red-500 text-white rounded-lg mt-2"
              >
                Cancel
              </button>
            </div>
          </div>
        )}

      </div>
    </div>
  );
}
