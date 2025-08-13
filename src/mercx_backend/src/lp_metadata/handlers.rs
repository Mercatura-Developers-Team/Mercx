
use crate::lp_metadata::stable_lp_metadata::{StableLpMetadataId,LPToken};
use crate::stable_memory::LPMETADATA;
use crate::stable_mercx_settings::mercx_settings_map;
use crate::StableToken;
use crate::stable_mercx_settings::mercx_settings_map::reset_lp_metadata_map_idx;
pub fn get_by_token_id(token_id: u32) -> Option<LPToken> {
    LPMETADATA.with(|m| m.borrow().get(&StableLpMetadataId(token_id)))
}

pub fn get_by_symbol(symbol: &str) -> Result<LPToken, String> {
    LPMETADATA.with(|map| {
        map.borrow()
            .iter()
            .find_map(|(_, token)| {
                if token.symbol == symbol {
                    Some(token.clone())
                } else {
                    None
                }
            })
    })
    .ok_or_else(|| format!("LP Token with symbol '{}' not found", symbol))
}

/// return all lp tokens
#[ic_cdk::query]
pub fn get() -> Vec<LPToken> {
    LPMETADATA.with(|m| {
        m.borrow()
            .iter()
            .filter_map(|(_, v)|   Some(v) )
            .collect()
    })
}


pub fn lp_token_exists_by_symbol(symbol: &str) -> bool {
    LPMETADATA.with(|m| {
        m.borrow()
            .iter()
            .any(|(_, token)| token.symbol == symbol)
    })
}

pub fn insert(token: &LPToken) -> Result<u32, String> {
     // 1. Check if LP token with the same symbol already exists
    if lp_token_exists_by_symbol(&token.symbol) {
        //i dont know if I should remove return here?
        return Err("LP Token with this symbol already exists".to_string());
    }

        // 2. Insert into the LPMETADATA map with a unique token_id
    let insert_token = LPMETADATA.with(|m| {
        let mut map = m.borrow_mut();
        let token_id = mercx_settings_map::inc_lp_metadata_map_idx();
        let insert_token =  LPToken { token_id, ..token.clone() };
        
        map.insert(StableLpMetadataId(token_id), insert_token.clone());
        insert_token
    });

    // 3. Return the new token_id
    Ok(insert_token.token_id())
}


pub fn add_lp_token(token_0: &StableToken, token_1: &StableToken) -> Result<LPToken, String> {
    let lp_token = LPToken::new(token_0, token_1);
    let token_id = insert(&lp_token)?;

    // Retrieves the inserted token by its token_id
    get_by_token_id(token_id).ok_or_else(|| "Failed to add LP token".to_string())
}

pub fn exists(symbol: &str) -> bool {
    LPMETADATA.with(|map| {
        map.borrow()
            .iter()
            .any(|(_, lp_token)| lp_token.symbol == symbol)
    })
}


#[cfg(not(feature = "prod"))]
#[ic_cdk::update]
fn reset_lp_metadata_tokens() -> Result<String, String> {
    LPMETADATA.with(|tokens| {
        tokens.borrow_mut().clear_new(); // `clear_new()` btmsh kolo remove law hanmsh haga specific
    });

    reset_lp_metadata_map_idx();

    Ok("✅ Tokens memory cleared".to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::stable_memory::LPMETADATA;
    use crate::stable_mercx_settings::mercx_settings_map;
    use candid::Principal;
    use candid::Nat;

    #[cfg(test)]
    impl StableToken {
        /// Minimal fake token for unit tests
        pub fn fake(token_id: u32, symbol: &str, decimals: u8) -> Self {
            Self {
                token_id,
                name: symbol.to_string(),
                symbol: symbol.to_string(),
                decimals,
                canister_id: Principal::from_text("aaaaa-aa").unwrap(),
                fee: Nat::from(0u32),
                icrc1: true,
                icrc2: false,
                icrc3: false,
            }
        }
    }

    #[test]
    fn test_add_lp_token_success() {
        // Setup - clear state
        LPMETADATA.with(|m| m.borrow_mut().clear_new());
        mercx_settings_map::reset_lp_metadata_map_idx();

        // Create test tokens
        let token_0 = StableToken::fake(1, "TOKENA", 8);
        let token_1 = StableToken::fake(2, "TOKENB", 8);

        // Test
        let result = add_lp_token(&token_0, &token_1);
        assert!(result.is_ok());
        
        let lp_token = result.unwrap();
        // Only test fields that exist in your struct
        assert_eq!(lp_token.symbol, "TOKENA_TOKENB");
        assert_eq!(lp_token.decimals, 8);
        
        // Verify the token was stored
        let stored_token = get_by_token_id(lp_token.token_id).expect("Token should exist");
        assert_eq!(stored_token.symbol, "TOKENA_TOKENB");
    }

    #[test]
    fn test_add_lp_token_duplicate_symbol() {
        // Setup
        LPMETADATA.with(|m| m.borrow_mut().clear_new());
        mercx_settings_map::reset_lp_metadata_map_idx();

        let token_0 = StableToken::fake(1, "TOKENA", 8);
        let token_1 = StableToken::fake(2, "TOKENB", 8);

        // First add should succeed
        assert!(add_lp_token(&token_0, &token_1).is_ok());
        
        // Second add with same tokens should fail (same symbol)
        let result = add_lp_token(&token_0, &token_1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_add_lp_token_reverse_pair() {
        // Setup
        LPMETADATA.with(|m| m.borrow_mut().clear_new());
        mercx_settings_map::reset_lp_metadata_map_idx();

        let token_0 = StableToken::fake(1, "TOKENA", 8);
        let token_1 = StableToken::fake(2, "TOKENB", 8);

        // Add both directions (should create different LP tokens)
        let lp_token1 = add_lp_token(&token_0, &token_1).unwrap();
        let lp_token2 = add_lp_token(&token_1, &token_0).unwrap();
        
        // Should create two separate LP tokens with different symbols
        assert_ne!(lp_token1.symbol, lp_token2.symbol);
        assert_eq!(lp_token1.symbol, "TOKENA_TOKENB");
        assert_eq!(lp_token2.symbol, "TOKENB_TOKENA");
        
        // Verify both tokens were stored
        assert_eq!(get().len(), 2);
    }
}