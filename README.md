### 1. Make sure you have a NEAR node running locally
```
# clone the project
$ git clone https://github.com/near/nearcore

# compile the project
$ cd nearcore && make neard

# remove previous credentials
$ rm -rf ~/.near

# initialize credentials for the node
$ ./target/release/neard --home ~/.near init --chain-id localnet # it will create the ~/.near directory

# run the node
$ ./target/release/neard --home ~/.near run
```

### 2. Compile the smart contracts

```
$ ./build.sh
```

### 3. Deploy and call the smart contracts

```
# remove previous credentials
$ rm -rf ~/.near-credentials

# we are deploying locally
$ export NEAR_ENV=local

$ near create-account hc_token.test.near --master-account=test.near # hc_token.test.near represents the address of the smart contract
$ near deploy --accountId=hc_token.test.near --wasmFile=./res/hc_token.wasm
$ near call hc_token.test.near initialize "{\"owner_id\": \"test.near\", \"total_supply\": \"1000\"}" --accountId=test.near
$ near call hc_token.test.near ft_total_supply --accountId=test.near
$ near call hc_token.test.near ft_balance_of "{\"account_id\": \"test.near\"}" --accountId=test.near

$ near create-account hc_wallet.test.near --master-account=test.near # hc_wallet.test.near represents the address of the smart contract
$ near deploy --accountId=hc_wallet.test.near --wasmFile=./res/hc_wallet.wasm
$ near call hc_wallet.test.near initialize "{\"erc20_token\": \"hc_token.test.near\", \"controller\": \"hc_token.test.near\", \"user_id\": \"hc_token.test.near\"}" --accountId=test.near
$ near call hc_wallet.test.near get_version_number --accountId=test.near
$ near call hc_wallet.test.near available_balance --accountId=test.near
```
