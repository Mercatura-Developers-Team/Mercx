echo "===========SETUP========="
dfx start --background --clean

# Deploy the ICRC1 Ledger Canister with necessary arguments
export MINTER=$(dfx identity --identity minter get-principal)
export DEFAULT=$(dfx identity get-principal)
export FEATURE_FLAGS=true


#For local Deployment 

dfx deploy icrc1_ledger_canister  --argument "(variant { Init =
record {
     token_symbol = \"BELLA\";
     token_name = \"BELLA\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10000;
      fee_collector_account = opt record { owner = principal \"a3shf-5eaaa-aaaaa-qaafa-cai\" };
     metadata = vec {
         record { \"logo_url\"; variant { Text = \"/Bella.jpeg\" } };  
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

dfx deploy icrc1_index_canister --argument '(opt variant{Init = record {ledger_id = principal "b77ix-eeaaa-aaaaa-qaada-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 10}})'

dfx deploy  ckUSDT_ledger_canister  --argument "(variant { Init =
record {
     token_symbol = \"ckusdt\";
     token_name = \"ckusdt\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10000;
 fee_collector_account = opt record { owner = principal \"ufhzn-u7uzt-pxhz7-am66q-y4oi5-lnrur-4ubin-nab6f-db7ml-5h2cz-tqe\" };    
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

 dfx deploy  tommy_icrc1_ledger  --argument "(variant { Init =
record {
     token_symbol = \"TOMMY\";
     token_name = \"TOMMY\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10000;
 fee_collector_account = opt record { owner = principal \"ufhzn-u7uzt-pxhz7-am66q-y4oi5-lnrur-4ubin-nab6f-db7ml-5h2cz-tqe\" };    
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

dfx deploy tommy_icrc1_index --argument '(opt variant{Init = record {ledger_id = principal "aovwi-4maaa-aaaaa-qaagq-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 86400}})'

 dfx deploy  fxmx_icrc1_ledger  --argument "(variant { Init = record {
    token_symbol = \"FXMX\";
   token_name = \"FixedIncomeMercX\";
   minting_account = record { owner = principal \"${MINTER}\" };
    transfer_fee = 10000;
         fee_collector_account = opt record { owner = principal \"ufhzn-u7uzt-pxhz7-am66q-y4oi5-lnrur-4ubin-nab6f-db7ml-5h2cz-tqe\" }; 
     metadata = vec {
         record { \"icrc1:logo\"; variant { Text = \"data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyBpZD0ibG9nbyIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmlld0JveD0iMCAwIDE1MzYgNzY4Ij4KICA8ZGVmcz4KICAgIDxzdHlsZT4KICAgICAgLmNscy0xIHsKICAgICAgICBmaWxsOiB1cmwoI2xpbmVhci1ncmFkaWVudCk7CiAgICAgIH0KCiAgICAgIC5jbHMtMiB7CiAgICAgICAgZmlsbDogI2ZmZjsKICAgICAgfQogICAgPC9zdHlsZT4KICAgIDxsaW5lYXJHcmFkaWVudCBpZD0ibGluZWFyLWdyYWRpZW50IiB4MT0iLS4wMiIgeTE9IjM4NC44NyIgeDI9IjE1MzYuMjgiIHkyPSIzODQuODciIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIj4KICAgICAgPHN0b3Agb2Zmc2V0PSIwIiBzdG9wLWNvbG9yPSIjMWU5ZWI5Ii8+CiAgICAgIDxzdG9wIG9mZnNldD0iLjEyIiBzdG9wLWNvbG9yPSIjMTk4ZmE5Ii8+CiAgICAgIDxzdG9wIG9mZnNldD0iLjUyIiBzdG9wLWNvbG9yPSIjMGM2MzdjIi8+CiAgICAgIDxzdG9wIG9mZnNldD0iLjgyIiBzdG9wLWNvbG9yPSIjMDQ0ODYxIi8+CiAgICAgIDxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb2xvcj0iIzAxM2Y1NyIvPgogICAgPC9saW5lYXJHcmFkaWVudD4KICA8L2RlZnM+CiAgPHJlY3QgY2xhc3M9ImNscy0xIiB4PSItLjAyIiB5PSItLjAyIiB3aWR0aD0iMTUzNi4zIiBoZWlnaHQ9Ijc2OS43OCIvPgogIDxnPgogICAgPHBhdGggY2xhc3M9ImNscy0yIiBkPSJNMzYyLjQzLDMwNS4wMXY1MC44M2gxMjEuNTl2NDkuODdoLTEyMS41OXYxMDQuNTFoLTU0LjUzdi0yMjcuMzFjMC0zLjA4LjU3LTUuOTksMS43MS04LjczczIuNzEtNS4xNCw0LjcxLTcuMTljMi0yLjA2LDQuMzctMy42Niw3LjExLTQuOHM1LjctMS43MSw4LjktMS43MWwxNjAuNTUtMS4wNnY0NS41OWgtMTI4LjQ1WiIvPgogICAgPHBhdGggY2xhc3M9ImNscy0yIiBkPSJNNjA0Ljk5LDMyNy4ybDQ5LjE1LTY3Ljc4aDc3LjgybC05Mi43MywxMjguNTEsOTMuMTMsMTIyLjI5aC03NS41OWwtNDguMi04OC40NS03MS42MSw4OC40NWgtMTA1LjY0bDE0Mi4yMy0xMzQuNTItODMuOTItMTE2LjI4aDc0Ljc1bDQwLjYzLDY3Ljc4WiIvPgogICAgPHBhdGggY2xhc3M9ImNscy0yIiBkPSJNMTA5Ni43LDMzMS41Mmw1MS42OC03Mi4xaDYwLjE5bC03Mi43NiwxMTkuMjgsOTIuMjksMTMxLjUyaC03MC41OWwtNTQuODMtODUuMTktNTEuOTgsODUuMTloLTc4LjY0bDEwMi4yMy0xMzQuNTItODMuOTItMTE2LjI4aDc0Ljc1bDMxLjU5LDcyLjFaIi8+CiAgICA8cGF0aCBjbGFzcz0iY2xzLTIiIGQ9Ik05ODMuMiw1MTAuMjJoLTU4LjU5di0xNDkuODNsLTY0LjU3LDc5LjY0aC01LjY1bC02NC43NC03OS42NHYxNDkuODNoLTU5LjUzdi0yNTAuOGg1NC45MWw3Mi45Niw5MC4xOCw3My4zLTkwLjE4aDUxLjkxdjI1MC44WiIvPgogIDwvZz4KICA8Zz4KICAgIDxwYXRoIGNsYXNzPSJjbHMtMiIgZD0iTTEyMzguMjQsMjY0LjUzaC03Ljd2MTUuOTFoLTYuMzJ2LTE1LjkxaC03Ljd2LTUuMTFoMjEuNzN2NS4xMVoiLz4KICAgIDxwYXRoIGNsYXNzPSJjbHMtMiIgZD0iTTEyNjYuMzksMjgwLjQ0aC02LjM0di0xMi4xOGwtNS43Miw3LjA1aC0uNWwtNS43My03LjA1djEyLjE4aC02LjE2di0yMS4wMmg1Ljc1bDYuNDYsNy44OSw2LjQ5LTcuODloNS43NXYyMS4wMloiLz4KICA8L2c+Cjwvc3ZnPg==\" } };  
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
dfx deploy  fxmx_icrc1_index --argument '(opt variant{Init = record {ledger_id = principal "be2us-64aaa-aaaaa-qaabq-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 86400}})'

export MINTER_ACCOUNT_ID=$(dfx identity --identity minter get-principal)
export DEFAULT_ACCOUNT_ID=$(dfx identity get-principal)
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

dfx canister install icp_ledger_canister --mode reinstall --argument "(
  variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record { e8s = 20_000_000_000_000_000 : nat64 };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record { e8s = 10_000 : nat64 };
      token_symbol = opt \"ICP\";
      token_name = opt \"Local ICP\";
    }
  }
)"



#dfx deploy icp_index_canister --specified-id qhbym-qaaaa-aaaaa-aaafq-cai --argument '(record {ledger_id = principal "b77ix-eeaaa-aaaaa-qaada-cai"})'


echo "DONE"

#for candid
#cargo build --release --target wasm32-unknown-unknown --package mercx_backend
#candid-extractor target/wasm32-unknown-unknown/release/mercx_backend.wasm > mercx_backend.did