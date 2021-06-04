#!/usr/bin/env sh

./target/release/crust-collator build-spec --raw --disable-default-bootnode --chain 2001 > ./crust-collator/res/2001.json
./target/release/crust-collator build-spec --raw --disable-default-bootnode --chain 2008 > ./crust-collator/res/2008.json

sed -i "" "s/\"protocolId\": null/\"protocolId\": \"2001\"/g" ./crust-collator/res/2001.json
sed -i "" "s/\"name\": \"Local Testnet\"/\"name\": \"2001\"/g" ./crust-collator/res/2001.json
sed -i "" "s/\"id\": \"local_testnet\"/\"id\": \"2001\"/g" ./crust-collator/res/2001.json
sed -i "" "s/\"properties\": null,/\"properties\": {\"ss58Format\": 42, \"tokenDecimals\": 12, \"tokenSymbol\": \"2001\"},/g" ./crust-collator/res/2001.json

sed -i "" "s/\"protocolId\": null/\"protocolId\": \"2008\"/g" ./crust-collator/res/2008.json
sed -i "" "s/\"name\": \"Local Testnet\"/\"name\": \"2008\"/g" ./crust-collator/res/2008.json
sed -i "" "s/\"id\": \"local_testnet\"/\"id\": \"2008\"/g" ./crust-collator/res/2008.json
sed -i "" "s/\"properties\": null,/\"properties\": {\"ss58Format\": 42, \"tokenDecimals\": 12, \"tokenSymbol\": \"2008\"},/g" ./crust-collator/res/2008.json

./target/release/crust-collator export-genesis-state --chain ./crust-collator/res/2008.json --parachain-id 2008  > crust-collator-state-2008
./target/release/crust-collator export-genesis-state --chain ./crust-collator/res/2001.json --parachain-id 2001 > crust-collator-state-2001
./target/release/crust-collator export-genesis-wasm --chain ./crust-collator/res/2008.json > crust-collator-wasm-2008
./target/release/crust-collator export-genesis-wasm --chain ./crust-collator/res/2001.json > crust-collator-wasm-2001