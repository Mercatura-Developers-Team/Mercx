import React, { useState, useEffect } from "react";
import { useAuth } from "./use-auth-client"; // Ensure this hook provides necessary functions
import './index.css';
import { HiOutlineLogout } from "react-icons/hi";
function MyNavbar() {
  const { isAuthenticated, login, whoamiActor, logout , principal } = useAuth(); // Consolidate useAuth calls into a single line
  const [principals, setPrincipal] = useState("");

  // Automatically fetch and set the principal when the user is authenticated
  useEffect(() => {
    const fetchPrincipal = async () => {
      if (isAuthenticated && whoamiActor) {
        try {
        //  const whoami = await whoamiActor.whoami();
          setPrincipal(principal);
        } catch (error) {
          console.error("Failed to fetch principal:", error);
          setPrincipal("Error fetching principal");
        }
      }
    };

    fetchPrincipal();
  }, [isAuthenticated, whoamiActor]); // Re-run the effect when isAuthenticated or whoamiActor changes

  return (
    <nav className="bg-white border-gray-200 dark:bg-gray-900">
      <div className="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
        <div className="flex items-center space-x-3 rtl:space-x-reverse">
          {/* <img src="/j.png" alt="logo" className="h-20 w-20" /> */}
        </div>
        <div className="hidden w-full md:block md:w-auto" id="navbar-default">
          <ul className="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
            {isAuthenticated ? (
              <div>

                <input
                  type="text"
                  readOnly
                  className="text-gray-900 pl-4 pr-4 py-2 border rounded-lg"
                  value={principals || "Fetching..."} // Display "Fetching..." while the principal is being loaded
                />

                <button onClick={logout} className="text-blue-500 hover:text-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 p-2 ">
                  <HiOutlineLogout size={24} style={{ color: 'currentColor' }} />
                </button>


              </div>
            ) : (
              <button
                type="button"
                onClick={login}
                className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
              >
                Connect to Wallet
              </button>
            )}
          </ul>
        </div>
      </div>
    </nav>
  );
}

export default MyNavbar;
