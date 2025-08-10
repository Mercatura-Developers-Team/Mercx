export function parseAmount(amountStr, decimals) {
    const [whole, fraction = ""] = String(amountStr).split(".");
    return BigInt(whole + fraction.padEnd(decimals, "0"));
  }
  
  export function normalizeAmount(amount, decimals) {
    return Number(amount) / 10 ** decimals;
  }
  