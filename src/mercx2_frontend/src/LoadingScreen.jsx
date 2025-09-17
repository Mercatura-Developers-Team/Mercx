import React from 'react';

const LoadingScreen = ({ 
  title = "Loading", 
  subtitle = "Please wait...", 
  showSkeleton = false,
  skeletonType = "table" // "table", "cards", "list"
}) => {
  const renderTableSkeleton = () => (
    <div className="w-full max-w-6xl px-4">
      <div className="bg-slate-800 rounded-xl p-6 shadow-2xl">
        <div className="space-y-4">
          {/* Table header skeleton */}
          <div className="grid grid-cols-7 gap-4 pb-4 border-b border-slate-700">
            {['Pool', 'Price', 'TVL', 'Volume (24h)', 'Fee Tier', 'APY', 'Actions'].map((header, i) => (
              <div key={i} className="h-4 bg-slate-700 rounded animate-pulse"></div>
            ))}
          </div>
          
          {/* Table rows skeleton */}
          {[...Array(5)].map((_, i) => (
            <div key={i} className="grid grid-cols-7 gap-4 py-3" style={{animationDelay: `${i * 0.1}s`}}>
              {/* Pool column with token logos */}
              <div className="flex items-center space-x-3">
                <div className="flex -space-x-2">
                  <div className="w-8 h-8 bg-gradient-to-r from-slate-600 to-slate-700 rounded-full animate-pulse"></div>
                  <div className="w-8 h-8 bg-gradient-to-r from-slate-700 to-slate-600 rounded-full animate-pulse"></div>
                </div>
                <div className="h-4 w-20 bg-slate-700 rounded animate-pulse"></div>
              </div>
              
              {/* Other columns */}
              <div className="h-4 bg-slate-700 rounded animate-pulse"></div>
              <div className="h-4 bg-slate-700 rounded animate-pulse"></div>
              <div className="h-4 bg-slate-700 rounded animate-pulse"></div>
              <div className="h-4 bg-slate-700 rounded animate-pulse"></div>
              <div className="h-4 bg-slate-700 rounded animate-pulse"></div>
              
              {/* Actions column */}
              <div className="flex space-x-2">
                <div className="h-6 w-16 bg-slate-700 rounded animate-pulse"></div>
                <div className="h-6 w-12 bg-slate-700 rounded animate-pulse"></div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );

  const renderCardsSkeleton = () => (
    <div className="w-full max-w-6xl px-4">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {[...Array(6)].map((_, i) => (
          <div key={i} className="bg-slate-800 rounded-xl p-6 shadow-lg" style={{animationDelay: `${i * 0.1}s`}}>
            <div className="space-y-4">
              <div className="flex items-center space-x-3">
                <div className="w-12 h-12 bg-slate-700 rounded-full animate-pulse"></div>
                <div className="space-y-2 flex-1">
                  <div className="h-4 bg-slate-700 rounded animate-pulse"></div>
                  <div className="h-3 bg-slate-700 rounded w-3/4 animate-pulse"></div>
                </div>
              </div>
              <div className="space-y-3">
                <div className="h-3 bg-slate-700 rounded animate-pulse"></div>
                <div className="h-3 bg-slate-700 rounded w-5/6 animate-pulse"></div>
                <div className="h-8 bg-slate-700 rounded animate-pulse"></div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );

  const renderListSkeleton = () => (
    <div className="w-full max-w-4xl px-4">
      <div className="bg-slate-800 rounded-xl p-6 shadow-2xl">
        <div className="space-y-4">
          {[...Array(8)].map((_, i) => (
            <div key={i} className="flex items-center space-x-4 p-4 bg-slate-700/50 rounded-lg" style={{animationDelay: `${i * 0.05}s`}}>
              <div className="w-10 h-10 bg-slate-600 rounded-full animate-pulse"></div>
              <div className="flex-1 space-y-2">
                <div className="h-4 bg-slate-600 rounded animate-pulse"></div>
                <div className="h-3 bg-slate-600 rounded w-3/4 animate-pulse"></div>
              </div>
              <div className="w-16 h-8 bg-slate-600 rounded animate-pulse"></div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );

  const getSkeleton = () => {
    switch (skeletonType) {
      case "cards":
        return renderCardsSkeleton();
      case "list":
        return renderListSkeleton();
      case "table":
      default:
        return renderTableSkeleton();
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 flex flex-col items-center justify-center">
      {/* Animated logo/icon */}
      <div className="relative mb-8">
        <div className="w-16 h-16 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-full animate-pulse"></div>
        <div className="absolute inset-0 w-16 h-16 bg-gradient-to-r from-purple-600 to-indigo-500 rounded-full animate-ping opacity-20"></div>
        <div className="absolute inset-2 w-12 h-12 bg-white rounded-full opacity-20 animate-bounce"></div>
      </div>

      {/* Loading text */}
      <div className="text-white text-2xl font-semibold mb-2 animate-pulse">
        {title}
      </div>

      {/* Animated dots */}
      <div className="flex space-x-2 mb-8">
        <div className="w-3 h-3 bg-indigo-400 rounded-full animate-bounce" style={{animationDelay: '0s'}}></div>
        <div className="w-3 h-3 bg-purple-400 rounded-full animate-bounce" style={{animationDelay: '0.1s'}}></div>
        <div className="w-3 h-3 bg-blue-400 rounded-full animate-bounce" style={{animationDelay: '0.2s'}}></div>
      </div>

      {/* Skeleton content */}
      {showSkeleton && getSkeleton()}

      {/* Loading progress bar */}
      <div className="w-full max-w-md mt-8">
        <div className="bg-slate-800 rounded-full h-2 overflow-hidden">
          <div className="bg-gradient-to-r from-indigo-500 to-purple-600 h-full rounded-full animate-pulse w-3/4 transition-all duration-1000"></div>
        </div>
        <p className="text-slate-400 text-sm text-center mt-2">{subtitle}</p>
      </div>
    </div>
  );
};

export default LoadingScreen;