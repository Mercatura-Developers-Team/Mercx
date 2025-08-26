import React, { useEffect, useState } from "react";
import { useAuth } from "../use-auth-client";
import { Principal } from "@dfinity/principal";
import { useNavigate } from 'react-router-dom';


export default function TableLiquidity() {

    const { mercx_Actor, principal } = useAuth();
    const [pools, setPools] = useState([]);
    const [hoveredRow, setHoveredRow] = useState(null);
    const [logos, setLogos] = useState({});
    const [prices, setPrices] = useState({});
    const [lpTokens, setLpTokens] = useState([]);

    const navigate = useNavigate();

    useEffect(() => {
        const fetchUserLpTokens = async () => {
            if (!mercx_Actor) return;
            try {
                if (!principal) return;
                const tokens = await mercx_Actor.get_lp_tokens_by_principal(principal);
                setLpTokens(tokens);
            } catch (e) {
                console.error("Failed to fetch user LP tokens", e);
            }
        };

        fetchUserLpTokens();
    }, [mercx_Actor, principal]);
    useEffect(() => {
        const loadPools = async () => {
            if (!mercx_Actor) return;

            try {
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
            }
        };

        loadPools();
    }, [mercx_Actor]);


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




    // useEffect(() => {
    //     const fetchPrices = async () => {
    //         if (!mercx_Actor || !pools.length) return;

    //         try {
    //             const priceEntries = await Promise.all(
    //                 pools.map(async (pool) => {
    //                     try {
    //                         const token0 = pool.symbol_0;
    //                         const token1 = pool.symbol_1;
    //                         const key = `${token0}/${token1}`; // ✅ define key here
    //                         const result = await mercx_Actor.get_pool_price(token0, token1);
    //                         console.log(result);
    //                         if (result && result.Ok !== undefined) {
    //                             return [key, result.Ok];
    //                         } else if (result && result.Err) {
    //                             console.warn(`Price fetch error for  ${result.Err}`);
    //                         }
    //                     } catch (err) {
    //                         console.warn(`Failed to get price for ${pool.symbol_0}/${pool.symbol_1}`, err);
    //                         return null;
    //                     }
    //                 })
    //             );

    //             const filtered = priceEntries.filter(Boolean);
    //             const priceMap = Object.fromEntries(filtered);
    //             setPrices(priceMap);
    //         } catch (error) {
    //             console.error("Error fetching pool prices", error);
    //         }
    //     };
    //     fetchPrices();
    // }, [mercx_Actor, pools]);


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
                                    <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">LP Token</th>
                                    <th className="px-6 py-5 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Amount</th>
                                    <th className="relative px-6 py-5"><span className="sr-only">Actions</span></th>
                                </tr>
                            </thead>
                            <tbody className="bg-slate-800 divide-y divide-slate-700">
                                {pools
                                    .filter((pool) => {
                                        const amount = findLpAmount(pool.symbol_0, pool.symbol_1);
                                        return amount !== "-" && parseFloat(amount) > 0;
                                    })
                                    .map((pool) => (

                                        <tr
                                            key={pool.pool_id}
                                            className={`transition-all duration-200 ${hoveredRow === pool.pool_id ? "hover:bg-slate-700 scale-[1]" : "hover:bg-slate-750"}`}
                                            onMouseEnter={() => setHoveredRow(pool.pool_id)}
                                            onMouseLeave={() => setHoveredRow(null)}
                                        >
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <div className="flex items-center gap-3 hover:scale-105 transition-transform">
                                                    <div className="flex items-center">
                                                        <img
                                                            src={logos[pool.symbol_0] || "/j.png"}
                                                            alt={pool.symbol_0}
                                                            className="w-8 h-8 rounded-full shadow-md border border-slate-500"
                                                        />
                                                        <img
                                                            src={logos[pool.symbol_1] || "/j.png"}
                                                            alt={pool.symbol_1}
                                                            className="w-8 h-8 rounded-full -ml-2 shadow-md border border-slate-500"
                                                        />
                                                    </div>
                                                    <span className="text-white font-medium text-sm">
                                                        {pool.symbol_0}/{pool.symbol_1}
                                                    </span>
                                                </div>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-white font-medium">
                                                <span className="text-xs text-indigo-300 border border-indigo-500 px-2 py-0.5 rounded-md">
                                                    {`${pool.symbol_0}_${pool.symbol_1} LP`}
                                                </span>

                                                {/* <span className="bg-indigo-600/20 text-indigo-300 text-xs font-medium px-2 py-1 rounded-lg shadow-sm">
  {`${pool.symbol_0}_${pool.symbol_1} LP`}
</span> */}


                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-white">
                                                {findLpAmount(pool.symbol_0, pool.symbol_1) || "Not Found"}
                                            </td>

                                            <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                                <div className="flex space-x-2">
                                                    <button
                                                        className="text-indigo-400 hover:text-indigo-300 bg-indigo-900/30 hover:bg-indigo-900/50 px-3 py-1 rounded-md text-xs font-medium transition-colors"
                                                        onClick={() => navigate(`/addPool?token0=${pool.symbol_0}&token1=${pool.symbol_1}`)}
                                                    >
                                                        Add Liquidity
                                                    </button>
                                                    <button className="text-white hover:text-gray-200 bg-indigo-600 hover:bg-indigo-700 px-3 py-1 rounded-md text-xs font-medium transition-colors">
                                                        Trade
                                                    </button>
                                                    <button
                                                        className="text-indigo-400 hover:text-indigo-300 bg-indigo-900/30 hover:bg-indigo-900/50 px-3 py-1 rounded-md text-xs font-medium transition-colors"
                                                        onClick={() =>
                                                            navigate(`/removeLiquidity?token0=${pool.symbol_0}&token1=${pool.symbol_1}`)
                                                        }
                                                    >
                                                        Remove Liquidity
                                                    </button>
                                                </div>
                                            </td>
                                        </tr>
                                    ))}
                            </tbody>
                        </table>
                    </div>
                </div>

                {/* <div className="mt-8 text-center">
                    <button
                        className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                        onClick={() => navigate('/addPool')}
                    >
                        <svg className="-ml-1 mr-2 h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clipRule="evenodd" />
                        </svg>
                        Create New Pool
                    </button>
                </div> */}
            </div>
        </div>
    );
}