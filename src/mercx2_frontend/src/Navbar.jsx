import React, { useState, useEffect } from "react";
import { useAuth } from "./use-auth-client";
import './index.css';
import { HiOutlineLogout, HiMenu, HiX, HiClipboardCopy } from "react-icons/hi";
import { NavLink } from 'react-router-dom';
import { CopyToClipboard } from 'react-copy-to-clipboard';
import { useNavigate } from 'react-router-dom';  // Ensure useNavigate is imported
import { Principal } from "@dfinity/principal"; // Import Principal
import { FaCopy } from "react-icons/fa";

const navigation = [
  { name: "Home", to: "/" },
  { name: "Trade", to: "/trade" },
  { name: "Transactions", to: "/transactions" },
  { name: "Transfer", to: "/transfer" }
];

function MyNavbar() {
  const { isAuthenticated, login, logout, principal, kycActor } = useAuth();
  const [principals, setPrincipal] = useState("");
  const [isOpen, setIsOpen] = useState(false);
  //const [copySuccess, setCopySuccess] = useState('');
  const [copied, setCopied] = useState(false);
  const navigate = useNavigate();  // Use navigate for redirection

  const handleLogin = async () => {
    // Perform login
    await login();
    // Check if the user exists
    if (principal) {
      try {
        const userExists = await kycActor.has_username_for_principal(principal);
        console.log(userExists);
        if (!userExists) {
          navigate('/signup');  // Redirect to signup if no user
        } else {
          navigate('/');
        }
      } catch (error) {
        console.error("Error checking user existence:", error);
      }
    }
  };

  useEffect(() => {
    if (isAuthenticated) {
      try {
        handleLogin();
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
          <div className="flex-1 flex items-center justify-center sm:justify-start">
            <div className="flex-shrink-0 flex items-center">
              <NavLink to="/" className="focus:outline-none  hidden sm:flex items-center">
                <img src={'j.png'} alt="Logo" className="h-10 sm:h-12 mt-2 mr-8 " />
              </NavLink>
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
                  className="text-gray-900 pl-4 pr-4 py-2 border rounded-lg text-xs sm:text-sm"
                  value={principals || "Fetching..."}
                />
                <CopyToClipboard text={principals} onCopy={() => {
                  setCopied(true);
                  setTimeout(() => {
                    setCopied(false);
                  }, 3000);
                }}>
                  <button className="ml-4 text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 p-2 ">
                    <FaCopy size={19} style={{ color: 'currentColor' }} />
                  </button>
                </CopyToClipboard>
                {copied && <span className="text-xs p-1" style={{ color: 'lightblue' }}>Copied</span>}

                <button onClick={logout} className="ml-1 text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 p-2">
                  <HiOutlineLogout size={24} style={{ color: 'currentColor' }} />
                </button>
              </>
            ) : (
              <button
                onClick={handleLogin}
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
            <NavLink to="/" className="focus:outline-none">
              <img src={'j.png'} alt="Logo" className="logo-class h-16 mt-6 " />
            </NavLink>
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
