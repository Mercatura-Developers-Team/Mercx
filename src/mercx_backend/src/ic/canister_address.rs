pub const MERCX_BACKEND: &str = if cfg!(feature = "staging") {
    "aovwi-4maaa-aaaaa-qaagq-cai"
} else {
    "zoa6c-riaaa-aaaan-qzmta-cai"
};

//cargo build --features staging
//cargo build => mainnet