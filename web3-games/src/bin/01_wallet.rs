use macroquad::prelude::*;

use web3_solana_wallet_sdk::{
    core::wallet::{self, WalletAdapter},
    solana_sdk::pubkey::Pubkey,
    WasmClient,
};

const PATH_TO_KEYPAIR: &str = "tmp/id-hack-me.json";
// replace with your own Solana RPC endpoint
const SOLANA_RPC_ENDPOINT: &str = "https://api.devnet.solana.com";

#[macroquad::main("Wallet")]
async fn main() {
    if cfg!(target_arch = "wasm32") {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());
    }

    log::info!("01 Wallet");

    #[cfg(target_arch = "wasm32")]
    let mut wallet = wallet::browser::BrowserWallet::new(wallet::Wallet::Phantom);

    #[cfg(not(target_arch = "wasm32"))]
    let mut wallet = wallet::local::LocalWallet::read_keypair_file(PATH_TO_KEYPAIR);

    wallet.emitter.on("connect", |public_key: Pubkey| {
        log::info!("emitter on 'connect': {:?}", public_key);
    });

    wallet.emitter.on("disconnect", |_: ()| {
        log::info!("emitter on 'disconnect'");
    });

    let mut is_shown_balance = false;
    let mut balance = 0;
    let mut block_time = 0;

    loop {
        clear_background(DARKPURPLE);
        let dt = get_frame_time();

        draw_text(&format!("Delta Time: {:.4}", dt), 20.0, 20.0, 20.0, WHITE);

        if wallet.connected() {
            let pubkey = wallet.public_key().unwrap();

            draw_text(&format!("Connected: {}", pubkey), 20.0, 40.0, 20.0, WHITE);

            draw_text(&format!("Disconnect. Press 'D'"), 20.0, 60.0, 20.0, WHITE);

            if is_shown_balance {
                draw_text(&format!("Balance (lamports): {}", balance), 20.0, 80.0, 20.0, WHITE);
                draw_text(&format!("Block Time: {}", block_time), 20.0, 100.0, 20.0, WHITE);
            } else {
                let client = WasmClient::new(SOLANA_RPC_ENDPOINT);
                balance = client.get_balance(&pubkey).await.unwrap();

                let slot = client.get_slot().await.unwrap();
                block_time = client.get_block_time(slot).await.unwrap();

                is_shown_balance = true;
            }

            if is_key_pressed(KeyCode::D) {
                let _ = wallet.disconnect().await;
            }
        } else {
            draw_text("Let's connect a Wallet! Press 'C'", 20.0, 40.0, 20.0, WHITE);

            if is_key_pressed(KeyCode::C) {
                is_shown_balance = false;
                let _ = wallet.connect().await;
            }
        }

        next_frame().await
    }
}
