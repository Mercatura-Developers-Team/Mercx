use candid::{CandidType, Deserialize, Principal};
//use crate::get_icp_rate_in_cents;
use ic_cdk::update;
use crate::general::CANISTER_ID_XRC;

#[derive(CandidType, Deserialize, Debug)]
pub struct Metadata {
    decimals: u32,
    forex_timestamp: Option<u64>,
    quote_asset_num_received_rates: u64,
    base_asset_num_received_rates: u64,
    base_asset_num_queried_sources: u64,
    standard_deviation: u64,
    quote_asset_num_queried_sources: u64,
}
#[derive(CandidType, Deserialize, Debug)]
pub struct ExchangeRateResponse {
    metadata: Metadata,
    rate: u64,
    timestamp: u64,
    quote_asset: Asset,
    base_asset: Asset,
}
#[derive(CandidType, Deserialize, Debug)]
pub enum AssetClass {
    Cryptocurrency,
    FiatCurrency,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Asset {
    class: AssetClass,
    symbol: String,
}

#[derive(CandidType, Deserialize,Debug)]
pub enum ExchangeRateError {
    AnonymousPrincipalNotAllowed,
    Pending,
    CryptoBaseAssetNotFound,
    CryptoQuoteAssetNotFound,
    StablecoinRateNotFound,
    StablecoinRateTooFewRates,
    StablecoinRateZeroRate,
    ForexInvalidTimestamp,
    ForexBaseAssetNotFound,
    ForexQuoteAssetNotFound,
    ForexAssetsNotFound,
    RateLimited,
    NotEnoughCycles,
    FailedToAcceptCycles,
    InconsistentRatesReceived,
    Other {
        code: u32,
        description: String,
    },
}


#[derive(CandidType, Deserialize,Debug)]
pub enum XRCResponse {
    Ok(ExchangeRateResponse),
    Err(ExchangeRateError),
}
#[derive(CandidType, Deserialize)]
pub struct GetExchangeRateRequest {
    base_asset: Asset,
    quote_asset: Asset,
    timestamp: Option<u64>,
}

#[update]
pub async fn get_icp_rate() -> Result<f64, String> {
    let request: GetExchangeRateRequest = GetExchangeRateRequest {
        base_asset: Asset {
            symbol: "ICP".to_string(),
            class: AssetClass::Cryptocurrency,
        },
        quote_asset: Asset {
            symbol: "USD".to_string(),
            class: AssetClass::FiatCurrency,
        },
        timestamp: None,
    };

    let xrc_canister_id = Principal::from_text(CANISTER_ID_XRC).unwrap();

    let call_result: Result<Vec<u8>, (ic_cdk::api::call::RejectionCode, String)> =
        ic_cdk::api::call::call_raw(
            xrc_canister_id,
            "get_exchange_rate",
            &candid::encode_args((request,)).unwrap(),
            10_000_000_000// payment fee
        )
        .await;

        match call_result {
            Ok(response_bytes) => match candid::decode_one::<XRCResponse>(&response_bytes) {
                Ok(response) => {
                    println!("Decoded response: {:?}", response);
                    match response {
                        XRCResponse::Ok(exchange_rate) => {
                            //Calculate the float rate in a way that mimics the Motoko handling of decimals
                            let divisor = 10f64.powi(exchange_rate.metadata.decimals as i32);
                            let float_rate = (exchange_rate.rate as f64) / divisor;
                            println!("Calculated rate: {}", float_rate);
                            Ok(float_rate)
                        },
                        XRCResponse::Err(err) => {
                            ic_cdk::println!("Error in XRC response: {:?}", err);
                            Err("Error in XRC response".to_string())
                        }
                    }
                },
                Err(_) => {
                    Err("Failed to decode response".to_string())
                }
            },
            Err((_rejection_code, msg)) => {
                ic_cdk::println!("Call rejected: {}", msg);
                Err("Error call rejected".to_string())
            }
        }
        
}
