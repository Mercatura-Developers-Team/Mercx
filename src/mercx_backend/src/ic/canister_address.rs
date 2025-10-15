

#[cfg(not(feature = "prod"))]
pub const MERCX_BACKEND: &str = "ahw5u-keaaa-aaaaa-qaaha-cai";

#[cfg(feature = "prod")]
pub const MERCX_BACKEND: &str ="zoa6c-riaaa-aaaan-qzmta-cai";

#[cfg(not(feature = "prod"))]
pub const KYC_CANISTER_ID: &str =  "ajuq4-ruaaa-aaaaa-qaaga-cai";

#[cfg(feature = "prod")]
pub const KYC_CANISTER_ID: &str =  "x2lku-6yaaa-aaaan-qzvia-cai";

#[cfg(not(feature = "prod"))]
pub const CANISTER_ID_XRC: &str =  "c5kvi-uuaaa-aaaaa-qaaia-cai";

#[cfg(feature = "prod")]
pub const CANISTER_ID_XRC: &str = "uf6dk-hyaaa-aaaaq-qaaaq-cai";


//cargo build --features staging
//cargo build => mainnet
//cargo build --release --features prod + put it on default 