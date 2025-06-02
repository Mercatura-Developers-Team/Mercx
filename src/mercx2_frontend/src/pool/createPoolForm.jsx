import React from "react";
import { useFormik } from "formik";
import * as Yup from "yup";

export default function CreatePoolForm({
  token0,
  token1,
  poolExists,
  onSubmit,
  isCreating,
  formError,
}) {
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
      amountToken0: Yup.number().typeError("Must be a number").positive("Must be > 0").required("Required"),
      amountToken1: Yup.number().typeError("Must be a number").positive("Must be > 0").required("Required"),
    }),
    onSubmit,
  });

  return (
    <form onSubmit={formik.handleSubmit} className="space-y-4">
      {!poolExists && (
        <div>
          <label className="text-sm text-gray-300">Initial Price</label>
          <input
            type="number"
            name="initialPrice"
            value={formik.values.initialPrice}
            onChange={formik.handleChange}
            placeholder="Enter initial price"
            className="w-full p-3 bg-gray-800 text-white rounded-lg"
          />
          <p className="text-red-400 text-xs">{formik.errors.initialPrice}</p>
        </div>
      )}

      <div className="w-full border border-gray-700 bg-[#1a1a2e] rounded-xl p-6 space-y-6">
        <h4 className="text-white text-base font-semibold mb-2">Token Amounts</h4>

        <div>
          <label className="text-sm text-gray-300">Amount of {token0?.name || "Token 0"}</label>
          <input
            type="text"
            name="amountToken0"
            value={formik.values.amountToken0}
            onChange={(e) => {
              formik.handleChange(e);
              formik.setTouched({ amountToken0: true });
            }}
            placeholder="0"
            className="w-full p-3 bg-gray-800 text-white rounded-lg"
          />
          <p className="text-red-400 text-xs">{formik.errors.amountToken0}</p>
        </div>

        <div>
          <label className="text-sm text-gray-300">Amount of {token1?.name || "Token 1"}</label>
          <input
            type="text"
            name="amountToken1"
            value={formik.values.amountToken1}
            onChange={(e) => {
              formik.handleChange(e);
              formik.setTouched({ amountToken1: true });
            }}
            placeholder="0"
            className="w-full p-3 bg-gray-700 text-white rounded-lg"
          />
          <p className="text-red-400 text-xs">{formik.errors.amountToken1}</p>
        </div>
      </div>

      {formError && (
        <div className="mt-3 bg-red-800/20 border border-red-600 text-red-400 text-sm rounded-lg p-3">
          {formError}
        </div>
      )}

      <button
        type="submit"
        disabled={isCreating ||
          !token0 ||
          !token1 ||
          (!poolExists && !formik.values.initialPrice) ||
          !/^[0-9]*[.]?[0-9]+$/.test(formik.values.amountToken0) ||
          !/^[0-9]*[.]?[0-9]+$/.test(formik.values.amountToken1) ||
          (formik.errors.amountToken0 || formik.errors.amountToken1 || formik.errors.initialPrice)}
        className={`w-full font-bold py-3 rounded-lg ${
          isCreating
            ? "bg-gray-500 cursor-not-allowed"
            : "bg-green-500 hover:bg-green-600 text-black"
        }`}
      >
        {isCreating ? "Creating..." : poolExists ? "Add Liquidity" : "Create Pool"}
      </button>
    </form>
  );
}
