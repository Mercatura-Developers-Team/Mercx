pub const MERCX_BACKEND: &str = if cfg!(feature = "staging") {
    "ahw5u-keaaa-aaaaa-qaaha-cai"
} else {
    "zoa6c-riaaa-aaaan-qzmta-cai"
};

//cargo build --features staging
//cargo build => mainnet