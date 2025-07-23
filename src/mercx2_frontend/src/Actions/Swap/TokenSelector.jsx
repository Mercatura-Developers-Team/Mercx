import React from "react";

export default function TokenSelector({ tokens, onSelect, onCancel }) {
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-[#1a1a2e] p-6 rounded-lg space-y-4 w-80">
        <h2 className="text-white text-lg text-center">Select a Token</h2>
        {tokens.length === 0 ? (
          <p className="text-gray-400 text-center">Coming soon...</p>
        ) : (
          tokens.map((token) => (
            <button
              key={token.canister_id.toText()}
              onClick={() => onSelect(token)}
              className="block w-full p-3 rounded-lg bg-gray-700 text-white hover:bg-gray-600 mb-2"
            >
              {token.symbol || token.name}
            </button>
          ))
        )}
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
