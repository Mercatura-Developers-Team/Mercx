import React, { useState } from "react";

export default function ImportTokenModal({ onClose, onImport, isOpen }) {
  const [canisterId, setCanisterId] = useState("");
  const [error, setError] = useState(""); //validation message

  const handleSubmit = async () => {
    if (!canisterId.trim()) {
        setError("Please enter a valid canister ID.");
        return;
      }

    setError(""); // clear any old errors
    await onImport(canisterId); // You handle backend call outside
    setCanisterId("");
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-[#1a1a2e] p-6 rounded-lg space-y-4 w-96">
        <h2 className="text-white text-lg mb-4">Add New Token</h2>
        <input
          type="text"
          placeholder="e.g. ryjl3-tyaaa-aaaaa-aaaba-cai"
          className="w-full p-3 bg-gray-800 text-white rounded-lg"
          value={canisterId}
          onChange={(e) => {
            setCanisterId(e.target.value);
            setError(""); // clear error as user types
          }}
        />
          {/* Show error message if input is invalid */}
          {error && <p className="text-red-500 text-sm mt-1">{error}</p>}
          
        <div className="flex gap-3 mt-4">
          <button
            onClick={handleSubmit}
            className="flex-1 p-3 bg-blue-600 text-white rounded-lg"
          >
            Add Token
          </button>
          <button
            onClick={onClose}
            className="flex-1 p-3 bg-gray-600 text-white rounded-lg"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
