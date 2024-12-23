#!/usr/bin/env bash


echo "===========SETUP========="
dfx start --background --clean

# Deploy the ICRC1 Ledger Canister with necessary arguments
export MINTER=$(dfx identity --identity minter get-principal)
export DEFAULT=$(dfx identity get-principal)
export FEATURE_FLAGS=true
 
# dfx canister --network ic install icrc1_ledger_canister --mode reinstall --argument "(variant { Init =
dfx deploy  icrc1_ledger_canister  --argument "(variant { Init =
record {
     token_symbol = \"BELLA\";
     token_name = \"BELLA\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10000;
     metadata = vec {
         record { \"logo_url\"; variant { Text = \"/j.png\" } };  
     };

     feature_flags = opt record{icrc2 = ${FEATURE_FLAGS}};
     initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; 100_000_000_000_000; }; };
     archive_options = record {
         num_blocks_to_archive = 1000;
         trigger_threshold = 2000;
         controller_id = principal \"${MINTER}\";
     };
 }
})"

dfx deploy icrc1_index_canister --argument '(opt variant{Init = record {ledger_id = principal "bkyz2-fmaaa-aaaaa-qaaaq-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 10}})'

export MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)
dfx deploy icp_ledger_canister --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 10_000_000_000 : nat64;
          };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record {
        e8s = 10_000 : nat64;
      };
      token_symbol = opt \"ICP\";
      token_name = opt \"Local ICP\";
    }
  })
"

dfx deploy icp_index_canister --specified-id qhbym-qaaaa-aaaaa-aaafq-cai --argument '(record {ledger_id = principal "be2us-64aaa-aaaaa-qaabq-cai"})'



echo "DONE"