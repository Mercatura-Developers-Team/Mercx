import React, { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { useAuth } from '../use-auth-client';

const PoolAnalyticsCharts = ({ poolId, token0, token1 }) => {
  const { mercx_Actor } = useAuth();
  const [chartData, setChartData] = useState([]);
  const [selectedMetric, setSelectedMetric] = useState('tvl'); // 'tvl' or 'volume'
  const [timeFrame, setTimeFrame] = useState(24); // hours
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [snapshotStatus, setSnapshotStatus] = useState('');
 
  // Format timestamp to readable date
  const formatTimestamp = (timestamp) => {
    const date = new Date(Number(timestamp) / 1_000_000); // Convert from nanoseconds to milliseconds
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  // Format USD values for display
  const formatUSD = (value) => {
    if (value === 0) return '$0';
    if (value < 1000) return `$${value.toFixed(2)}`;
    if (value < 1000000) return `$${(value / 1000).toFixed(1)}K`;
    return `$${(value / 1000000).toFixed(1)}M`;
  };

  

  // Fetch chart data from backend
  const fetchChartData = async () => {
    if (!mercx_Actor || !poolId) return;
    
    setLoading(true);
    setError('');
    
    try {
      // Call the backend function to get pool chart data
      const rawData = await mercx_Actor.get_pool_chart_data(poolId, timeFrame);
      
      if (rawData && rawData.length > 0) {
        // Transform the data: (timestamp, tvl_usd, volume_24h_usd)
        const formattedData = rawData.map(([timestamp, tvl_usd, volume_24h_usd]) => ({
          timestamp: Number(timestamp),
          tvl_usd: Number(tvl_usd),
          volume_24h_usd: Number(volume_24h_usd),
          formattedTime: formatTimestamp(timestamp),
          displayValue: selectedMetric === 'tvl' ? Number(tvl_usd) : Number(volume_24h_usd)
        }));
        
        // Sort by timestamp
        formattedData.sort((a, b) => a.timestamp - b.timestamp);
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

  // // Record snapshots for all pools
  // const recordAllPoolsSnapshot = async () => {
  //   if (!mercx_Actor) return;
    
  //   setSnapshotStatus('Recording snapshots...');
  //   try {
  //     const result = await mercx_Actor.record_all_pools_snapshot();
  //     setSnapshotStatus("");
      
  //     // Refresh the chart data after recording snapshots
  //     setTimeout(() => {
  //       fetchChartData();
  //     }, 1000);
  //   } catch (err) {
  //     console.error('Failed to record snapshots:', err);
  //     setSnapshotStatus('Error recording snapshots');
  //   }
  // };

  // Record snapshot for this specific pool
const recordPoolSnapshot = async () => {
  if (!mercx_Actor || !poolId) return;
  
  setSnapshotStatus('Recording snapshot...');
  setLoading(true);
  
  try {
    // First get current pool metrics to record accurate data
    const metricsResult = await mercx_Actor.get_pool_metrics(poolId);
    
    if (metricsResult?.Ok) {
      const metrics = metricsResult.Ok;
      const tvl_usd = metrics.tvl.tvl_usd;
      const volume_24h_usd = metrics.volume.volume_24h_usd;
      
      // Record snapshot for this specific pool
      const result = await mercx_Actor.record_pool_snapshot(poolId, tvl_usd, volume_24h_usd);
      
      console.log('Snapshot recording result:', result);
      setSnapshotStatus("Snapshot recorded successfully");
      
      // Refresh the chart data after recording snapshot
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

  // Fetch data when component mounts or parameters change
  useEffect(() => {
    fetchChartData();
  }, [mercx_Actor, poolId, timeFrame]);

  // Update display values when metric changes
  useEffect(() => {
    if (chartData.length > 0) {
      const updatedData = chartData.map(item => ({
        ...item,
        displayValue: selectedMetric === 'tvl' ? item.tvl_usd : item.volume_24h_usd
      }));
      setChartData(updatedData);
    }
  }, [selectedMetric]);

  const poolName = token0 && token1 ? `${token0.symbol}/${token1.symbol}` : 'Pool';

  return (
    <div className="w-full bg-[#1a1a2e] rounded-xl p-6 space-y-4">
      {/* Header */}
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <h3 className="text-white text-lg font-semibold">
          {poolName} Analytics
        </h3>
        
        {/* Controls */}
        <div className="flex flex-col sm:flex-row gap-2">
          {/* Metric Selector */}
          <select
            value={selectedMetric}
            onChange={(e) => setSelectedMetric(e.target.value)}
            className="px-3 py-2 bg-gray-700 text-white rounded-lg text-sm border-0"
          >
            <option value="tvl">Total Value Locked (TVL)</option>
            <option value="volume">24h Volume</option>
          </select>
          
          {/* Time Frame Selector */}
          <select
            value={timeFrame}
            onChange={(e) => setTimeFrame(parseInt(e.target.value))}
            className="px-3 py-2 bg-gray-700 text-white rounded-lg text-sm border-0"
          >
            <option value={24}>24 Hours</option>
            <option value={168}>7 Days</option>
            <option value={720}>30 Days</option>
          </select>
          
          {/* Refresh Button */}
          <button
            onClick={recordPoolSnapshot}
            disabled={loading}
            className="px-3 py-2 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-600 text-white rounded-lg text-sm"
          >
            {loading ? 'Loading...' : 'Refresh'}
          </button>
        </div>
      </div>

      {/* Snapshot Status */}
      {snapshotStatus && (
        <div className={`p-2 rounded text-sm text-center ${
          snapshotStatus.includes('Error')
            ? 'bg-red-900 text-red-300'
            : 'bg-blue-900 text-blue-300'
        }`}>
          {snapshotStatus}
        </div>
      )}

      {/* Chart Container */}
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
              <p className="text-sm">Chart data will appear after some trading activity</p>
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
                interval="preserveStartEnd"
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
                formatter={(value) => [formatUSD(value), selectedMetric === 'tvl' ? 'TVL' : '24h Volume']}
                labelStyle={{ color: '#F3F4F6' }}
              />
              <Legend
                wrapperStyle={{ color: '#F3F4F6' }}
              />
              <Line
                type="monotone"
                dataKey="displayValue"
                stroke={selectedMetric === 'tvl' ? '#3B82F6' : '#10B981'}
                strokeWidth={2}
                dot={{ fill: selectedMetric === 'tvl' ? '#3B82F6' : '#10B981', strokeWidth: 2, r: 4 }}
                activeDot={{ r: 6, stroke: selectedMetric === 'tvl' ? '#3B82F6' : '#10B981', strokeWidth: 2 }}
                name={selectedMetric === 'tvl' ? 'TVL' : '24h Volume'}
              />
            </LineChart>
          </ResponsiveContainer>
        )}
      </div>

      {/* Chart Summary */}
      {chartData.length > 0 && (
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-4 border-t border-gray-700">
          <div className="text-center">
            <p className="text-gray-400 text-sm">Current</p>
            <p className="text-white font-semibold">
              {formatUSD(chartData[chartData.length - 1]?.displayValue || 0)}
            </p>
          </div>
          <div className="text-center">
            <p className="text-gray-400 text-sm">Highest</p>
            <p className="text-white font-semibold">
              {formatUSD(Math.max(...chartData.map(d => d.displayValue)))}
            </p>
          </div>
          <div className="text-center">
            <p className="text-gray-400 text-sm">Lowest</p>
            <p className="text-white font-semibold">
              {formatUSD(Math.min(...chartData.map(d => d.displayValue)))}
            </p>
          </div>
        </div>
      )}
    </div>
  );
};

export default PoolAnalyticsCharts;