import React, { useState, useEffect } from "react";
import { useAuth } from "./use-auth-client";
import './index.css';
import { HiOutlineLogout, HiMenu, HiX } from "react-icons/hi";
import { NavLink } from 'react-router-dom';

const navigation = [
  { name: "Home", to: "/" },
  { name: "Swap", to: "/swap" },
  { name: "Transactions", to: "/transactions" },
];

function MyNavbar() {
  const { isAuthenticated, login, logout, principal } = useAuth();
  const [principals, setPrincipal] = useState("");
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    if (isAuthenticated) {
      try {
        setPrincipal(principal);
      } catch (error) {
        console.error("Failed to fetch principal:", error);
        setPrincipal("Error fetching principal");
      }
    }
  }, [isAuthenticated, principal]);

  return (
    <nav className="bg-gray-900">
      <div className="max-w-screen-xl mx-auto px-2 sm:px-6 lg:px-8">
        <div className="relative flex items-center justify-between h-16">
          {/* Mobile menu button*/}
          <div className="absolute inset-y-0 left-0 flex items-center sm:hidden">
            <button
              type="button"
              className="inline-flex items-center justify-center p-2 rounded-md text-gray-800 dark:text-gray-400 hover:bg-gray-300 dark:hover:text-white dark:hover:bg-gray-700"
              onClick={() => setIsOpen(!isOpen)}
            >
              <span className="sr-only">Open main menu</span>
              {isOpen ? <HiX className="block h-6 w-6" /> : <HiMenu className="block h-6 w-6" />}
            </button>
          </div>
          <div className="flex-1 flex items-center justify-center sm:items-stretch sm:justify-start">
            <div className="flex-shrink-0 flex items-center">
              {/* Your Logo Here */}
            </div>
            <div className="hidden sm:block sm:ml-6">
              <div className="flex space-x-4">
                {navigation.map((item) => (
                  <NavLink
                    key={item.name}
                    to={item.to}
                    className={({ isActive }) =>
                      isActive ? "bg-gray-700 text-white px-3 py-2 rounded-md text-sm font-medium" : "text-white hover:bg-gray-700 px-3 py-2 rounded-md text-sm font-medium"
                  }
                  >
                    {item.name}
                  </NavLink>
                ))}
              </div>
            </div>
          </div>
          {/* User Authentication and Settings */}
          <div className="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
            {isAuthenticated ? (
              <>
              
                <input
                  type="text"
                  readOnly
                  className="text-gray-900 pl-4 pr-4 py-2 border rounded-lg"
                  value={principals || "Fetching..."}
                />
                <button onClick={logout} className="ml-4 text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 p-2">
                  <HiOutlineLogout size={24} style={{ color: 'currentColor' }} />
                </button>
              </>
            ) : (
              <button
                onClick={login}
                className="bg-gradient-to-r-indigo-500-700 hover:bg-gradient-to-r-indigo-700-darker text-white font-bold text-lg  py-2 px-4 rounded"
              >
                Connect to Wallet
              </button>
            )}
          </div>
        </div>
      </div>
      {/* Mobile Menu */}
      {isOpen && (
        <div className="sm:hidden">
          <div className="px-2 pt-2 pb-3 space-y-1">
            {navigation.map((item) => (
              <NavLink
                key={item.name}
                to={item.to}
                className={({ isActive }) =>
                  isActive ? "bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium" : "text-gray-600 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium"
                }
                onClick={() => setIsOpen(false)}
              >
                {item.name}
              </NavLink>
            ))}
          </div>
        </div>
      )}
    </nav>
  );
}

export default MyNavbar;
