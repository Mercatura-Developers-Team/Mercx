import React from "react";

export default function PoolInfo({ token0, token1, poolStats }) {
  if (!token0 || !token1) return null;
  if (!poolStats) return null;

  if (poolStats.message) {
    return (
      <div className="bg-yellow-800/20 text-yellow-300 border border-yellow-600 p-3 rounded-xl w-full g:w-[80%] max-w-3xl shadow-md">
        <p className="font-medium text-sm">
          No pool exists for <strong>{token0.symbol}/{token1.symbol}</strong>.<br />
          Create the first liquidity position!
        </p>
      </div>
    );
  }

  return (
    <div className="bg-[#1a1a2e] p-4 rounded-xl shadow-xl w-full g:w-[80%] max-w-3xl">
      <h3 className="text-white text-base font-semibold mb-3">Pool Information</h3>

      <div className="space-y-2 text-sm">
        <div className="flex justify-between">
          <span className="text-gray-400">Pair</span>
          <span className="text-white">{token0.symbol}/{token1.symbol}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-400">Fee Tier</span>
          <span className="text-white">0.30%</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-400">{token0.symbol} Balance</span>
          <span className="text-white">{poolStats.token0Balance || '0'} {token0.symbol}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-400">{token1.symbol} Balance</span>
          <span className="text-white">{poolStats.token1Balance || '0'} {token1.symbol}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-gray-400">TVL</span>
          <span className="text-white">${poolStats.tvl ? poolStats.tvl.toLocaleString() : '0'}</span>
        </div>
      </div>
    </div>
  );
}
