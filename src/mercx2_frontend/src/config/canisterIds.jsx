// config/canisterIds.js
// Access DFX environment variables
export const DFX_CONFIG = {
    VERSION: process.env.DFX_VERSION,
    NETWORK: process.env.DFX_NETWORK,
  };
  
  // Main canister IDs from DFX environment
  export const CANISTER_IDS = {
    // Core MercX canisters
    MERCX_BACKEND: process.env.CANISTER_ID_MERCX_BACKEND,
    MERCX_FRONTEND: process.env.CANISTER_ID_MERCX2_FRONTEND,
    KYC: process.env.CANISTER_ID_KYC,
    
    // Token ledgers
    TOMMY_ICRC1_LEDGER: process.env.CANISTER_ID_TOMMY_ICRC1_LEDGER,
    TOMMY_ICRC1_INDEX: process.env.CANISTER_ID_TOMMY_ICRC1_INDEX,
    GBX_ICRC1_LEDGER: process.env.CANISTER_ID_GBX_ICRC1_LEDGER,
    FXMX_ICRC1_LEDGER: process.env.CANISTER_ID_FXMX_ICRC1_LEDGER,
    FXMX_ICRC1_INDEX: process.env.CANISTER_ID_FXMX_ICRC1_INDEX,
    EGX30_ICRC1_LEDGER: process.env.CANISTER_ID_EGX30_ICRC1_LEDGER,
    
    // ICP ecosystem canisters
    INTERNET_IDENTITY: process.env.CANISTER_ID_INTERNET_IDENTITY,
    ICRC1_LEDGER: process.env.CANISTER_ID_ICRC1_LEDGER_CANISTER,
    ICRC1_INDEX: process.env.CANISTER_ID_ICRC1_INDEX_CANISTER,
    ICP_LEDGER: process.env.CANISTER_ID_ICP_LEDGER_CANISTER,
    CKUSDT_LEDGER: process.env.CANISTER_ID_CKUSDT_LEDGER_CANISTER,
    
    // Exchange rate canister
    XRC: process.env.CANISTER_ID_XRC,
  };
  
  // Validation function to ensure required canisters are available
  export const validateCanisterIds = () => {
    const required = [
      'MERCX_BACKEND',
      'KYC',
      'INTERNET_IDENTITY'
    ];
    
    const missing = required.filter(key => !CANISTER_IDS[key]);
    
    if (missing.length > 0) {
      console.warn(`Missing canister IDs: ${missing.join(', ')}`);
    }
    
    return missing.length === 0;
  };
  
  // Helper functions for commonly used canisters
  export const getMercXBackendId = () => CANISTER_IDS.MERCX_BACKEND;
  export const getKYCCanisterId = () => CANISTER_IDS.KYC;
  export const getSpenderId = () => CANISTER_IDS.MERCX_BACKEND; // If spender is same as backend
  
  // Export individual IDs for direct import
  export const {
    MERCX_BACKEND,
    KYC,
    TOMMY_ICRC1_LEDGER,
    FXMX_ICRC1_LEDGER,
    EGX30_ICRC1_LEDGER,
    CKUSDT_LEDGER,
    INTERNET_IDENTITY
  } = CANISTER_IDS;
  
  export default CANISTER_IDS;