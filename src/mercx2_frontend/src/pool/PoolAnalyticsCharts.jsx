import React, { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { useAuth } from '../use-auth-client';

const PoolAnalyticsCharts = ({ poolId, token0, token1, poolBalance0, poolBalance1 }) => {
  const { mercx_Actor } = useAuth();
  const [chartData, setChartData] = useState([]);
  const [selectedMetric, setSelectedMetric] = useState('tvl');
  const [timeFrame, setTimeFrame] = useState(7);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [snapshotStatus, setSnapshotStatus] = useState('');
 
  // Format timestamp to readable date
  const formatTimestamp = (timestamp) => {
    const date = new Date(Number(timestamp) / 1_000_000);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };

  // Format USD values for display
  const formatUSD = (value) => {
    if (value === 0) return '$0';
    if (value < 1000) return `$${value.toFixed(2)}`;
    if (value < 1000000) return `$${(value / 1000).toFixed(1)}K`;
    return `$${(value / 1000000).toFixed(1)}M`;
  };

  // Fetch chart data from backend using daily chart function
  const fetchChartData = async () => {
    if (!mercx_Actor || !poolId) return;
    
    setLoading(true);
    setError('');
    
    try {
      const rawData = await mercx_Actor.get_pool_daily_chart(poolId, BigInt(timeFrame));
      
      if (rawData && rawData.length > 0) {
        const formattedData = rawData.map(([timestamp, day_number, tvl_usd, volume_24h_usd]) => ({
          timestamp: Number(timestamp),
          day_number: Number(day_number),
          tvl_usd: Number(tvl_usd),
          volume_24h_usd: Number(volume_24h_usd),
          formattedTime: formatTimestamp(timestamp)
        }));
        
        setChartData(formattedData);
      } else {
        setChartData([]);
        setError('No chart data available for this time period');
      }
    } catch (err) {
      console.error('Failed to fetch chart data:', err);
      setError('Failed to load chart data');
      setChartData([]);
    } finally {
      setLoading(false);
    }
  };

  // Record snapshot for this specific pool
  const recordPoolSnapshot = async () => {
    if (!mercx_Actor || !poolId) return;
    
    setSnapshotStatus('Recording snapshot...');
    setLoading(true);
    
    try {
      const metricsResult = await mercx_Actor.get_pool_metrics(poolId);
      
      if (metricsResult?.Ok) {
        const metrics = metricsResult.Ok;
        const tvl_usd = metrics.tvl.tvl_usd;
        const volume_24h_usd = metrics.volume.volume_24h_usd;
        
        const result = await mercx_Actor.record_pool_snapshot(poolId, tvl_usd, volume_24h_usd);
        
        console.log('Snapshot recording result:', result);
        setSnapshotStatus("Snapshot recorded successfully");
        
        setTimeout(() => {
          fetchChartData();
          setSnapshotStatus('');
        }, 1500);
        
      } else {
        throw new Error('Failed to get current pool metrics');
      }
    } catch (err) {
      console.error('Failed to record snapshot:', err);
      setSnapshotStatus('Error recording snapshot');
      setTimeout(() => setSnapshotStatus(''), 3000);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchChartData();
  }, [mercx_Actor, poolId, timeFrame, poolBalance0, poolBalance1]);

  const poolName = token0 && token1 ? `${token0.symbol}/${token1.symbol}` : 'Pool';
  
  // Get the current data key based on selected metric
  const currentDataKey = selectedMetric === 'tvl' ? 'tvl_usd' : 'volume_24h_usd';
  const currentColor = selectedMetric === 'tvl' ? '#3B82F6' : '#10B981';
  const currentLabel = selectedMetric === 'tvl' ? 'TVL' : '24h Volume';

  return (
    <div className="w-full bg-[#1a1a2e] rounded-xl p-6 space-y-4">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <h3 className="text-white text-lg font-semibold">
          {poolName} Analytics
        </h3>
        
        <div className="flex flex-col sm:flex-row gap-2">
          <select
            value={selectedMetric}
            onChange={(e) => setSelectedMetric(e.target.value)}
            className="px-3 py-2 bg-gray-700 text-white rounded-lg text-sm border-0"
          >
            <option value="tvl">Total Value Locked (TVL)</option>
            <option value="volume">24h Volume</option>
          </select>
          
          <select
            value={timeFrame}
            onChange={(e) => setTimeFrame(parseInt(e.target.value))}
            className="px-3 py-2 bg-gray-700 text-white rounded-lg text-sm border-0"
          >
            <option value={7}>7 Days</option>
            <option value={14}>14 Days</option>
            <option value={30}>30 Days</option>
          </select>
          
          <button
            onClick={recordPoolSnapshot}
            disabled={loading}
            className="px-3 py-2 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-600 text-white rounded-lg text-sm"
          >
            {loading ? 'Loading...' : 'Refresh'}
          </button>
        </div>
      </div>

      {snapshotStatus && (
        <div className={`p-2 rounded text-sm text-center ${
          snapshotStatus.includes('Error')
            ? 'bg-red-900 text-red-300'
            : 'bg-blue-900 text-blue-300'
        }`}>
          {snapshotStatus}
        </div>
      )}

      <div className="h-80 w-full">
        {loading ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-gray-400">Loading chart data...</div>
          </div>
        ) : error ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-red-400 text-center">
              <p>{error}</p>
              <button
                onClick={fetchChartData}
                className="mt-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm"
              >
                Retry
              </button>
            </div>
          </div>
        ) : chartData.length === 0 ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-gray-400 text-center">
              <p>No data available</p>
              <p className="text-sm">Record snapshots daily to see trends</p>
            </div>
          </div>
        ) : (
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={chartData} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
              <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
              <XAxis
                dataKey="formattedTime"
                stroke="#9CA3AF"
                fontSize={12}
                tick={{ fill: '#9CA3AF' }}
              />
              <YAxis
                stroke="#9CA3AF"
                fontSize={12}
                tick={{ fill: '#9CA3AF' }}
                tickFormatter={formatUSD}
              />
              <Tooltip
                contentStyle={{
                  backgroundColor: '#1F2937',
                  border: '1px solid #374151',
                  borderRadius: '8px',
                  color: '#F3F4F6'
                }}
                formatter={(value) => [formatUSD(value), currentLabel]}
                labelStyle={{ color: '#F3F4F6' }}
              />
              <Legend wrapperStyle={{ color: '#F3F4F6' }} />
              <Line
                type="monotone"
                dataKey={currentDataKey}
                stroke={currentColor}
                strokeWidth={2}
                dot={{ fill: currentColor, strokeWidth: 2, r: 4 }}
                activeDot={{ r: 6 }}
                name={currentLabel}
              />
            </LineChart>
          </ResponsiveContainer>
        )}
      </div>

      {chartData.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-4 border-t border-gray-700">
          <div className="text-center">
            <p className="text-gray-400 text-sm">Current</p>
            <p className="text-white font-semibold">
              {formatUSD(chartData[chartData.length - 1]?.[currentDataKey] || 0)}
            </p>
          </div>
          <div className="text-center">
            <p className="text-gray-400 text-sm">Highest</p>
            <p className="text-white font-semibold">
              {formatUSD(Math.max(...chartData.map(d => d[currentDataKey])))}
            </p>
          </div>
          <div className="text-center">
            <p className="text-gray-400 text-sm">Lowest</p>
            <p className="text-white font-semibold">
              {formatUSD(Math.min(...chartData.map(d => d[currentDataKey])))}
            </p>
          </div>
        </div>
      )}
    </div>
  );
};

export default PoolAnalyticsCharts;