pub const MERCX_BACKEND: &str = if cfg!(feature = "staging") {
    "a3shf-5eaaa-aaaaa-qaafa-cai"
} else {
    "zoa6c-riaaa-aaaan-qzmta-cai"
};

//cargo build --features staging
//cargo build => mainnet