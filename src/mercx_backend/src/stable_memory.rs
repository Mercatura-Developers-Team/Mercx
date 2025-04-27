
use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};
use std::cell::RefCell;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;


type Memory = VirtualMemory<DefaultMemoryImpl>;
