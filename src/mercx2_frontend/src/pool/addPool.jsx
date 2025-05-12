import React, { useState, useEffect } from "react";
import { useAuth } from "../use-auth-client";
import ImportTokenModal from "./ImportTokenModal";
import { Principal } from "@dfinity/principal";
import { useFormik } from "formik";
import * as Yup from "yup";

export default function CreatePool() {
  const { mercx_Actor } = useAuth();
  const [token0, setToken0] = useState(null);
  const [token1, setToken1] = useState(null);
  const [tokens, setTokens] = useState([]);
  const [poolExists, setPoolExists] = useState(false);

  const [openTokenSelect, setOpenTokenSelect] = useState(false);
  const [selectingFor, setSelectingFor] = useState(null); // "token0" or "token1"
  const [showImportModal, setShowImportModal] = useState(false);

  const formik = useFormik({
    initialValues: {
      initialPrice: "",
      amountToken0: "0",
      amountToken1: "0",
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
      const args = {
        token_0: token0.canister_id.toText(),
        token_1: token1.canister_id.toText(),
        amount_0: Number(values.amountToken0),
        amount_1: Number(values.amountToken1),
        lp_fee_bps: [],
      };
      try {
        const result = await mercx_Actor.add_pool(args);
        console.log("Result:", result);
      } catch (err) {
        alert("❌ Failed to add pool: " + err);
      }
    },
  });


  useEffect(() => {
    const price = parseFloat(formik.values.initialPrice);
    const val0 = parseFloat(formik.values.amountToken0);
    const val1 = parseFloat(formik.values.amountToken1);

    if (!price || price <= 0) return;

    if (formik.touched.amountToken0 && !isNaN(val0)) {
      formik.setFieldValue("amountToken1", (val0 * price));
    } else if (formik.touched.amountToken1 && !isNaN(val1)) {
      formik.setFieldValue("amountToken0", (val1 / price));
    }
  }, [formik.values.initialPrice, formik.values.amountToken0, formik.values.amountToken1]);


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

  useEffect(() => {
    const checkPool = async () => {
      if (token0 && token1 && mercx_Actor) {
        try {
          const exists = await mercx_Actor.pool_exists(
            token0.canister_id.toText(),
            token1.canister_id.toText()
          );
          setPoolExists(exists);
        } catch (e) {
          console.warn("Pool existence check failed:", e);
          setPoolExists(false);
        }
      }
    };
    checkPool();
  }, [token0, token1]);




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

  const poolHeader = token0 && token1
    ? `${token0.symbol || token0.name}/${token1.symbol || token1.name} Pool`
    : "Select Tokens to Create a Pool";


  return (
    <div className="min-h-screen flex items-center justify-center bg-[#0f0f23] px-4">
      <div className="bg-[#1a1a2e] p-6 rounded-xl shadow-xl w-full max-w-2xl space-y-6">

        {/* Pool header */}
        <h2 className="text-white text-xl font-semibold">{poolHeader}</h2>
        {!poolExists && token0 && token1 && (
          <div className="text-yellow-400 bg-yellow-900/20 p-3 rounded-md text-sm">
            🚀 New Pool — You are the first to create this pair. Set its initial ratio.
          </div>
        )}

        {/* Token Select */}
        <div className="flex gap-4">
          <button
            onClick={() => { setOpenTokenSelect(true); setSelectingFor("token0"); }}
            className="flex-1 bg-gray-700 text-white p-3 rounded-lg text-center"
          >
            {token0 ? `Token 0: ${token0.name}` : "Select Token 0"}
          </button>
          <button
            onClick={() => { setOpenTokenSelect(true); setSelectingFor("token1"); }}
            className="flex-1 bg-gray-700 text-white p-3 rounded-lg text-center"
          >
            {token1 ? `Token 1: ${token1.name}` : "Select Token 1"}
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
          <div>
            <label className="text-sm text-gray-300 ">Amount of {token0?.name || "Token 0"}</label>
            <input
              type="text"
              // value={amountToken0}
              // onChange={(e) => {
              //   setAmountToken0(e.target.value);
              //   setLastChangedField("token0");
              // }}
              name="amountToken0"
              value={formik.values.amountToken0}
              onChange={(e) => {
                formik.handleChange(e);
                formik.setTouched({ amountToken0: true });
              }}
              placeholder="Enter amount of token 0"
              className="w-full p-3 bg-gray-800 text-white rounded-lg"
            />
            <p className="text-red-400 text-xs">{formik.errors.amountToken0}</p>

          </div>

          <div>
            <label className="text-sm text-gray-300 ">Amount of {token1?.name || "Token 1"}</label>
            <input
              type="text"
              // value={amountToken1}
              // onChange={(e) => {
              //   setAmountToken1(e.target.value);
              //   setLastChangedField("token1");
              // }}
              name="amountToken1"
              value={formik.values.amountToken1}
              onChange={(e) => {
                formik.handleChange(e);
                formik.setTouched({ amountToken1: true });
              }}
              placeholder="Auto-calculated"
              className="w-full p-3 bg-gray-700 text-white rounded-lg"
            />
            <p className="text-red-400 text-xs">{formik.errors.amountToken1}</p>

          </div>


          {/* Create Pool */}
          <button
            type="submit"
            disabled={!token0 || !token1 || (!poolExists && !formik.values.initialPrice) ||
              !/^[0-9]*[.]?[0-9]+$/.test(formik.values.amountToken0) ||
              !/^[0-9]*[.]?[0-9]+$/.test(formik.values.amountToken1) ||
              (formik.errors.amountToken0 || formik.errors.amountToken1 || formik.errors.initialPrice)}
            className="w-full bg-green-500 text-black font-bold py-3 rounded-lg hover:bg-green-600 disabled:bg-gray-500"
          >
            {poolExists ? "Add Liquidity" : "Create Pool"}
          </button>
        </form>

        {/* Token Selection Modal */}
        {openTokenSelect && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-[#1a1a2e] p-6 rounded-lg space-y-4 w-80">
              <h2 className="text-white text-lg ">Select a Token</h2>
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
                onClick={() => {
                  setShowImportModal(true);
                  setOpenTokenSelect(false);
                }}
                className="w-full p-3 bg-blue-500 text-white rounded-lg "
              >
                + Import Token
              </button>

              <button
                onClick={() => setOpenTokenSelect(false)}
                className="w-full p-3 bg-red-500 text-white rounded-lg "
              >
                Cancel
              </button>
            </div>
          </div>
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

      </div>
    </div>
  );
}
