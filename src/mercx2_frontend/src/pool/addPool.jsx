import React, { useState, useEffect } from "react";
import { useAuth } from "../use-auth-client";
import ImportTokenModal from "./ImportTokenModal";
import { Principal } from "@dfinity/principal";
import { useFormik } from "formik";
import * as Yup from "yup";
import PoolInfo from "./PoolInfo";
import { useSearchParams } from "react-router-dom";
import SuccessModal from "./SuccessModel"; // update path if needed
import { parseAmount, normalizeAmount } from "./tokenUtils";
import TokenSelector from "./TokenSelector";

export default function CreatePool() {
  const { mercx_Actor } = useAuth();
  const [token0, setToken0] = useState(null);
  const [token1, setToken1] = useState(null);
  const [tokens, setTokens] = useState([]);
  const [poolExists, setPoolExists] = useState(false);
  const [isCreating, setIsCreating] = useState(false);

  const [openTokenSelect, setOpenTokenSelect] = useState(false);
  const [selectingFor, setSelectingFor] = useState(null); // "token0" or "token1"
  const [showImportModal, setShowImportModal] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [successAction, setSuccessAction] = useState("create");
  const [searchParams, setSearchParams] = useSearchParams();
  const { createTokenActor, principal, isAuthenticated } = useAuth();
  const [formError, setFormError] = useState("");
  const [lastEditedField, setLastEditedField] = useState(null); // 'amountToken0' or 'amountToken1'
  
 
  
  
  const formik = useFormik({
    initialValues: {
      initialPrice: "",
      amountToken0: "",
      amountToken1: "",
    },
    validationSchema: Yup.object({
      initialPrice: Yup.number().when([], {
        is: () => !poolExists,
        then: (schema) =>
          schema.typeError("Enter a valid price").positive("Must be > 0").required("Required"),
        otherwise: (schema) => schema.notRequired(),
      }),
      amountToken0: Yup.number().typeError("Must be a number").positive("> 0").required("Required"),
      amountToken1: Yup.number().typeError("Must be a number").positive("> 0").required("Required"),
    }),
    onSubmit: async (values) => {
      setIsCreating(true);
      setFormError("");
      const spenderId = "ajuq4-ruaaa-aaaaa-qaaga-cai";
      try {
        const amount0 = parseAmount(values.amountToken0, token0.decimals) + BigInt(20_000);
        const amount1 = parseAmount(values.amountToken1, token1.decimals) + BigInt(20_000);

        const token0Actor = await createTokenActor(token0.canister_id.toText());
        const token1Actor = await createTokenActor(token1.canister_id.toText());

        // Get balances first
        const balance0Res = await token0Actor.icrc1_balance_of({
          owner: principal,
          subaccount: [],
        });

        const balance1Res = await token1Actor.icrc1_balance_of({
          owner: principal,
          subaccount: [],
        });

        if (balance0Res < amount0) throw new Error(`❌ Insufficient ${token0.symbol} balance`);
        if (balance1Res < amount1) throw new Error(`❌ Insufficient ${token1.symbol} balance`);

        //check allowance
        const allowanceCheck0 = {
          account: { owner: principal, subaccount: [] },
          spender: { owner: Principal.fromText(spenderId), subaccount: [] },
        };

        const allowanceCheck1 = {
          account: { owner: principal, subaccount: [] },
          spender: { owner: Principal.fromText(spenderId), subaccount: [] },
        };

        const currentAllowance0 = (await token0Actor.icrc2_allowance(allowanceCheck0)).allowance;
        const currentAllowance1 = (await token1Actor.icrc2_allowance(allowanceCheck1)).allowance;

        if (currentAllowance0 < amount0) {
          const approve0 = await token0Actor.icrc2_approve({
            spender: { owner: Principal.fromText(spenderId), subaccount: [] },
            amount: amount0,
            fee: [],
            memo: [],
            from_subaccount: [],
            created_at_time: [],
            expires_at: [],
            expected_allowance: [],
          });

          if ("Err" in approve0) throw new Error("Token0 approval failed: " + JSON.stringify(approve0.Err));
        }

        if (currentAllowance1 < amount1) {
          const approve1 = await token1Actor.icrc2_approve({
            spender: { owner: Principal.fromText(spenderId), subaccount: [] },
            amount: amount1,
            fee: [],
            memo: [],
            from_subaccount: [],
            created_at_time: [],
            expires_at: [],
            expected_allowance: [],
          });

          if ("Err" in approve1) throw new Error("Token1 approval failed: " + JSON.stringify(approve1.Err));
        }

        if (poolExists) {
          let addResult;
          try {
           addResult = await mercx_Actor.add_liquidity_transfer_from({
            token_0: token0.symbol,
            token_1:  token1.symbol,
            amount_0: parseAmount(values.amountToken0, token0.decimals),
            tx_id_0: [],
            amount_1: parseAmount(values.amountToken1, token1.decimals),
            tx_id_1: [],
          });
          console.log("Add Liquidity Result:", addResult);

          if ("Err" in addResult) {
            // Try reverse order if first attempt fails
            addResult = await mercx_Actor.add_liquidity_transfer_from({
              token_0: token1.symbol,
              token_1: token0.symbol,
              amount_0: parseAmount(values.amountToken1, token1.decimals), // Note the swap
              tx_id_0: [],
              amount_1: parseAmount(values.amountToken0, token0.decimals), // Note the swap
              tx_id_1: [],
            });
          }
          setShowSuccessModal(true);
          setSuccessAction("add");
        }
        catch (err) {
          console.error("Add liquidity failed:", err);
          setFormError(err.message || "Failed to add liquidity. Please try again.");
        } }

        else {

        
        const args = {
          token_0: token0.canister_id.toText(),
          token_1: token1.canister_id.toText(),
          amount_0: parseAmount(values.amountToken0, token0.decimals),
          tx_id_0: [],
          amount_1: parseAmount(values.amountToken1, token1.decimals),
          tx_id_1: [],
          lp_fee_bps: [],
        };

        const result = await mercx_Actor.add_pool(args);
        console.log("Result:", result);
        setShowSuccessModal(true);
        setSuccessAction("create");
      }
      } catch (err) {
        console.error(" Pool creation failed:", err);
        setFormError(err.message || "Something went wrong. Please try again.");
      
      } finally {
        setIsCreating(false);
      }
    },
  });


  const [poolStats, setPoolStats] = useState(null); //information 

  useEffect(() => {
    if (token0 && token1 && mercx_Actor) {
      (async () => {
        try {
          const exists = await mercx_Actor.pool_exists(
            token0.canister_id.toText(),
            token1.canister_id.toText()
          );

          if (exists) {
            setPoolExists(exists);
            // Try both symbol combinations since order matters
            let pool;
            try {
              pool = await mercx_Actor.get_by_tokens(token0.symbol, token1.symbol);
              if ("Err" in pool) {
                // Try reverse order if first attempt fails
                pool = await mercx_Actor.get_by_tokens(token1.symbol, token0.symbol);
              }
            } catch (err) {
              console.warn("Pool lookup failed:", err);
            }

            if (pool && "Ok" in pool) {
              const data = pool.Ok;
              let balForToken0 = data.balance_0;
              let balForToken1 = data.balance_1;
              if (data.token_id_0 !== token0.token_id) {
                // UI order is opposite to canister order ➜ swap
                balForToken0 = data.balance_1;
                balForToken1 = data.balance_0;
              }
              setPoolStats({
                poolId: data.pool_id,
                token0Balance: normalizeAmount(balForToken0, token0.decimals),
                token1Balance: normalizeAmount(balForToken1, token1.decimals),
                tvl: Number(data.amount_0) + Number(data.amount_1),
              });
            } else {
              console.warn("Pool exists but get_by_tokens returned error:", pool?.Err);
              setPoolStats({
                message: `Pool found but data could not be loaded`,
              });
            }
          } else {
            setPoolExists(false);
            setPoolStats({
              message: `No pool exists for ${token0.symbol}/${token1.symbol}. Create the first liquidity position!`,
            });
          }
        } catch (err) {
          console.error("Failed to fetch pool stats:", err);
          setPoolStats({
            message: `Error fetching pool data for ${token0.symbol}/${token1.symbol}.`,
          });
        }
      })();
    } else {
      setPoolStats(null);
    }
  }, [token0, token1, isCreating]);

  useEffect(() => {
  if (!token0 || !token1 || !poolExists || !lastEditedField || !mercx_Actor) return;

const calculateAmounts = async () => {
  try {
    const amountStr =
      lastEditedField === "amountToken0"
        ? formik.values.amountToken0
        : formik.values.amountToken1;

    if (!amountStr || isNaN(+amountStr) || +amountStr <= 0) return;

    const parsedAmount = parseAmount(
      amountStr,
      lastEditedField === "amountToken0" ? token0.decimals : token1.decimals
    );

    // Call canister
    const resp = await mercx_Actor.add_liquidity_amounts(
      lastEditedField === "amountToken0" ? token0.symbol : token1.symbol,
      parsedAmount,
      lastEditedField === "amountToken0" ? token1.symbol : token0.symbol
    );

    if ("Err" in resp) {
      console.error("Calculation failed:", resp.Err);
      return;
    }

    // ------ here is the key change ------
    const {
      symbol_0,
      amount_0,
      amount_1,
    } = resp.Ok; // <----- destructure once

    // Decide which amount belongs to which field
    if (lastEditedField === "amountToken0") {
      // user typed in Token0, so auto-fill Token1
      const otherAmount =
        symbol_0 === token0.symbol ? amount_1 : amount_0;
      formik.setFieldValue(
        "amountToken1",
        normalizeAmount(otherAmount, token1.decimals)
      );
    } else {
      // user typed in Token1, so auto-fill Token0
      const otherAmount =
        symbol_0 === token0.symbol ? amount_0 : amount_1;
      formik.setFieldValue(
        "amountToken0",
        normalizeAmount(otherAmount, token0.decimals)
      );
    }
  } catch (err) {
    console.error("Failed to calculate amounts:", err);
  }
};


  const debounceTimer = setTimeout(calculateAmounts, 500);
  return () => clearTimeout(debounceTimer);
}, [formik.values.amountToken0, formik.values.amountToken1, lastEditedField, token0, token1, poolExists]);

  //URL
  useEffect(() => {
    if (tokens.length > 0) {
      const t0 = searchParams.get("token0");
      const t1 = searchParams.get("token1");

      const foundToken0 = tokens.find((t) => t.symbol === t0);
      const foundToken1 = tokens.find((t) => t.symbol === t1);

      if (foundToken0) setToken0(foundToken0);
      if (foundToken1) setToken1(foundToken1);
    }
  }, [tokens]);

  // 👉 NEW useEffect to keep URL updated when token0/token1 change
  useEffect(() => {
    if (token0 && token1) {
      setSearchParams({
        token0: token0.symbol,
        token1: token1.symbol,
      });
    }
  }, [token0, token1, setSearchParams]);

  useEffect(() => {
    if (poolExists) return; // Skip for existing pools
  
    const price = parseFloat(formik.values.initialPrice);
    const val0 = parseFloat(formik.values.amountToken0);
    const val1 = parseFloat(formik.values.amountToken1);
  
    if (!price || price <= 0) return;
  
    if (lastEditedField === 'amountToken0' && !isNaN(val0)) {
      formik.setFieldValue("amountToken1", (val0 * price).toFixed(token1?.decimals || 8));
    } else if (lastEditedField === 'amountToken1' && !isNaN(val1)) {
      formik.setFieldValue("amountToken0", (val1 / price).toFixed(token0?.decimals || 8));
    }
  }, [formik.values.initialPrice, formik.values.amountToken0, formik.values.amountToken1, lastEditedField, poolExists]);


  useEffect(() => {
    const fetchTokens = async () => {
      if (!mercx_Actor) return;  // Exit early if actor not ready
      try {
        const tokenList = await mercx_Actor.get_all_tokens();
        setTokens(tokenList);
      } catch (err) {
        console.error("Failed to fetch tokens:", err);
      }
    };

    fetchTokens();
  }, [mercx_Actor]);



  const handleTokenSelect = (token) => {
    if (
      (selectingFor === "token0" && token1 && token.canister_id.toText() === token1.canister_id.toText()) ||
      (selectingFor === "token1" && token0 && token.canister_id.toText() === token0.canister_id.toText())
    ) {
      setFormError(err.message || "Something went wrong. Please try again.");
      return;
    }

    if (selectingFor === "token0") {
      setToken0(token);
    } else {
      setToken1(token);
    }
    setOpenTokenSelect(false);
  };

  const poolHeader = token0 && token1
    ? `${token0.symbol || token0.name}/${token1.symbol || token1.name} Pool`
    : "Select Tokens to Create a Pool";


  return (
    <div className="min-h-screen flex items-center justify-center bg-[#0f0f23] px-4 py-8">
      <div className="w-full max-w-6xl"> {/* Increased max width */}
        <div className="flex flex-col lg:flex-row gap-6">
          {/* Left Column - Form */}
          <div className="w-full lg:w-2/3 bg-[#1a1a2e] p-6 rounded-xl shadow-xl space-y-6">
            {/* Pool header */}
            <h2 className="text-white text-xl font-semibold">{poolHeader}</h2>
            {!poolExists && token0 && token1 && (
              <div className="text-yellow-400 bg-yellow-900/20 p-3 rounded-md text-sm">
                🚀 New Pool — You are the first to create this pair. Set its initial ratio.
              </div>
            )}
            {formError && (
              <div className="mt-3 bg-red-800/20 border border-red-600 text-red-400 text-sm rounded-lg p-3">
                {formError}
              </div>
            )}
            {/* Token Select */}
            <div className="flex gap-4">
              <button
                onClick={() => { setOpenTokenSelect(true); setSelectingFor("token0"); }}
                className="flex-1 bg-gray-700 text-white p-3 rounded-lg text-center"
              >
                {token0 ? `${token0.name}` : "Select Token 0"}
              </button>
              <button
                onClick={() => { setOpenTokenSelect(true); setSelectingFor("token1"); }}
                className="flex-1 bg-gray-700 text-white p-3 rounded-lg text-center"
              >
                {token1 ? `${token1.name}` : "Select Token 1"}
              </button>
            </div>

            <form onSubmit={formik.handleSubmit} className="space-y-4">
              {/* Set Initial Price */}
              {!poolExists && (
                <div>
                  <label className="text-sm text-gray-300">Initial Price</label>
                  <input
                    type="number"
                    // value={initialPrice}
                    // onChange={(e) => setInitialPrice(e.target.value)}
                    name="initialPrice"
                    value={formik.values.initialPrice}
                    onChange={formik.handleChange}
                    placeholder="Enter initial price"
                    className="w-full p-3 bg-gray-800 text-white rounded-lg"
                  />
                  <p className="text-red-400 text-xs">{formik.errors.initialPrice}</p>

                </div>
              )}

              {/* Amounts */}

              <div className="w-full border border-gray-700 bg-[#1a1a2e] rounded-xl p-6 space-y-6">
                <h4 className="text-white text-base font-semibold mb-2">Token Amounts</h4>

                <div>
                  <label className="text-sm text-gray-300 ">Amount of {token0?.name || "Token 0"}</label>
                  <input
                    type="text"
                    name="amountToken0"
                    value={formik.values.amountToken0}
                    onChange={(e) => {
                      formik.handleChange(e);
                      formik.setTouched({ amountToken0: true });
                      setLastEditedField('amountToken0');
                    }}
                    placeholder="0"
                    className="w-full p-3 bg-gray-800 text-white rounded-lg"
                  />
                  <p className="text-red-400 text-xs">{formik.errors.amountToken0}</p>
                </div>
                <div>
                  <label className="text-sm text-gray-300 ">Amount of {token1?.name || "Token 1"}</label>
                  <input
                    type="text"
                    name="amountToken1"
                    value={formik.values.amountToken1}
                    onChange={(e) => {
                      formik.handleChange(e);
                      formik.setTouched({ amountToken1: true });
                      setLastEditedField('amountToken1');
                    }}
                    placeholder="0"
                    className="w-full p-3 bg-gray-700 text-white rounded-lg"
                  />
                  <p className="text-red-400 text-xs">{formik.errors.amountToken1}</p>
                </div>
              </div>


              {/* Create Pool */}
              <button
                type="submit"
                disabled={!isAuthenticated || isCreating || !token0 || !token1 || (!poolExists && !formik.values.initialPrice) ||
                  !/^[0-9]*[.]?[0-9]+$/.test(formik.values.amountToken0) ||
                  !/^[0-9]*[.]?[0-9]+$/.test(formik.values.amountToken1) ||
                  (formik.errors.amountToken0 || formik.errors.amountToken1 || formik.errors.initialPrice)}
                className={`w-full font-bold py-3 rounded-lg ${isCreating
 ? "bg-gray-500 cursor-not-allowed "
      : "bg-gradient-to-r from-indigo-500 to-indigo-700 hover:from-indigo-700 text-white"
                  }`}
              >
                {isCreating ? "Creating..." : poolExists ? "Add Liquidity" : "Create Pool"}
              </button>

            </form>
          </div>
          {/* Right Column - Pool Info */}
          <div className="w-full lg:w-1/3">
            <PoolInfo token0={token0} token1={token1} poolStats={poolStats} />
          </div>
        </div>


        {/* Token Selection Modal */}
        {openTokenSelect && (
  <TokenSelector
    tokens={tokens}
    selectingFor={selectingFor}
    token0={token0}
    token1={token1}
    onSelect={handleTokenSelect}
    onImport={() => {
      setShowImportModal(true);
      setOpenTokenSelect(false);
    }}
    onCancel={() => setOpenTokenSelect(false)}
  />
)}
        <ImportTokenModal
          isOpen={showImportModal}
          onClose={() => setShowImportModal(false)}
          onImport={async (canisterIdString) => {
            try {
              // Validate Principal format
              let validatedPrincipal;
              try {
                validatedPrincipal = Principal.fromText(canisterIdString);
              } catch (err) {
                alert("❌ Invalid Canister ID format. Please use a valid Principal.");
                return;
              }
              // Now call with a valid Principal string
              const result = await mercx_Actor.add_token(validatedPrincipal);

              if ("Ok" in result) {
                alert("✅ Token imported successfully!");
                const updatedTokens = await mercx_Actor.get_all_tokens();
                setTokens(updatedTokens);
              } else {
                alert("❌ " + result.Err);
              }
            } catch (err) {
              console.error("Import token error:", err);
              alert("❌ Failed to import token.");
            }
          }}

        />
        <SuccessModal isVisible={showSuccessModal}  action={successAction} onClose={() => setShowSuccessModal(false)} />

      </div>
    </div>
  );
}
