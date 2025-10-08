import React, { useState, useEffect } from 'react';
import { useAuth } from '../use-auth-client';

const ProtocolStats = () => {
  const { mercx_Actor } = useAuth();
  const [stats, setStats] = useState(null);
  const [loading, setLoading] = useState(true);
  const [refreshing, setRefreshing] = useState(false);
  const [error, setError] = useState('');

  // Format USD values for display
  const formatUSD = (value) => {
    if (value === 0) return '$0';
    if (value < 1000) return `$${value.toFixed(2)}`;
    if (value < 1000000) return `$${(value / 1000).toFixed(1)}K`;
    return `$${(value / 1000000).toFixed(1)}M`;
  };

  // Format number with commas
  const formatNumber = (value) => {
    return new Intl.NumberFormat().format(value);
  };

  // Load cached stats from localStorage immediately
  useEffect(() => {
    const cachedStats = localStorage.getItem('protocol_stats');
    if (cachedStats) {
      try {
        setStats(JSON.parse(cachedStats));
        setLoading(false); // Show cached data immediately
      } catch (e) {
        console.error('Error parsing cached stats:', e);
      }
    }
  }, []);

  // Fetch fresh data
  const fetchProtocolStats = async (isRefresh = false) => {
    if (!mercx_Actor) return;

    if (isRefresh) {
      setRefreshing(true);
    } else {
      setLoading(true);
    }
    setError('');

    try {
      const result = await mercx_Actor.get_protocol_stats();
      
      if (result) {
        setStats(result);
        // Cache the result
        localStorage.setItem('protocol_stats', JSON.stringify(result));
        localStorage.setItem('protocol_stats_timestamp', Date.now().toString());
      } else {
        setError('Failed to load protocol statistics');
      }
    } catch (err) {
      console.error('Error fetching protocol stats:', err);
      setError('Failed to load protocol statistics');
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  };

  useEffect(() => {
    if (!mercx_Actor) return;
    
    // Check if cached data is fresh (< 5 minutes old)
    const cachedTimestamp = localStorage.getItem('protocol_stats_timestamp');
    const isFresh = cachedTimestamp && (Date.now() - parseInt(cachedTimestamp)) < 5 * 60 * 1000;
    
    if (!isFresh) {
      fetchProtocolStats();
    }
  }, [mercx_Actor]);

  // Show loading skeleton only if no cached data
  if (loading && !stats) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-6 gap-4 mb-8">
        {[1, 2, 3, 4, 5, 6].map((i) => (
          <div key={i} className="bg-slate-800 rounded-xl p-6 animate-pulse">
            <div className="h-4 bg-slate-700 rounded w-20 mb-3"></div>
            <div className="h-8 bg-slate-700 rounded w-32 mb-2"></div>
            <div className="h-3 bg-slate-700 rounded w-24"></div>
          </div>
        ))}
      </div>
    );
  }

  if (error && !stats) {
    return (
      <div className="bg-slate-800 rounded-xl p-6 mb-8 text-center">
        <p className="text-red-400">{error || 'No protocol data available'}</p>
        <button 
          onClick={() => fetchProtocolStats(false)}
          className="mt-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm"
        >
          Retry
        </button>
      </div>
    );
  }

  if (!stats) {
    return null;
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-6 gap-4 mb-8">
      {/* Show refreshing indicator */}
      {refreshing && (
        <div className="md:col-span-3 lg:col-span-6 flex justify-center mb-2">
          <div className="bg-blue-500/20 text-blue-400 px-4 py-2 rounded-lg text-sm flex items-center gap-2">
            <svg className="w-4 h-4 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            Updating stats...
          </div>
        </div>
      )}

      {/* Total TVL */}
      <div className="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl p-6 border border-slate-700 hover:border-slate-600 transition-colors">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-400 uppercase tracking-wide">Total TVL</p>
            <p className="text-2xl font-bold text-white mt-1">{formatUSD(stats.total_tvl_usd)}</p>
            <p className="text-xs text-gray-500 mt-1">{stats.active_pools} Active Pools</p>
          </div>
          <div className="p-2 bg-blue-500/20 rounded-lg">
            <svg className="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
            </svg>
          </div>
        </div>
      </div>

      {/* 24h Volume */}
      <div className="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl p-6 border border-slate-700 hover:border-slate-600 transition-colors">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-400 uppercase tracking-wide">24h Volume</p>
            <p className="text-2xl font-bold text-white mt-1">{formatUSD(stats.total_volume_24h_usd)}</p>
            <p className="text-xs text-gray-500 mt-1">{formatNumber(stats.total_transactions_24h)} Transactions</p>
          </div>
          <div className="p-2 bg-green-500/20 rounded-lg">
            <svg className="w-6 h-6 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
            </svg>
          </div>
        </div>
      </div>

      {/* 24h Fees */}
      <div className="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl p-6 border border-slate-700 hover:border-slate-600 transition-colors">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-400 uppercase tracking-wide">24h Fees</p>
            <p className="text-2xl font-bold text-white mt-1">{formatUSD(stats.total_fees_24h_usd)}</p>
            <p className="text-xs text-gray-500 mt-1">Trading Fees</p>
          </div>
          <div className="p-2 bg-purple-500/20 rounded-lg">
            <svg className="w-6 h-6 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2m2 4h10a2 2 0 002-2v-6a2 2 0 00-2-2H9a2 2 0 00-2 2v6a2 2 0 002 2zm7-5a2 2 0 11-4 0 2 2 0 014 0z" />
            </svg>
          </div>
        </div>
      </div>

      {/* 7d Volume */}
      <div className="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl p-6 border border-slate-700 hover:border-slate-600 transition-colors">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-400 uppercase tracking-wide">7d Volume</p>
            <p className="text-2xl font-bold text-white mt-1">{formatUSD(stats.total_volume_7d_usd)}</p>
            <p className="text-xs text-gray-500 mt-1">{formatNumber(stats.total_transactions_7d)} Transactions</p>
          </div>
          <div className="p-2 bg-indigo-500/20 rounded-lg">
            <svg className="w-6 h-6 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
          </div>
        </div>
      </div>

      {/* 7d Fees */}
      <div className="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl p-6 border border-slate-700 hover:border-slate-600 transition-colors">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-400 uppercase tracking-wide">7d Fees</p>
            <p className="text-2xl font-bold text-white mt-1">{formatUSD(stats.total_fees_7d_usd)}</p>
            <p className="text-xs text-gray-500 mt-1">Weekly Fees</p>
          </div>
          <div className="p-2 bg-orange-500/20 rounded-lg">
            <svg className="w-6 h-6 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
            </svg>
          </div>
        </div>
      </div>

      {/* Total Pools */}
      <div className="bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl p-6 border border-slate-700 hover:border-slate-600 transition-colors">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-400 uppercase tracking-wide">Total Pools</p>
            <p className="text-2xl font-bold text-white mt-1">{formatNumber(stats.total_pools)}</p>
            <p className="text-xs text-gray-500 mt-1">Liquidity Pools</p>
          </div>
          <div className="p-2 bg-cyan-500/20 rounded-lg">
            <svg className="w-6 h-6 text-cyan-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
            </svg>
          </div>
        </div>
      </div>

      {/* Refresh Button */}
      <div className="md:col-span-3 lg:col-span-6 flex justify-center mt-4">
        <button
          onClick={() => fetchProtocolStats(true)}
          disabled={refreshing || loading}
          className="px-4 py-2 bg-slate-700 hover:bg-slate-600 disabled:bg-slate-800 disabled:cursor-not-allowed text-white rounded-lg text-sm font-medium transition-colors flex items-center gap-2"
        >
          <svg className={`w-4 h-4 ${refreshing ? 'animate-spin' : ''}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          {refreshing ? 'Refreshing...' : 'Refresh Stats'}
        </button>
      </div>
    </div>
  );
};

export default ProtocolStats;