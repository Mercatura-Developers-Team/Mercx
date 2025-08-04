pub const ICP_TOKEN_ID: u32 = 2;
pub const ICP_SYMBOL: &str = "ICP";
pub const ICP_SYMBOL_WITH_CHAIN: &str = "IC.ICP";
pub const ICP_ADDRESS: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
pub const ICP_ADDRESS_WITH_CHAIN: &str = "IC.ryjl3-tyaaa-aaaaa-aaaba-cai";

pub fn is_icp_token_id(token_id: u32) -> bool {
    token_id == ICP_TOKEN_ID
}
