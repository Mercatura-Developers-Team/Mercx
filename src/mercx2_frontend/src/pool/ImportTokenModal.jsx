import React, { useState } from "react";

export default function ImportTokenModal({ onClose, onImport, isOpen }) {
  const [canisterId, setCanisterId] = useState("");
  const [error, setError] = useState(""); //validation message
  const [isSubmitting, setIsSubmitting] = useState(false); // New loading state


  const handleSubmit = async () => {
    if (!canisterId.trim()) {
        setError("Please enter a valid canister ID.");
        return;
      }

    setError(""); // clear any old errors
    setIsSubmitting(true); // Start loading

    try {
      await onImport(canisterId); // You handle backend call outside
      setCanisterId("");
      onClose();
    } catch (err) {
      console.error("Token import failed:", err);
      setError("Failed to import token.");
    } finally {
      setIsSubmitting(false); // Reset loading state
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/50 ">
      <div className="bg-[#1a1a2e] text-white p-6 rounded-xl shadow-2xl w-full max-w-md ">
        <h2 className="text-xl font-semibold mb-4">Add New Token</h2>
        <input
          type="text"
          placeholder="e.g. ryjl3-tyaaa-aaaaa-aaaba-cai"
          className="w-full p-3 bg-gray-800 border border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          value={canisterId}
          onChange={(e) => {
            setCanisterId(e.target.value);
            setError(""); // clear error as user types
          }}
        />
          {/* Show error message if input is invalid */}
          {error && <p className="text-red-500 text-sm mt-1">{error}</p>}
          
        <div className="flex gap-4 mt-6">
          <button
            onClick={handleSubmit}
            className="flex-1 p-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg disabled:opacity-50  "
            disabled={isSubmitting}
          >
                       {isSubmitting ? "Adding..." : "Add Token"}
          </button>
          <button
            onClick={onClose}
            className="flex-1 p-3 bg-gray-600 text-white rounded-lg"
            disabled={isSubmitting}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
