pub const MERCX_BACKEND: &str = if cfg!(feature = "staging") {
    "ahw5u-keaaa-aaaaa-qaaha-cai"
} else {
    "zoa6c-riaaa-aaaan-qzmta-cai"
};

pub const KYC_CANISTER_ID: &str = if cfg!(feature = "staging") {
    "ajuq4-ruaaa-aaaaa-qaaga-cai"
} else {
    "x2lku-6yaaa-aaaan-qzvia-cai"
};

pub const CANISTER_ID_XRC: &str = if cfg!(feature = "staging") {
    "c5kvi-uuaaa-aaaaa-qaaia-cai"
} else {
    "uf6dk-hyaaa-aaaaq-qaaaq-cai"
};


//cargo build --features staging
//cargo build => mainnet
