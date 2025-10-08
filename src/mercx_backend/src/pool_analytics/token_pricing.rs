//  Token Pricing Utilities
//  ------------------------------------------
//  Provides USD pricing for tokens using:
//    1. Pool-based routes (stable, anchor, 2-hop)
//    2. Cached XRC rates for major anchors
// ==========================================
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::api::time;

use crate::pool::handlers::get_by_tokens;
use crate::xrc_mock::get_rate_vs_usd;

// ----------------------------
// 🔹 Configuration Constants
// ----------------------------

/// Anchor assets used for multi-hop routing
const ANCHORS: [&str; 5] = ["USDT", "USDC", "ICP", "ETH", "BTC"];

/// Determine if a symbol is a stablecoin (always ≈ 1 USD)
fn is_stable(sym: &str) -> bool {
    matches!(sym, "USDT" | "USDC" | "CKUSDT" | "CKUSDC")
}

// ----------------------------
// 🔹 Symbol Normalization
// ----------------------------

/// Normalize token symbols to a canonical uppercase form
///
/// Examples:
/// - "ckusdt"  → "CKUSDT"
/// - "usdt.e"  → "USDT"
/// - "weth"    → "ETH"
/// - "bella"   → "BELLA"
pub fn normalize<'a>(sym: &'a str) -> Cow<'a, str> {
    let s = sym.trim().to_uppercase();
    let s = s.strip_suffix(".E").unwrap_or(&s).to_string();

    match s.as_str() {
        // Keep ck-prefixed tokens distinct (not same as native)
        "CKBTC"  => Cow::from("CKBTC"),
        "CKETH"  => Cow::from("CKETH"),
        "CKUSDT" => Cow::from("CKUSDT"),
        "CKUSDC" => Cow::from("CKUSDC"),

        // Normalize wrapped aliases
        "WBTC"   => Cow::from("BTC"),
        "WETH"   => Cow::from("ETH"),

        // Default: return uppercase canonical
        _ => Cow::from(s),
    }
}

// ----------------------------
// 🔹 Pool-Based Pricing Logic
// ----------------------------

/// Return price of `a` in terms of `b` (how many `b` units for 1 `a`)
fn pool_price_a_in_b(a: &str, b: &str) -> Result<f64, String> {
    let na = normalize(a);
    let nb = normalize(b);

    // Try both symbol orders when searching for the pool
    let pool = get_by_tokens(na.to_string(), nb.to_string())
        .or_else(|_| get_by_tokens(nb.to_string(), na.to_string()))
        .map_err(|_| "Pool not found".to_string())?;

    // Get pool token symbols
    let symbol0 = pool.symbol_0();
    let symbol1 = pool.symbol_1();
    let sym0 = normalize(symbol0.as_str());
    let sym1 = normalize(symbol1.as_str());

    // Get price (mid-price) from pool
    let mut price = pool
        .get_price_as_f64()
        .ok_or("Price unavailable".to_string())?;

    // If pool is reversed, invert the price
    if sym0 == nb && sym1 == na {
        if price <= 0.0 {
            return Err("Invalid inverted price".to_string());
        }
        price = 1.0 / price;
    }

    Ok(price)
}

// ----------------------------
// 🔹 Cached XRC Rate Access
// ----------------------------

// Cache of recently fetched XRC prices (symbol → (rate, timestamp))
thread_local! {
    static XRC_CACHE: RefCell<HashMap<String, (f64, u64)>> = RefCell::new(HashMap::new());
}

/// Get cached or fresh USD rate for an anchor asset using XRC.
/// Falls back to `get_rate_vs_usd()` if cache expired.
async fn get_anchor_usd_cached(symbol: &str) -> Result<f64, String> {
    if is_stable(symbol) {
        return Ok(1.0);
    }

    let now = time();
    let key = symbol.to_uppercase();

    // Return cached value if still valid (TTL = 5 minutes)
    if let Some((val, ts)) = XRC_CACHE.with(|c| c.borrow().get(&key).cloned()) {
        if now - ts < 300_000_000_000 {
            return Ok(val);
        }
    }

    // Not cached → fetch fresh rate
    let val = get_rate_vs_usd(symbol.to_string()).await?;
    XRC_CACHE.with(|c| c.borrow_mut().insert(key, (val, now)));
    Ok(val)
}

