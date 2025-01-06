import React, { useState, useEffect } from 'react';
import { useAuth } from '.././use-auth-client';
import Sell from '../Actions/Sell/Sell';
import Buy from './Buy/Buy';


const ActionsOptions = () => {

    const [currentTab, setCurrentTab] = useState('Buy'); // Manage the current tab

    const handleTabClick = (tabName) => {
        setCurrentTab(tabName);
    };

    return (
        <div className="min-h-screen bg-gray-900">
            <main>
                <div className="max-w-xl mx-auto sm:px-6 lg:px-8 pt-8 lg:pt-14 2xl:pt-18">
                    <div className="shadow-xl rounded-3xl h-auto border-t-[1px] border-slate-800 bg-slate-800">
                        <div className="flex justify-around p-3 ">
                            <button
                                onClick={() => handleTabClick('Buy')}
                                className={`text-lg ${currentTab === 'Buy' ? 'text-white' : 'text-gray-400'} font-bold`}
                            >
                                Buy
                            </button>
                            <button
                                onClick={() => handleTabClick('Sell')}
                                className={`text-lg ${currentTab === 'Sell' ? 'text-white' : 'text-gray-400'} font-bold`}
                            >
                                Sell
                            </button>
                            <button
                                onClick={() => handleTabClick('Swap')}
                                className={`text-lg ${currentTab === 'Swap' ? 'text-white ' : 'text-gray-400'} font-bold`}
                            >
                                Swap
                            </button>
                        </div>
                        {/* Conditionally render the UI based on the selected tab */}
                        {currentTab === 'Buy' && (
                            <div>
                                {/* Buy tab content */}
                                <Buy/>
                            </div>
                        )}
                        {currentTab === 'Sell' && (
                            <Sell/>
                              
                        )}
                        {currentTab === 'Swap' && (
                           <div class="p-9 h-[480px] flex flex-col items-center justify-center">
                           <h1 class="text-5xl animate-pulse text-white font-bold mb-8">
                               Coming Soon
                           </h1>
                        
                       </div>
                        )}
                        {/* Other components and logic */}
                    </div>
                </div>
            </main>
        </div>
    );
};

export default ActionsOptions;
