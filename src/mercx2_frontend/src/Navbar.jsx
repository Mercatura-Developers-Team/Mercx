import React, { useState, useEffect } from "react";
import { useAuth } from "./use-auth-client";
import './index.css';
import { HiOutlineLogout, HiMenu, HiX, HiClipboardCopy } from "react-icons/hi";
import { NavLink } from 'react-router-dom';
import { CopyToClipboard } from 'react-copy-to-clipboard';
import { useNavigate } from 'react-router-dom';  
import { FaCopy } from "react-icons/fa";

const navigation = [
  { name: "Home", to: "/" },
  { name: "Trade", to: "/trade" },
  { name: "Transactions", to: "/transactions" },
  { name: "Wallet", to: "/wallet" }
];

function MyNavbar() {
  const { isAuthenticated, login, logout, principal, kycActor } = useAuth();
  const [isOpen, setIsOpen] = useState(false);
  const [copied, setCopied] = useState(false);
  const navigate = useNavigate();  // Use navigate for redirection
  const [userExists, setUserExists] = useState(false);  // State to track if user exists
  const [isUserChecked, setIsUserChecked] = useState(false);  // State to track if the user check is complete
  const [username, setUsername] = useState(""); // State to store the username
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const [initial,setInitial]  = useState("");



  // const handleLogin = async () => {
  //   await login();
   
    
  // };

  useEffect(() => {
    const fetchUsername = async () => {
      if (isAuthenticated && principal) {  
        try {
          const exists = await kycActor.has_username_for_principal(principal);
          console.log("User exists:", exists); // Debug log
  
          setUserExists(exists);
  
          if (exists) {
            const fetchedUsername = await kycActor.get_username_by_principal(principal);
            console.log("Fetched username:", fetchedUsername); // Debug log
  
            setUsername(fetchedUsername?.Ok || "Unknown User");

            const username1 = fetchedUsername.Ok; // Extract the actual string
            console.log("Extracted username:", username);

            const userInitial = username1.charAt(0).toUpperCase(); // Use extracted string
            setInitial(userInitial);
            console.log("I:", userInitial); // Debug log

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
  }, [isAuthenticated, principal]); // We should NOT add `userExists` to dependencies
  

  // useEffect(() => {
  //   if (isAuthenticated && principal) {
  //     try {
  //       setPrincipal(principal);
  //     } catch (error) {
  //       console.error("Failed to fetch principal:", error);
  //       setPrincipal("Error fetching principal");
  //     }
  //   }
  // }, [isAuthenticated, principal]);

  // // Effect to handle redirection based on user existence
  // useEffect(() => {
  //   if (isUserChecked) {  // Ensure we navigate only after checking the user status
  //     if (!userExists) {
  //       navigate('/signup');
  //     } else {
  //       navigate('/');
  //     }
  //   }
  // }, [userExists, isUserChecked]);  // Depend on userExists and isUserChecked

  useEffect(() => {
    if (!isAuthenticated) {
      setIsDropdownOpen(false); // Close the dropdown when the user logs out
    }
  }, [isAuthenticated]);

  //const userInitial = username ? username.charAt(0).toUpperCase() : "U";

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
                  className="text-gray-900 pl-4 pr-4 py-2 border rounded-3xl text-xs sm:text-sm"
                  value={principal || "Fetching..."}
                />

                <CopyToClipboard text={principal} onCopy={() => {
                  setCopied(true);
                  setTimeout(() => {
                    setCopied(false);
                  }, 3000);
                }}>
                  <button className="m-3  bg-gradient-to-r-indigo-500-700 hover:bg-gradient-to-r-indigo-700-darker focus:outline-none focus:ring-2 focus:ring-blue-500 p-2 rounded-full">
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
                          <HiOutlineLogout className="mr-2" /> {/* Logout icon */}
                          Logout
                        </button>
                      </div>
                    </div>
                  )}
                </div>

                {/* <button onClick={logout} className="ml-1 text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 p-2">
                  <HiOutlineLogout size={24} style={{ color: 'currentColor' }} />
                </button> */}
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
