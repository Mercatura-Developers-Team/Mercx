use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

use crate::models::user::User;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Memory IDs for different maps
const USERS_MEM_ID: MemoryId = MemoryId::new(0);
const USERNAMES_MEM_ID: MemoryId = MemoryId::new(1);

const MAX_VALUE_SIZE: u32 = 65536; // 64KB should be plenty for our structures

// Add this near the top with other types
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserIdList(pub Vec<u64>);

#[derive(Debug, Clone)]
pub struct StorableString(pub String);

impl Storable for StorableString {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(Decode!(bytes.as_ref(), String).unwrap())
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Debug, Clone)]
pub struct StorablePrincipal(pub Principal);

impl Storable for StorablePrincipal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(Decode!(bytes.as_ref(), Principal).unwrap())
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: 29,
        is_fixed_size: false,
    };
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static USERS: RefCell<StableBTreeMap<Principal, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(USERS_MEM_ID)))
    );

    pub static USERNAMES: RefCell<StableBTreeMap<String, Principal, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(USERNAMES_MEM_ID)))
    );

}

// Implement Storable for our types
impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

impl Storable for UserIdList {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: ic_stable_structures::storable::Bound = ic_stable_structures::storable::Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}