import { AuthClient } from "@dfinity/auth-client";
import React, { createContext, useContext, useEffect, useState } from "react";
import { canisterId as MercxId, createActor as createMercxActor } from "../../declarations/mercx_backend";
import { canisterId, createActor } from "../../declarations/icrc1_ledger_canister";
import { canisterId as icrcIndexCanisterId, createActor as createIndexActor } from "../../declarations/icrc1_index_canister";
import { canisterId as icpCanisterId, createActor as createIcpActor } from "../../declarations/icp_ledger_canister";
import { canisterId as tommyCanisterId, createActor as createTommyActor } from "../../declarations/tommy_icrc1_ledger";
import { canisterId as kycCanisterId, createActor as createKycActor } from "../../declarations/kyc";
import { canisterId as fxmxCanisterId, createActor as createFXMXActor } from "../../declarations/fxmx_icrc1_ledger";


// Create a React Context for sharing authentication status across the component tree
const AuthContext = createContext();
function detectInAppBrowser() {
  const ua = navigator.userAgent.toLowerCase();
  return (
    ua.includes("linkedin") ||
    ua.includes("fban") || ua.includes("fbav") ||  // Facebook
    ua.includes("instagram") ||
    ua.includes("twitter") ||
    ua.includes("edge") || ua.includes("edg")
  );
}

function handleRedirect() {
  if (detectInAppBrowser()) {
    if (window.location.pathname !== "/InappBrowser") {
      window.location.href = "/InappBrowser";  // Redirect only if NOT already there
    }
  }
}

// Run the function when the page loads


// function openInExternalBrowser() {
//   const ua = navigator.userAgent.toLowerCase();

//   // Detect in-app browsers
//   if (
//     ua.includes("LinkedIn") ||
//     ua.includes("FBAN") || ua.includes("FBAV") ||  // Facebook
//     ua.includes("Instagram") ||                    // Instagram
//     ua.includes("Twitter") ||
//     //ua.includes("chrome") 
//     ua.includes("edge") || ua.includes("edg") // Twitter
//   ) {
//     alert("Please open this link in Safari or Chrome for authentication.");
//     //window.location.href = `googlechrome://${window.location.href.replace("https://xpm3z-7qaaa-aaaan-qzvlq-cai.icp0.io/")}`;
//     // Get the current URL without protocol
//     let formattedURL = window.location.href;
//     // Ensure the URL uses HTTPS (required for Edge redirection)
//     if (!formattedURL.startsWith("https")) {
//       formattedURL = formattedURL.replace(/^http:\/\//, "https://");
//     }


//     //Redirect user to Microsoft Edge with the correct format
//     window.location.href = `googlechrome:${formattedURL}`;

//   }
// }
// function openInExternalBrowser() {
//   const ua = navigator.userAgent.toLowerCase();

//   if (ua.includes("linkedin") || ua.includes("fban") || ua.includes("fbav") || ua.includes("instagram") || ua.includes("twitter")) {
//     const formattedURL = window.location.href.replace(/^http:\/\//, "https://");
//     const isIOS = /iphone|ipad|ipod/.test(ua);

//     if (isIOS) {
//       // Attempt to open in Safari (iOS only)
//       window.location.href = `googlechrome://${formattedURL}`;
//     } else {
//       // Attempt to open in Chrome or Edge (Android/Desktop)
//       window.location.href = `microsoft-edge://${formattedURL}`;
//     }


//   }
// }
// function openInExternalBrowser() {
//   const ua = navigator.userAgent.toLowerCase();

//   // Detect in-app browsers
//   if (
//     ua.includes("linkedin") || // LinkedIn
//     ua.includes("fban") || ua.includes("fbav") || // Facebook
//     ua.includes("instagram") || // Instagram
//     ua.includes("twitter") || // Twitter
//     ua.includes("edge") || ua.includes("edg")
//   ) {
//     // Show a message to the user
//     alert("Please open this link in Safari, Chrome, or Edge for authentication.");
//     const formattedURL = window.location.href.replace(/^http:\/\//, "https://");
//     // Optionally, provide a button or link for the user to manually open the URL
//     const confirmation = confirm("Do you want to copy the link and open it in an external browser?");
//     if (confirmation) {
//       // Copy the URL to the clipboard
//       window.location.href = `googlechrome://${formattedURL}`;
//       //alert("Please manually copy the URL and open it in an external browser.");

//     }
//   }
// }



