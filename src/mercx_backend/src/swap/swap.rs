use ic_cdk::update;

use super::swap_args::SwapArgs;
use super::swap_reply::SwapReply;
use super::swap_transfer::{swap_transfer};
use super::swap_transfer_from::{swap_transfer_from};


/// Pay and Receive are from the user's perspective
/// Swap tokens
#[update]
pub async fn swap_tokens(args: SwapArgs) -> Result<SwapReply, String> {
    // determine if using icrc2_approve+icrc2_transfer_from or icrc1_transfer method
    match args.pay_tx_id {
        None => swap_transfer_from(args).await,
        Some(_) => swap_transfer(args).await,
    }
}

