use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Memory IDs for different maps
const USERS_MEM_ID: MemoryId = MemoryId::new(0);
const USERNAMES_MEM_ID: MemoryId = MemoryId::new(1);

const MAX_VALUE_SIZE: u32 = 65536; // 64KB should be plenty for our structures

