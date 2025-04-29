use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;


use crate::pool::stable_pool::{StablePool, StablePoolId};


type Memory = VirtualMemory<DefaultMemoryImpl>;

//stable memory
pub const POOL_MEMORY_ID: MemoryId = MemoryId::new(0);

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static POOLS: RefCell<StableBTreeMap<StablePoolId, StablePool, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(POOL_MEMORY_ID)))
    );
  
}