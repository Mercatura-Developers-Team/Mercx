
use crate::lp_metadata::stable_lp_metadata::{StableLpMetadataId,LPToken};
use crate::stable_memory::LPMETADATA;
use crate::stable_mercx_settings::mercx_settings_map;

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
#[ic_cdk::update]
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


pub fn insert(token: LPToken) -> Result<u32, String> {
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