// Public wrapper that applies a second cache (for anchors)
thread_local! {
    static RATE_CACHE: RefCell<HashMap<String, (f64, u64)>> = RefCell::new(HashMap::new());
}

/// Get anchor/USD price (cached) for anchor assets like ICP, BTC, ETH.
pub async fn get_anchor_usd(anchor: &str) -> Result<f64, String> {
    if is_stable(anchor) {
        return Ok(1.0);
    }

    let key = anchor.to_uppercase();
    let now = time();

    // Return cached rate if still valid
    if let Some((v, ts)) = RATE_CACHE.with(|c| c.borrow().get(&key).cloned()) {
        if now - ts < 300_000_000_000 {
            return Ok(v);
        }
    }

    // Otherwise fetch from XRC
    let rate = get_anchor_usd_cached(anchor).await?;
    RATE_CACHE.with(|c| c.borrow_mut().insert(key, (rate, now)));
    Ok(rate)
}


/// Try to price any token in USD by routing through pools and XRC.
///

/// Flow:
/// 1️⃣ Direct stablecoin pair (TKN/USDT or TKN/USDC)
/// 2️⃣ Single-hop anchor (TKN → ICP/ETH/BTC)
/// 3️⃣ Two-hop route (TKN → REF → USD)
/// 4️⃣ Fallback → error
#[ic_cdk::update]
pub async fn get_usd_price_from_pools(raw_symbol: String) -> Result<f64, String> {
    let token = normalize(&raw_symbol);
    ic_cdk::println!("🔍 [PRICE TRACE] Checking USD route for {}", token);
    // 0️⃣ Stablecoins → always 1.0 USD
    if is_stable(token.as_ref()) {
              ic_cdk::println!("💵 {} is stable → 1.0 USD", token);
        return Ok(1.0);
    }

    // 1️⃣ Direct USD-pegged pair
    for usd_sym in ["USDT", "USDC", "CKUSDT" , "CKUSDC"] {
        if let Ok(p) = pool_price_a_in_b(token.as_ref(), usd_sym) {
            if p.is_finite() && p > 0.0 {
                     ic_cdk::println!(
                    "💠 Direct stable pair found: {} / {} → {:.6} USD",
                    token, usd_sym, p
                );
                return Ok(p);
            }
        }
    }

    // 2️⃣ Single-hop via anchor (ICP, ETH, BTC)
    for anchor in ["ICP", "ETH", "BTC"] {
        if let Ok(price_tkn_in_anchor) = pool_price_a_in_b(token.as_ref(), anchor) {
            let anchor_usd = get_anchor_usd(anchor).await?;
              let price = price_tkn_in_anchor * anchor_usd;
            ic_cdk::println!(
                "🌍 Anchor route found: {} / {} = {:.6} × {}_USD {:.6} → {:.6} USD",
                token, anchor, price_tkn_in_anchor, anchor, anchor_usd, price
            );
            return Ok(price_tkn_in_anchor * anchor_usd);
        }
    }

    // 3️⃣ Two-hop route: TKN → REF → USD (REF = anchor or stable)
    for ref_sym in ANCHORS {
        let hop1 = match pool_price_a_in_b(token.as_ref(), ref_sym) {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => continue,
        };

        let hop2 = get_anchor_usd(ref_sym).await?;
             let final_price = hop1 * hop2;
             ic_cdk::println!(
                "🔗 Two-hop route: {} → {} → USD = {:.6} × {:.6} → {:.6} USD",
                token, ref_sym, hop1, hop2, final_price
            );
        return Ok(hop1 * hop2);
    }

    // 4️⃣ Fallback
    Err(format!(
        "No USD route found for {} (no stable, anchor, or two-hop route)",
        token
    ))
}