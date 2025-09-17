import React, { useEffect, useState } from "react";
import { useAuth } from "../use-auth-client";
import { Principal } from "@dfinity/principal";
import { useNavigate } from 'react-router-dom';
import LoadingScreen from '../LoadingScreen'; // Import the reusable component


export default function Pools() {

  const { mercx_Actor } = useAuth();
  const [pools, setPools] = useState([]);
  const [hoveredRow, setHoveredRow] = useState(null);
  const [logos, setLogos] = useState({});
  const [prices, setPrices] = useState({});
const [poolAnalytics, setPoolAnalytics] = useState({}); // New state for analytics
const [loading, setLoading] = useState(true);


  const navigate = useNavigate();


  useEffect(() => {
    const loadPools = async () => {
      if (!mercx_Actor) return;

      try {
        setLoading(true);

        const result = await mercx_Actor.get_all_pools();

        if (result?.Ok) {
          setPools(result.Ok);
          const logoMap = await fetchTokenLogos(result.Ok); // ✅ use result.Ok, not raw result
          setLogos(logoMap);
        } else {
          console.error("get_all_pools error:", result?.Err || result);
        }

      } catch (err) {
        console.error("Failed to load pools or logos", err);
      }finally {
        setLoading(false);
      }
    };

    loadPools();
  }, [mercx_Actor]);


  useEffect(() => {
    const fetchPrices = async () => {
      if (!mercx_Actor || !pools.length) return;

      try {
        const priceEntries = await Promise.all(
          pools.map(async (pool) => {
            try {
              const token0 = pool.symbol_0;
              const token1 = pool.symbol_1;
              const key = `${token0}/${token1}`; // ✅ define key here
              const result = await mercx_Actor.get_pool_price(token0, token1);
              console.log(result);
              if (result && result.Ok !== undefined) {
                return [key, result.Ok];
              } else if (result && result.Err) {
                console.warn(`Price fetch error for  ${result.Err}`);
              }
            } catch (err) {
              console.warn(`Failed to get price for ${pool.symbol_0}/${pool.symbol_1}`, err);
              return null;
            }
          })
        );

        const filtered = priceEntries.filter(Boolean);
        const priceMap = Object.fromEntries(filtered);
        setPrices(priceMap);
      } catch (error) {
        console.error("Error fetching pool prices", error);
      }
    };
    fetchPrices();
  }, [mercx_Actor, pools]);

  // New useEffect to fetch pool analytics (TVL, Volume, APY)
  useEffect(() => {
    const fetchPoolAnalytics = async () => {
      if (!mercx_Actor || !pools.length) return;

      try {
        const analyticsEntries = await Promise.all(
          pools.map(async (pool) => {
            try {
              // Use get_pool_metrics to get all data in one call
              const result = await mercx_Actor.get_pool_metrics(pool.pool_id);
              
              if (result?.Ok) {
                return [pool.pool_id, {
                  tvl: result.Ok.tvl.tvl_usd,
                  volume_24h: result.Ok.volume.volume_24h_usd,
                  apy: result.Ok.apy,
                  loading: false
                }];
              } else {
                console.warn(`Failed to get metrics for pool ${pool.pool_id}:`, result?.Err);
                return [pool.pool_id, {
                  tvl: 0,
                  volume_24h: 0,
                  apy: 0,
                  loading: false
                }];
              }
            } catch (err) {
              console.warn(`Error fetching analytics for pool ${pool.pool_id}:`, err);
              return [pool.pool_id, {
                tvl: 0,
                volume_24h: 0,
                apy: 0,
                loading: false
              }];
            }
          })
        );

        const analyticsMap = Object.fromEntries(analyticsEntries);
        setPoolAnalytics(analyticsMap);
      } catch (error) {
        console.error("Error fetching pool analytics:", error);
      }
    };

    fetchPoolAnalytics();
  }, [mercx_Actor, pools]);


  const extractCanisterId = (canisterUrl) => {
    return canisterUrl.replace("canister://", "");
  };

  const fetchTokenLogos = async (pools) => {
    const logoMap = {};

    for (const pool of pools) {
      const canister0 = extractCanisterId(pool.address_0);
      const canister1 = extractCanisterId(pool.address_1);

      // Fetch logo for token 0
      if (!logoMap[pool.symbol_0]) {
        try {
          const principal0 = Principal.fromText(canister0);
          const url0 = await mercx_Actor.get_logo_url(principal0);
          logoMap[pool.symbol_0] = url0;
        } catch (e) {
          console.warn(`Failed to fetch logo for ${pool.symbol_0}`, e);
          logoMap[pool.symbol_0] = "/j.png";
        }
      }

      // Fetch logo for token 1
      if (!logoMap[pool.symbol_1]) {
        try {
          const principal1 = Principal.fromText(canister1);
          const url1 = await mercx_Actor.get_logo_url(principal1);
          logoMap[pool.symbol_1] = url1;
        } catch (e) {
          console.warn(`Failed to fetch logo for ${pool.symbol_1}`, e);
          logoMap[pool.symbol_1] = "/j.png";
        }
      }
    }
    console.log(logoMap);
    return logoMap;

  };

  const renderTokenLogos = (symbol_0, symbol_1) => (
    <div className="flex items-center gap-3 hover:scale-105 transition-transform">
      <div className="flex items-center">
        <img
          src={logos[symbol_0] || "/j.png"}
          alt={symbol_0}
          className="w-8 h-8 rounded-full shadow-md border border-slate-500"
        />
        <img
          src={logos[symbol_1] || "/j.png"}
          alt={symbol_1}
          className="w-8 h-8 rounded-full -ml-2 shadow-md border border-slate-500"
        />
      </div>
      <span className="text-white font-medium text-sm">{symbol_0}/{symbol_1}</span>
    </div>
  );

  // Helper function to format USD values
  const formatUSD = (value) => {
    if (value === 0) return '$0';
    if (value < 1000) return `$${value.toFixed(2)}`;
    if (value < 1000000) return `$${(value / 1000).toFixed(1)}K`;
    return `$${(value / 1000000).toFixed(1)}M`;
  };

  // Helper function to format percentage
  const formatPercentage = (value) => {
    if (value === 0) return '0%';
    return `${value.toFixed(2)}%`;
  };
  
  if (loading) {
    return (
      <LoadingScreen 
        title="Loading Pools" 
        subtitle="Fetching pool data and analytics..." 
        showSkeleton={true}
        skeletonType="table"
      />
    );
  }


  return (
    <div className="min-h-screen bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-10">
          <h2 className="text-3xl font-extrabold text-white sm:text-4xl">
            Liquidity Pools
          </h2>
          <p className="mt-3 text-xl text-gray-300">
            Provide liquidity and earn trading fees
          </p>
        </div>

        <div className="bg-slate-800 rounded-xl shadow-2xl overflow-hidden">
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-slate-700">
              <thead className="bg-slate-900">
                <tr>
                  <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Pool</th>
                  <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Price</th>
                  <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">TVL</th>
                  <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Volume (24h)</th>
                  <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Fee Tier</th>
                  <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">APY</th>
                  <th className="relative px-6 py-5"><span className="sr-only">Actions</span></th>
                </tr>
              </thead>
              <tbody className="bg-slate-800 divide-y divide-slate-700">
              {pools
  .filter((pool) => {
    const key = `${pool.symbol_0}/${pool.symbol_1}`;
    const price = prices[key];

    return price !== undefined && typeof price !== "object"; // ✅ filters out undefined or Err
  }).map((pool) => {
                  const key = `${pool.symbol_0}/${pool.symbol_1}`;
                  const price = prices[key];
                  const analytics = poolAnalytics[pool.pool_id];

                  return (
                    <tr
                      key={pool.pool_id}
                      className={`transition-all duration-200 ${hoveredRow === pool.pool_id ? "hover:bg-slate-700 scale-[1]" : "hover:bg-slate-750"
                        }`}
                      onMouseEnter={() => setHoveredRow(pool.pool_id)}
                      onMouseLeave={() => setHoveredRow(null)}
                    >
                      <td className="px-6 py-4 whitespace-nowrap">
                        {renderTokenLogos(pool.symbol_0, pool.symbol_1)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-white font-medium">
                        {price !== undefined ? price : 'Loading...'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-white">
                      {analytics?.loading === false ? (
                            <span className={analytics.tvl > 0 ? 'text-green-400' : 'text-gray-400'}>
                              {formatUSD(analytics.tvl)}
                            </span>
                          ) : (
                            <span className="text-gray-400">Loading...</span>
                          )}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-white">
                      {analytics?.loading === false ? (
                            <span className={analytics.volume_24h > 0 ? 'text-blue-400' : 'text-gray-400'}>
                              {formatUSD(analytics.volume_24h)}
                            </span>
                          ) : (
                            <span className="text-gray-400">Loading...</span>
                          )}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-purple-300">
                        {pool.lp_fee_bps / 100}%
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-green-400 font-medium"> 
                        {analytics?.loading === false ? (
                            <span className={`${analytics.apy > 0 ? 'text-green-400' : 'text-gray-400'}`}>
                              {formatPercentage(analytics.apy)}
                            </span>
                          ) : (
                            <span className="text-gray-400">Loading...</span>
                          )}
                          </td>
                      <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                        <div className="flex space-x-2">
                          <button
                            className="text-indigo-400 hover:text-indigo-300 bg-indigo-900/30 hover:bg-indigo-900/50 px-3 py-1 rounded-md text-xs font-medium transition-colors"
                            onClick={() => navigate(`/addPool?token0=${pool.symbol_0}&token1=${pool.symbol_1}`)}
                          >
                            Add Liquidity
                          </button>
                          <button className="text-white hover:text-gray-200 bg-indigo-600 hover:bg-indigo-700 px-3 py-1 rounded-md text-xs font-medium transition-colors" onClick={() => navigate(`/trade?tab=Swap&from=${pool.symbol_0}&to=${pool.symbol_1}`)}>
                              Trade
                            </button>
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>

        <div className="mt-8 text-center">
          <button
            className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
            onClick={() => navigate('/addPool')}
          >
            <svg className="-ml-1 mr-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clipRule="evenodd" />
            </svg>
            Create New Pool
          </button>
        </div>
      </div>
    </div>
  )
}
