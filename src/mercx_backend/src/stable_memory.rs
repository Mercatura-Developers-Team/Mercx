use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;

use crate::transfers::stable_transfer::{StableTransferId,StableTransfer};

use candid::Principal;

use crate::pool::stable_pool::{StablePool, StablePoolId};
use crate::token::stable_token::{StableTokenId, StableToken};
use crate::stable_mercx_settings::stable_mercx_settings::StableMercxSettings;

use crate::lp_metadata::stable_lp_metadata::{StableLpMetadataId,LPToken};

use crate::stable_lp_token::stable_lp_token::{StableLPToken,StableLPTokenId};

type Memory = VirtualMemory<DefaultMemoryImpl>;

//stable memory
pub const POOL_MEMORY_ID: MemoryId = MemoryId::new(0);
pub const TOKEN_MEMORY_ID: MemoryId = MemoryId::new(1);
pub const MERCX_SETTINGS_MEMORY_ID: MemoryId = MemoryId::new(3);
pub const TRANSFER_MEMORY_ID: MemoryId = MemoryId::new(4);
pub const WHITELIST_MEM_ID: MemoryId = MemoryId::new(5);
pub const LP_METADATA_MEM_ID: MemoryId = MemoryId::new(6);
pub const LP_TOKEN_MEMORY_ID: MemoryId = MemoryId::new(7);



thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static POOLS: RefCell<StableBTreeMap<StablePoolId, StablePool, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(POOL_MEMORY_ID)))
    );

    pub static TOKENS: RefCell<StableBTreeMap<StableTokenId, StableToken, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(TOKEN_MEMORY_ID)))
    );

    pub static MERCX_SETTINGS: RefCell<StableCell<StableMercxSettings, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MERCX_SETTINGS_MEMORY_ID)),
            StableMercxSettings::default()
        ).expect("Failed to initialize mercx settings")
    );


    pub static TRANSFERS: RefCell<StableBTreeMap<StableTransferId, StableTransfer, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(TRANSFER_MEMORY_ID)))
    );

    pub static WHITELIST: RefCell<StableBTreeMap<Principal, bool, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(WHITELIST_MEM_ID)))
    );
    

    pub static LPMETADATA: RefCell<StableBTreeMap<StableLpMetadataId, LPToken, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(LP_METADATA_MEM_ID)))
    );

    pub static LP_TOKEN_MAP: RefCell<StableBTreeMap<StableLPTokenId, StableLPToken, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(LP_TOKEN_MEMORY_ID)))
    );
  
}