import React, { useState, useEffect } from "react";
import { useAuth } from "./use-auth-client";
import './index.css';
import {HiOutlineLogout, 
  HiMenu, 
  HiX, 
  HiHome, 
  HiCurrencyDollar, 
  HiReceiptRefund, 
  HiCollection, 
  HiUserGroup,
  HiSparkles} from "react-icons/hi";
import { NavLink } from 'react-router-dom';
import { CopyToClipboard } from 'react-copy-to-clipboard';
import { FaCopy ,  FaExchangeAlt, 
  FaWater, 
  FaUserTie,
  FaChevronDown } from "react-icons/fa";

const navigation = [
  { name: "Home", to: "/", icon: <HiHome className="mr-2" /> },
  { name: "Trade", to: "/trade", icon: <FaExchangeAlt className="mr-2" /> },
  { name: "Transactions", to: "/transactions", icon: <HiReceiptRefund className="mr-2" /> },
  { name: "Wallet", to: "/wallet", icon: <HiCurrencyDollar className="mr-2" /> },
  { 
    name: "Pools", 

    icon: <HiCollection className="mr-2" />,
    dropdown: [
      { name: "Liquidity Pools", to: "/pools", icon: <FaWater className="mr-2 text-blue-400" /> },
      { name: "Liquidity Providers", to: "/liquidity", icon: <FaUserTie className="mr-2 text-purple-400" /> }
    ]
  }
];

