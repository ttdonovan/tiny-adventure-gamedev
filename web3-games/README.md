# Blockchain/Web3 Games

An exploration of game development with blockchain technology.

## Crates

* macroquad
* solana_sdk

## Setup

```
rustup target add wasm32-unknown-unknown
cargo install basic-http-server
```

## Usage (WASM/Web3)

```
mkdir -p dist
cargo build --release -p web3-games --target wasm32-unknown-unknown  --bin 00_basic 

# -- 00 Basic
# cp .\target\wasm32-unknown-unknown\release\00_basic.wasm .\dist\.
# cp web3-games\index.html.basic dist\index.html
# edit dist/index.html

# -- 01 Wallet
# cargo build --release -p web3-games --target wasm32-unknown-unknown  --bin 01_wallet
# wasm-bindgen .\target\wasm32-unknown-unknown\release\01_wallet.wasm --out-dir dist --target web --no-typescript
# cp web3-games\index.html.plugin dist\index.html 

# shim to tie the thing together
# sed -i "s/import \* as __wbg_star0 from 'env';//" dist/"$PROJECT_NAME".js
# sed -i "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" dist/"$PROJECT_NAME".js
# sed -i "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" dist/"$PROJECT_NAME".js
# sed -i "s/const imports = __wbg_get_imports();/return __wbg_get_imports();/" dist/"$PROJECT_NAME".js

# cp .\tmp\01_wallet.js .\dist\01_wallet.js

basic-http-server dist
```

## Usage (Desktop)

```
cargo run -p web3-games --bin 01_wallet
```