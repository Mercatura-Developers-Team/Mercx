use std::env;

use dotenv::dotenv;

fn main() {
    dotenv().ok(); // Load .env file at compile time

    let ledger_canister_id = env::var("CANISTER_ID_ICRC1_LEDGER_CANISTER")
        .expect("CANISTER_ID_ICRC1_LEDGER_CANISTER must be set");

    println!("cargo:rerun-if-env-changed=CANISTER_ID_ICRC1_LEDGER_CANISTER");
    println!("Ledger Canister ID: {}", ledger_canister_id);
}