// Function to determine the correct identity provider URL based on environment and browser
export const getIdentityProvider = () => {
  let idpProvider;
  // Safeguard against server rendering
  // Check if the code is running in a browser environment
  if (typeof window !== "undefined") {
    // Determine if the environment is local (not production)
    const isLocal = process.env.DFX_NETWORK !== "ic";
    // Safari does not support localhost subdomains
    // Check if the user's browser is Safari to handle specific limitations
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
    if (isLocal && isSafari) {
      // Safari handling for local development environment
      idpProvider = `http://localhost:8001/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;
    } else if (isLocal) {
      // General handling for non-Safari browsers in local development
      idpProvider = `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:8001`;
    }
  }
  return idpProvider;
};

// Default options for the authentication client
export const defaultOptions = {
  /**
   *  @type {import("@dfinity/auth-client").AuthClientCreateOptions}
   */
  createOptions: {
    idleOptions: {
      // Set to true if you do not want idle functionality
      // Opt to disable idle functionality for the auth client
      disableIdle: true,
    },
  },
  /**
   * @type {import("@dfinity/auth-client").AuthClientLoginOptions}
   */
  loginOptions: {
    // Specify the identity provider determined by getIdentityProvider
    identityProvider: getIdentityProvider(),
  },
};

/**
 *
 * @param options - Options for the AuthClient
 * @param {AuthClientCreateOptions} options.createOptions - Options for the AuthClient.create() method
 * @param {AuthClientLoginOptions} options.loginOptions - Options for the AuthClient.login() method
 * @returns
 */
// Custom hook to manage authentication state
export const useAuthClient = (options = defaultOptions) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [authClient, setAuthClient] = useState(null);
  const [identity, setIdentity] = useState(null);
  const [principal, setPrincipal] = useState(null);
  const [whoamiActor, setWhoamiActor] = useState(null);
  const [icrcIndexActor, setIcrcIndexActor] = useState(null);
  const [icpActor, setIcpActor] = useState(null);
  const [mercx_Actor, setMercxActor] = useState(null);
  const [tommy_Actor, setTommyActor] = useState(null);
  const [kycActor, setKycActor] = useState(null);    // ✅ KYC actor
  const [fxmxActor, setFXMXActor] = useState(null);


  useEffect(() => {
    handleRedirect();
    // openInExternalBrowser();
    // Initialize AuthClient
    // Create an AuthClient instance when the component mounts
    AuthClient.create(options.createOptions).then(async (client) => {

      updateClient(client);

    });


  }, []);

  // Function to handle user login
  const login = () => {
    authClient.login({
      ...options.loginOptions,
      onSuccess: () => {
        updateClient(authClient);
      },

    });
  };

  // Update local state with the new auth client details
  async function updateClient(client) {
    const isAuthenticated = await client.isAuthenticated();
    setIsAuthenticated(isAuthenticated);

    const identity = client.getIdentity();
    setIdentity(identity);

    const principal = identity.getPrincipal();
    setPrincipal(principal);

    setAuthClient(client);

    // Create an actor with the authenticated identity
    const actor = createActor(canisterId, {
      agentOptions: {
        identity,
      },
    });

    setWhoamiActor(actor);
    const indexActor = createIndexActor(icrcIndexCanisterId, {
      agentOptions: {
        identity,
      },
    });
    setIcrcIndexActor(indexActor);

    const IcpActor = createIcpActor(icpCanisterId, {
      agentOptions: {
        identity,
      },
    });
    setIcpActor(IcpActor);

    const MercxActor = createMercxActor(MercxId, {
      agentOptions: {
        identity,
      },
    });
    setMercxActor(MercxActor);

    const tommyActor = createTommyActor(tommyCanisterId, {
      agentOptions: {
        identity,
      },
    });
    setTommyActor(tommyActor);

    // ✅ Create the KYC Actor
    const kycActor = createKycActor(kycCanisterId, {
      agentOptions: {
        identity,
      },
    });
    setKycActor(kycActor);

    const FxmxActor = createFXMXActor(fxmxCanisterId, {
      agentOptions: {
        identity,
      },
    });
    setFXMXActor(FxmxActor);
  }

  async function logout() {
    await authClient?.logout();
    await updateClient(authClient);
  }

  return {
    isAuthenticated,
    login,
    logout,
    authClient,
    identity,
    principal,
    whoamiActor,
    icrcIndexActor,
    icpActor,
    mercx_Actor,
    tommy_Actor,
    kycActor,     // ✅ Return KYC actor
    fxmxActor,
  };
};

/**
 * @type {React.FC}
 */
// Provider component to wrap the application and provide auth state via context
export const AuthProvider = ({ children }) => {
  const auth = useAuthClient();

  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
};

//A simple hook that allows any component within the context provider to access the auth state and methods easily.
export const useAuth = () => useContext(AuthContext);