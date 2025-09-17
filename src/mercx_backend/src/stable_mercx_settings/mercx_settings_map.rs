use crate::stable_mercx_settings::stable_mercx_settings::StableMercxSettings;
use crate::stable_memory::MERCX_SETTINGS;


pub fn get() -> StableMercxSettings {
    MERCX_SETTINGS.with(|s| s.borrow().get().clone())
}

pub fn inc_token_map_idx() -> u32 {
    MERCX_SETTINGS.with(|s| {
        ic_cdk::println!("🔍 MERCX_SETTINGS accessed here");
        let mut map = s.borrow_mut();
        let mercx_settings = map.get();
        let token_map_idx = mercx_settings.token_map_idx + 1;
        let new_mercx_settings = StableMercxSettings {
            token_map_idx,
            ..mercx_settings.clone()
        };
        _ = map.set(new_mercx_settings);
        token_map_idx
    })
}


 
pub fn inc_lp_metadata_map_idx() -> u32 {
    MERCX_SETTINGS.with(|rc| {
        ic_cdk::println!("🔍 MERCX_SETTINGS accessed here");
        // Borrow the RefCell mutably to access the StableCell
        let mut cell = rc.borrow_mut();

        // Read current settings (clone because get() returns &T)
        let mut st = cell.get().clone();

        // Increment safely
        st.lp_metadata_map_idx = st.lp_metadata_map_idx.saturating_add(1);

        // Persist back to StableCell
        cell.set(st.clone()).expect("Failed to save settings");

        st.lp_metadata_map_idx
    })
}

pub fn inc_lp_token_map_idx() -> u64 {
    MERCX_SETTINGS.with(|s| {
        ic_cdk::println!("🔍 MERCX_SETTINGS accessed here");
        let mut map = s.borrow_mut();
        let mercx_settings = map.get();
        let lp_token_map_idx = mercx_settings.lp_token_map_idx + 1;
        let new_mercx_settings = StableMercxSettings {
            lp_token_map_idx,
            ..mercx_settings.clone()
        };
        _ = map.set(new_mercx_settings);
        lp_token_map_idx
    })
}



pub fn inc_pool_map_idx() -> u32 {
    MERCX_SETTINGS.with(|s| {
        let mut map = s.borrow_mut();
        let mercx_settings = map.get();
        let pool_map_idx = mercx_settings.pool_map_idx + 1;
        let new_mercx_settings = StableMercxSettings {
            pool_map_idx,
            ..mercx_settings.clone()
        };
        _ = map.set(new_mercx_settings);
        pool_map_idx
    })
}


pub fn reset_token_map_idx() {
    MERCX_SETTINGS.with(|s| {
        let mut map = s.borrow_mut();
      
        let current_settings = map.get();
        let new_settings = StableMercxSettings {
            token_map_idx: 0,
            ..current_settings.clone()
        };
        _ = map.set(new_settings);
    });
} 

pub fn reset_pool_map_idx() {
    MERCX_SETTINGS.with(|s| {
        let mut map = s.borrow_mut();
      
        let current_settings = map.get();
        let new_settings = StableMercxSettings {
            pool_map_idx: 0,
            ..current_settings.clone()
        };
        _ = map.set(new_settings);
    });
} 

pub fn inc_transfer_map_idx() -> u64 {
    MERCX_SETTINGS.with(|s| {
        let mut map = s.borrow_mut();
        let mercx_settings = map.get();
        let transfer_map_idx = mercx_settings.transfer_map_idx + 1;
        let new_mercx_settings = StableMercxSettings {
            transfer_map_idx,
            ..mercx_settings.clone()
        };
        _ = map.set(new_mercx_settings);
        transfer_map_idx
    })
}



pub fn reset_lp_map_idx() {
    MERCX_SETTINGS.with(|s| {
        let mut map = s.borrow_mut();
      
        let current_settings = map.get();
        let new_settings = StableMercxSettings {
            lp_token_map_idx: 0,
            ..current_settings.clone()
        };
        _ = map.set(new_settings);
    });
} 


pub fn reset_lp_metadata_map_idx() {
    MERCX_SETTINGS.with(|s| {
        let mut map = s.borrow_mut();
      
        let current_settings = map.get();
        let new_settings = StableMercxSettings {
            lp_metadata_map_idx: 0,
            ..current_settings.clone()
        };
        _ = map.set(new_settings);
    });
} 


pub fn reset_transfers_map_idx() {
    MERCX_SETTINGS.with(|s| {
        let mut map = s.borrow_mut();
      
        let current_settings = map.get();
        let new_settings = StableMercxSettings {
            transfer_map_idx: 0,
            ..current_settings.clone()
        };
        _ = map.set(new_settings);
    });
} 