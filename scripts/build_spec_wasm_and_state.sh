#!/usr/bin/env sh

./target/release/crust-collator build-spec --raw --disable-default-bootnode --chain 2001 > ./crust-collator/res/2001.json
./target/release/crust-collator build-spec --raw --disable-default-bootnode --chain 2012 > ./crust-collator/res/2012.json

sed -i "" "s/\"protocolId\": null/\"protocolId\": \"2001\"/g" ./crust-collator/res/2001.json
sed -i "" "s/\"name\": \"Local Testnet\"/\"name\": \"2001\"/g" ./crust-collator/res/2001.json
sed -i "" "s/\"id\": \"local_testnet\"/\"id\": \"2001\"/g" ./crust-collator/res/2001.json
sed -i "" "s/\"properties\": null,/\"properties\": {\"ss58Format\": 42, \"tokenDecimals\": 12, \"tokenSymbol\": \"2001\"},/g" ./crust-collator/res/2001.json

sed -i "" "s/\"protocolId\": null/\"protocolId\": \"2012\"/g" ./crust-collator/res/2012.json
sed -i "" "s/\"name\": \"Local Testnet\"/\"name\": \"2012\"/g" ./crust-collator/res/2012.json
sed -i "" "s/\"id\": \"local_testnet\"/\"id\": \"2012\"/g" ./crust-collator/res/2012.json
sed -i "" "s/\"properties\": null,/\"properties\": {\"ss58Format\": 42, \"tokenDecimals\": 12, \"tokenSymbol\": \"2012\"},/g" ./crust-collator/res/2012.json

./target/release/crust-collator export-genesis-state --chain ./crust-collator/res/2012.json --parachain-id 2012  > crust-collator-state-2012
./target/release/crust-collator export-genesis-state --chain ./crust-collator/res/2001.json --parachain-id 2001 > crust-collator-state-2001
./target/release/crust-collator export-genesis-wasm --chain ./crust-collator/res/2012.json > crust-collator-wasm-2012
./target/release/crust-collator export-genesis-wasm --chain ./crust-collator/res/2001.json > crust-collator-wasm-2001