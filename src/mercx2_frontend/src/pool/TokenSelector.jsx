import React from "react";

export default function TokenSelector({
  tokens,
  selectingFor,
  token0,
  token1,
  onSelect,
  onImport,
  onCancel,
}) {
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-[#1a1a2e] p-6 rounded-lg space-y-4 w-80">
        <h2 className="text-white text-lg">Select a Token</h2>
        {tokens.map((token) => {
          const isDisabled =
            (selectingFor === "token0" &&
              token1?.canister_id.toText() === token.canister_id.toText()) ||
            (selectingFor === "token1" &&
              token0?.canister_id.toText() === token.canister_id.toText());

          return (
            <button
              key={token.canister_id.toText()}
              onClick={() => onSelect(token)}
              disabled={isDisabled}
              className={`block w-full p-3 rounded-lg mb-2 ${
                isDisabled
                  ? "bg-gray-600 text-gray-400 cursor-not-allowed"
                  : "bg-gray-700 text-white hover:bg-gray-600"
              }`}
            >
              {token.symbol || token.name}
            </button>
          );
        })}

        <button
          onClick={onImport}
          className="w-full p-3 bg-gradient-to-r from-indigo-500 to-indigo-700 hover:from-indigo-700 text-white disabled:bg-gray-500 disabled:cursor-not-allowed disabled:hover:from-gray-500 rounded-lg"
        >
          + Import Token
        </button>

        <button
          onClick={onCancel}
          className="w-full p-3 bg-red-500 text-white rounded-lg"
        >
          Cancel
        </button>
      </div>
    </div>
  );
}
