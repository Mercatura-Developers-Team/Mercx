import React, { useEffect, useState } from "react";
import { useAuth } from "../use-auth-client";
import { Principal } from "@dfinity/principal";
import { useNavigate } from 'react-router-dom';
import LoadingScreen from '../LoadingScreen';

export default function TableLiquidity() {
    const { mercx_Actor, principal } = useAuth();
    const [pools, setPools] = useState([]);
    const [hoveredRow, setHoveredRow] = useState(null);
    const [logos, setLogos] = useState({});
    const [lpTokens, setLpTokens] = useState([]);
    const [loading, setLoading] = useState(true);
    const [loadingStage, setLoadingStage] = useState("Initializing...");

    const navigate = useNavigate();

    // Cache for logos to avoid refetching
    const logoCache = new Map();

    useEffect(() => {
        const loadAllData = async () => {
            if (!mercx_Actor) return;

            try {
                setLoading(true);
                
                // Stage 1: Load pools and LP tokens in parallel
                setLoadingStage("Loading pools and positions...");
                const [poolsResult, lpTokensResult] = await Promise.allSettled([
                    mercx_Actor.get_all_pools(),
                    principal ? mercx_Actor.get_lp_tokens_by_principal(principal) : Promise.resolve([])
                ]);

                // Process pools
                if (poolsResult.status === 'fulfilled' && poolsResult.value?.Ok) {
                    setPools(poolsResult.value.Ok);
                } else {
                    console.error("get_all_pools error:", poolsResult.reason || poolsResult.value?.Err);
                    setPools([]);
                }

                // Process LP tokens
                if (lpTokensResult.status === 'fulfilled') {
                    setLpTokens(lpTokensResult.value);
                } else {
                    console.error("Failed to fetch LP tokens:", lpTokensResult.reason);
                    setLpTokens([]);
                }

                // Stage 2: Load logos in parallel (only for pools with user liquidity)
                if (poolsResult.status === 'fulfilled' && poolsResult.value?.Ok) {
                    setLoadingStage("Loading token logos...");
                    const logoMap = await fetchTokenLogosOptimized(poolsResult.value.Ok, lpTokensResult.value || []);
                    setLogos(logoMap);
                }

            } catch (err) {
                console.error("Failed to load data:", err);
            } finally {
                setLoading(false);
                setLoadingStage("");
            }
        };

        loadAllData();
    }, [mercx_Actor, principal]);

    // Optimized logo fetching - only fetch logos for pools with user liquidity
    const fetchTokenLogosOptimized = async (allPools, userLpTokens) => {
        // First, determine which pools have user liquidity
        const poolsWithLiquidity = allPools.filter(pool => {
            const hasLiquidity = checkUserHasLiquidity(pool.symbol_0, pool.symbol_1, userLpTokens);
            return hasLiquidity;
        });

        // If no liquidity positions, return empty map
        if (poolsWithLiquidity.length === 0) {
            return {};
        }

        const logoMap = {};
        const logoPromises = [];

        // Collect unique tokens from pools with liquidity
        const uniqueTokens = new Set();
        poolsWithLiquidity.forEach(pool => {
            uniqueTokens.add(pool.symbol_0);
            uniqueTokens.add(pool.symbol_1);
        });

        // Create promises for each unique token
        Array.from(uniqueTokens).forEach(symbol => {
            const pool = poolsWithLiquidity.find(p => p.symbol_0 === symbol || p.symbol_1 === symbol);
            const address = pool.symbol_0 === symbol ? pool.address_0 : pool.address_1;
            
            // Check cache first
            if (logoCache.has(symbol)) {
                logoMap[symbol] = logoCache.get(symbol);
                return;
            }

            const logoPromise = fetchSingleLogo(symbol, address)
                .then(url => {
                    logoMap[symbol] = url;
                    logoCache.set(symbol, url); // Cache the result
                    return { symbol, url };
                })
                .catch(error => {
                    console.warn(`Failed to fetch logo for ${symbol}:`, error);
                    const fallbackUrl = "/j.png";
                    logoMap[symbol] = fallbackUrl;
                    logoCache.set(symbol, fallbackUrl);
                    return { symbol, url: fallbackUrl };
                });

            logoPromises.push(logoPromise);
        });

        // Fetch all logos in parallel
        if (logoPromises.length > 0) {
            await Promise.allSettled(logoPromises);
        }

        return logoMap;
    };

    const fetchSingleLogo = async (symbol, address) => {
        try {
            const canisterId = address.replace("canister://", "");
            const principal = Principal.fromText(canisterId);
            return await mercx_Actor.get_logo_url(principal);
        } catch (error) {
            throw new Error(`Logo fetch failed for ${symbol}: ${error.message}`);
        }
    };

    // Optimized liquidity check
    const checkUserHasLiquidity = (symbol0, symbol1, userLpTokens) => {
        if (!userLpTokens || userLpTokens.length === 0) return false;

        const normalize = (s) => (typeof s === "string" ? s.toUpperCase() : "");
        const possible = [
            `${normalize(symbol0)}_${normalize(symbol1)}`,
            `${normalize(symbol1)}_${normalize(symbol0)}`,
            `${normalize(symbol0)}_${normalize(symbol1)} LP`,
            `${normalize(symbol1)}_${normalize(symbol0)} LP`,
        ];

        const match = userLpTokens.find((t) => {
            if (typeof t === "string") {
                return possible.includes(normalize(t));
            } else if (t && typeof t.symbol === "string") {
                return possible.includes(normalize(t.symbol));
            }
            return false;
        });

        if (!match) return false;

        const amount = typeof match === "string"
            ? 0
            : typeof match.amount === "bigint"
                ? Number(match.amount) / 10 ** 8
                : parseFloat(match.amount) / 10 ** 8;

        return amount > 0;
    };

    const findLpAmount = (symbol0, symbol1) => {
        const normalize = (s) => (typeof s === "string" ? s.toUpperCase() : "");
        const possible = [
            `${normalize(symbol0)}_${normalize(symbol1)}`,
            `${normalize(symbol1)}_${normalize(symbol0)}`,
            `${normalize(symbol0)}_${normalize(symbol1)} LP`,
            `${normalize(symbol1)}_${normalize(symbol0)} LP`,
        ];

        const match = lpTokens.find((t) => {
            if (typeof t === "string") {
                return possible.includes(normalize(t));
            } else if (t && typeof t.symbol === "string") {
                return possible.includes(normalize(t.symbol));
            }
            return false;
        });

        if (!match) return "-";

        const amount = typeof match === "string"
            ? 0
            : typeof match.amount === "bigint"
                ? Number(match.amount) / 10 ** 8
                : parseFloat(match.amount) / 10 ** 8;

        return amount.toFixed(4);
    };

    // Show loading screen with dynamic stage
    if (loading) {
        return (
            <LoadingScreen 
                title="Loading Your Liquidity" 
                subtitle={loadingStage}
                showSkeleton={true}
                skeletonType="table"
            />
        );
    }

    // Filter pools to only show user's liquidity positions
    const userPoolsWithLiquidity = pools.filter((pool) => {
        const amount = findLpAmount(pool.symbol_0, pool.symbol_1);
        return amount !== "-" && parseFloat(amount) > 0;
    });

    // Show empty state if user has no liquidity positions
    if (userPoolsWithLiquidity.length === 0) {
        return (
            <div className="min-h-screen bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
                <div className="max-w-7xl mx-auto">
                    <div className="text-center mb-10">
                        <h2 className="text-3xl font-extrabold text-white sm:text-4xl">
                            Your Liquidity Positions
                        </h2>
                        <p className="mt-3 text-xl text-gray-300">
                            Manage your liquidity positions and earn trading fees
                        </p>
                    </div>

                    <div className="bg-slate-800 rounded-xl shadow-2xl p-12 text-center">
                        <div className="w-24 h-24 mx-auto mb-6 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-full flex items-center justify-center opacity-20">
                            <svg className="w-12 h-12 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                            </svg>
                        </div>
                        <h3 className="text-2xl font-semibold text-white mb-4">No Liquidity Positions</h3>
                        <p className="text-gray-400 mb-8 max-w-md mx-auto">
                            You don't have any liquidity positions yet. Start providing liquidity to earn trading fees and rewards.
                        </p>
                        <div className="space-y-4 sm:space-y-0 sm:space-x-4 sm:flex sm:justify-center">
                            <button
                                className="w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 transition-colors"
                                onClick={() => navigate('/pools')}
                            >
                                Browse Pools
                            </button>
                            <button
                                className="w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 border border-slate-600 text-base font-medium rounded-md text-slate-300 bg-slate-700 hover:bg-slate-600 transition-colors"
                                onClick={() => navigate('/addPool')}
                            >
                                Create New Pool
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
            <div className="max-w-7xl mx-auto">
                <div className="text-center mb-10">
                    <h2 className="text-3xl font-extrabold text-white sm:text-4xl">
                        Your Liquidity Positions
                    </h2>
                    <p className="mt-3 text-xl text-gray-300">
                        Manage your liquidity positions and earn trading fees
                    </p>
                    <div className="mt-2">
                        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-900/20 text-green-400">
                            {userPoolsWithLiquidity.length} Active Position{userPoolsWithLiquidity.length !== 1 ? 's' : ''}
                        </span>
                    </div>
                </div>

                <div className="bg-slate-800 rounded-xl shadow-2xl overflow-hidden">
                    <div className="overflow-x-auto">
                        <table className="min-w-full divide-y divide-slate-700">
                            <thead className="bg-slate-900">
                                <tr>
                                    <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Pool</th>
                                    <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">LP Token</th>
                                    <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Amount</th>
                                    <th className="relative px-6 py-5"><span className="sr-only">Actions</span></th>
                                </tr>
                            </thead>
                            <tbody className="bg-slate-800 divide-y divide-slate-700">
                                {userPoolsWithLiquidity.map((pool) => (
                                    <tr
                                        key={pool.pool_id}
                                        className={`transition-all duration-200 ${hoveredRow === pool.pool_id ? "hover:bg-slate-700" : ""}`}
                                        onMouseEnter={() => setHoveredRow(pool.pool_id)}
                                        onMouseLeave={() => setHoveredRow(null)}
                                    >
                                        <td className="px-6 py-4 whitespace-nowrap">
                                            <div className="flex items-center gap-3">
                                                <div className="flex items-center">
                                                    <img
                                                        src={logos[pool.symbol_0] || "/j.png"}
                                                        alt={pool.symbol_0}
                                                        className="w-8 h-8 rounded-full shadow-md border border-slate-500"
                                                        loading="lazy"
                                                    />
                                                    <img
                                                        src={logos[pool.symbol_1] || "/j.png"}
                                                        alt={pool.symbol_1}
                                                        className="w-8 h-8 rounded-full -ml-2 shadow-md border border-slate-500"
                                                        loading="lazy"
                                                    />
                                                </div>
                                                <span className="text-white font-medium text-sm">
                                                    {pool.symbol_0}/{pool.symbol_1}
                                                </span>
                                            </div>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap">
                                            <span className="text-xs text-indigo-300 border border-indigo-500 px-2 py-0.5 rounded-md">
                                                {`${pool.symbol_0}_${pool.symbol_1} LP`}
                                            </span>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap">
                                            <div className="text-white font-medium">
                                                {findLpAmount(pool.symbol_0, pool.symbol_1)}
                                            </div>
                                            <div className="text-gray-400 text-sm">LP Tokens</div>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                            <div className="flex space-x-2">
                                                <button
                                                    className="text-indigo-400 hover:text-indigo-300 bg-indigo-900/30 hover:bg-indigo-900/50 px-3 py-1 rounded-md text-xs font-medium transition-colors"
                                                    onClick={() => navigate(`/addPool?token0=${pool.symbol_0}&token1=${pool.symbol_1}`)}
                                                >
                                                    Add
                                                </button>
                                                <button 
                                                    className="text-white hover:text-gray-200 bg-indigo-600 hover:bg-indigo-700 px-3 py-1 rounded-md text-xs font-medium transition-colors"
                                                    onClick={() => navigate(`/swap?from=${pool.symbol_0}&to=${pool.symbol_1}`)}
                                                >
                                                    Trade
                                                </button>
                                                <button
                                                    className="text-red-400 hover:text-red-300 bg-red-900/30 hover:bg-red-900/50 px-3 py-1 rounded-md text-xs font-medium transition-colors"
                                                    onClick={() => navigate(`/removeLiquidity?token0=${pool.symbol_0}&token1=${pool.symbol_1}`)}
                                                >
                                                    Remove
                                                </button>
                                            </div>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    );
}