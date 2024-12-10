build-web3-game-wallet:
    cargo build --release -p web3-games --target wasm32-unknown-unknown --bin 01_wallet
    wasm-bindgen target/wasm32-unknown-unknown/release/01_wallet.wasm --out-dir dist --target web --no-typescript
    sed -i "s/import \* as __wbg_star0 from 'env';//" dist/01_wallet.js
    sed -i "s/let wasm;/let wasm; export const set_wasm = (w) => wasm = w;/" dist/01_wallet.js
    sed -i "s/imports\['env'\] = __wbg_star0;/return imports.wbg\;/" dist/01_wallet.js
    sed -i "s/const imports = __wbg_get_imports();/return __wbg_get_imports();/" dist/01_wallet.js