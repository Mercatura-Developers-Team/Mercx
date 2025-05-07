use crate::stable_mercx_settings::stable_mercx_settings::StableMercxSettings;
use crate::stable_memory::MERCX_SETTINGS;


pub fn get() -> StableMercxSettings {
    MERCX_SETTINGS.with(|s| s.borrow().get().clone())
}

pub fn inc_token_map_idx() -> u32 {
    MERCX_SETTINGS.with(|s| {
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