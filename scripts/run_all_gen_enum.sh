#!/bin/bash
# run_all_gen_enum.sh: Run all enum generations for OpenFIGI API

# All runs:
./scripts/gen_enum.sh "https://api.openfigi.com/v3/mapping/values/exchCode" ExchCode >./scripts/exch_code.rs
./scripts/gen_enum.sh "https://api.openfigi.com/v3/mapping/values/micCode" MicCode >./scripts/mic_code.rs
./scripts/gen_enum.sh "https://api.openfigi.com/v3/mapping/values/currency" Currency >./scripts/currency.rs
./scripts/gen_enum.sh "https://api.openfigi.com/v3/mapping/values/marketSecDes" MarketSecDes >./scripts/market_sec_des.rs
./scripts/gen_enum.sh "https://api.openfigi.com/v3/mapping/values/securityType" SecurityType >./scripts/security_type.rs
./scripts/gen_enum.sh "https://api.openfigi.com/v3/mapping/values/securityType2" SecurityType2 >./scripts/security_type2.rs
./scripts/gen_enum.sh "https://api.openfigi.com/v3/mapping/values/stateCode" StateCode >./scripts/state_code.rs
