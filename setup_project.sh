#!/usr/bin/env bash


echo "===========SETUP========="
dfx start --background --clean

# Deploy the ICRC1 Ledger Canister with necessary arguments
export MINTER=$(dfx identity --identity minter get-principal)
export DEFAULT=$(dfx identity get-principal)
export FEATURE_FLAGS=true

 dfx canister install fxmx_icrc1_ledger --mode=reinstall --network ic --argument "(variant { Init = record {
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
 
# dfx canister --network ic install icrc1_ledger_canister --mode reinstall --argument "(variant { Init =
 dfx canister install icrc1_ledger_canister --mode=reinstall --network ic --argument "(variant { Init = record {
    token_symbol = \"BELLA\";
   token_name = \"BELLA\";
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

dfx deploy icrc1_ledger_canister  --argument "(variant { Init =
record {
     token_symbol = \"BELLA\";
     token_name = \"BELLA\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10000;
      fee_collector_account = opt record { owner = principal \"a3shf-5eaaa-aaaaa-qaafa-cai\" };
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

export TOKEN_LOGO="data:image/png;base64,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"

dfx deploy ckUSDT_ledger_canister  --argument "(variant { Init =
record {
     token_symbol = \"ckUSDT\";
     token_name = \"ckUSDT\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = 10000;
      fee_collector_account = opt record { owner = principal \"a3shf-5eaaa-aaaaa-qaafa-cai\" };
     metadata = vec {
         record { \"logo_url\"; variant { Text = \"${TOKEN_LOGO}\" }; };  
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

# Mainnet 
# dfx deploy icrc1_index_canister --argument '(opt variant{Init = record {ledger_id = principal "bkyz2-fmaaa-aaaaa-qaaaq-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 10}})'
dfx deploy icrc1_index_canister --argument '(opt variant{Init = record {ledger_id = principal "bkyz2-fmaaa-aaaaa-qaaaq-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 10}})'

dfx deploy --network ic fxmx_icrc1_index --argument '(opt variant{Init = record {ledger_id = principal "b7p2k-giaaa-aaaan-qzwta-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 86400}})'


dfx canister install tommy_icrc1_ledger --mode reinstall --network ic --argument "(variant { Init =
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

dfx deploy tommy_icrc1_index --argument '(opt variant{Init = record {ledger_id = principal "br5f7-7uaaa-aaaaa-qaaca-cai"; retrieve_blocks_from_ledger_interval_seconds = opt 86400}})'

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

dfx deploy icp_index_canister --specified-id qhbym-qaaaa-aaaaa-aaafq-cai --argument '(record {ledger_id = principal "b77ix-eeaaa-aaaaa-qaada-cai"})'



echo "DONE"