function MyNavbar() {
  const { isAuthenticated, login, logout, principal, kycActor } = useAuth();
  const [isOpen, setIsOpen] = useState(false);
  const [copied, setCopied] = useState(false);
  const [userExists, setUserExists] = useState(false);
  const [username, setUsername] = useState("");
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const [initial, setInitial] = useState("");
  const [poolsDropdownOpen, setPoolsDropdownOpen] = useState(false);

  useEffect(() => {
    const fetchUsername = async () => {
      if (isAuthenticated && principal) {
        try {
          const exists = await kycActor.has_username_for_principal(principal);
          setUserExists(exists);
          if (exists) {
            const fetchedUsername = await kycActor.get_username_by_principal(principal);
            setUsername(fetchedUsername?.Ok || "Unknown User");
            const username1 = fetchedUsername.Ok;
            const userInitial = username1.charAt(0).toUpperCase();
            setInitial(userInitial);
          } else {
            setUsername("Unknown User");
            setInitial("U");
          }
        } catch (error) {
          console.error("Error fetching username:", error);
        }
      } else {
        console.log("Not authenticated or no principal yet.");
      }
    };
    fetchUsername();
  }, [isAuthenticated, principal]);

  useEffect(() => {
    if (!isAuthenticated) {
      setIsDropdownOpen(false);
    }
  }, [isAuthenticated]);

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
          <div className="flex-1 flex items-center justify-center sm:justify-start">
            <div className="flex-shrink-0 flex items-center">
              <NavLink to="/" className="focus:outline-none hidden sm:flex items-center">
                <img src={'j.png'} alt="Logo" className="h-10 sm:h-12 mt-2 mr-8 " />
              </NavLink>
            </div>
            <div className="hidden sm:block sm:ml-6">
              <div className="flex space-x-4">
                {navigation.map((item) => (
                  item.dropdown ? (
                    <div key={item.name} className="relative group">
                         <button
                        className={`flex items-center px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                          poolsDropdownOpen
                            ? "bg-gradient-to-r-indigo-500-700 text-white shadow-lg"
                            : "text-blue-100 hover:bg-gradient-to-r-indigo-500-700 hover:text-white hover:shadow-md"
                        }`}
                        onClick={() => setPoolsDropdownOpen(!poolsDropdownOpen)}
                      >
                        {item.icon}
                        {item.name}
                        <FaChevronDown className={`ml-2 transition-transform ${poolsDropdownOpen ? "rotate-180" : ""}`} />
                      </button>
                      {poolsDropdownOpen && (
  <div className="absolute left-0 mt-2 w-56 rounded-lg shadow-xl bg-gradient-to-b from-gray-800 to-gray-900 ring-1 ring-blue-500 ring-opacity-50 z-50 overflow-hidden">                          <div className="py-1">
                            {item.dropdown.map((dropdownItem) => (
                              <NavLink
                                key={dropdownItem.name}
                                to={dropdownItem.to}
                                className={({ isActive }) =>
                                    `flex items-center px-4 py-3 text-sm transition-all duration-200 hover:bg-gray-900 hover:transform hover:translate-x-1 ${
                                    isActive
                                      ? " text-white font-semibold"
                                      : "text-blue-100"
                                  }`
                                }
                                onClick={() => setPoolsDropdownOpen(false)}
                              >
                                {dropdownItem.icon}
                                {dropdownItem.name}
                              </NavLink>
                            ))}
                          </div>
                        </div>
                      )}
                    </div>
                  ) : (
                    <NavLink
                      key={item.name}
                      to={item.to}
                          className={({ isActive }) =>
                        `flex items-center px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                          isActive
                            ? "bg-gradient-to-r-indigo-500-700 text-white shadow-lg"
                            : "text-blue-100 hover:bg-gradient-to-r-indigo-500-700 hover:text-white hover:shadow-md"
                        }`
                      }
                    >
                            {item.icon}
                      {item.name}
                    </NavLink>
                  )
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
                  className="text-gray-900 pl-4 pr-4 py-2 border rounded-3xl text-xs sm:text-sm"
                  value={principal || "Fetching..."}
                />
                <CopyToClipboard
                  text={principal}
                  onCopy={() => {
                    setCopied(true);
                    setTimeout(() => {
                      setCopied(false);
                    }, 3000);
                  }}
                >
                  <button className="m-3 bg-gradient-to-r-indigo-500-700 hover:bg-gradient-to-r-indigo-700-darker focus:outline-none focus:ring-2 focus:ring-blue-500 p-2 rounded-full">
                    <FaCopy size={19} style={{ color: 'currentColor' }} />
                  </button>
                </CopyToClipboard>
                {copied && <span className="text-xs p-1" style={{ color: 'lightblue' }}>Copied</span>}
                <div className="relative">
                  <button
                    onClick={() => setIsDropdownOpen(!isDropdownOpen)}
                    className="flex items-center justify-center w-10 h-10 bg-gradient-to-r-indigo-500-700 hover:bg-gradient-to-r-indigo-700-darker rounded-full text-white font-bold focus:outline-none"
                  >
                    {initial}
                  </button>
                  {isDropdownOpen && (
                    <div className="absolute right-0 mt-2 w-48 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 z-50">
                      <div className="py-1">
                        <div className="block px-4 py-2 text-sm text-bold text-gray-700 text-center font-semibold">{username}</div>
                        <button
                          onClick={logout}
                          className="flex items-center w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                        >
                          <HiOutlineLogout className="mr-2" />
                          Logout
                        </button>
                      </div>
                    </div>
                  )}
                </div>
              </>
            ) : (
              <button
                onClick={login}
                className="bg-gradient-to-r-indigo-500-700 hover:bg-gradient-to-r-indigo-700-darker text-white font-bold text-lg py-2 px-4 rounded"
              >
                Connect to Wallet
              </button>
            )}
          </div>
        </div>

      {/* Mobile Menu */}
        {isOpen && (
          <div className="sm:hidden bg-gray-900 shadow-inner">
            <div className="px-2 pt-2 pb-3 space-y-1">
              <div className="flex justify-center mb-4">
                <img src={'j.png'} alt="Logo" className="h-14 mt-2" />
              </div>
              
              {navigation.map((item) =>
                item.dropdown ? (
                  <div key={item.name} className="pl-3">
                    <button
                      className="flex items-center w-full text-left px-3 py-3 rounded-lg text-base font-medium text-blue-100 hover:bg-gray-800 transition-colors"
                      onClick={() => setPoolsDropdownOpen(!poolsDropdownOpen)}
                    >
                      {item.icon}
                      {item.name}
                      <FaChevronDown className={`ml-auto transition-transform ${poolsDropdownOpen ? "rotate-180" : ""}`} />
                    </button>
                    
                    {poolsDropdownOpen && (
                      <div className="pl-6 mt-1 space-y-2 border-l-2 border-blue-700 ml-2">
                        {item.dropdown.map((dropdownItem) => (
                          <NavLink
                            key={dropdownItem.name}
                            to={dropdownItem.to}
                            className={({ isActive }) =>
                              `flex items-center px-3 py-2 rounded-lg text-base transition-all ${
                                isActive
                                  ? "bg-blue-700 text-white font-semibold"
                                  : "text-blue-200 hover:bg-blue-800"
                              }`
                            }
                            onClick={() => {
                              setIsOpen(false);
                              setPoolsDropdownOpen(false);
                            }}
                          >
                            {dropdownItem.icon}
                            {dropdownItem.name}
                          </NavLink>
                        ))}
                      </div>
                    )}
                  </div>
                ) : (
                  <NavLink
                    key={item.name}
                    to={item.to}
                    className={({ isActive }) =>
                      `flex items-center px-3 py-3 rounded-lg text-base font-medium transition-colors ${
                        isActive
                          ? "bg-blue-700 text-white"
                          : "text-blue-100 hover:bg-blue-800"
                      }`
                    }
                    onClick={() => setIsOpen(false)}
                  >
                    {item.icon}
                    {item.name}
                  </NavLink>
             ))}
            </div>
          </div>
        )}
      </div>
    </nav>
  );
}

export default MyNavbar;