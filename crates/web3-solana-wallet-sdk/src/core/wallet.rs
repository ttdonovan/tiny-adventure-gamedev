#![allow(async_fn_in_trait)]
use emitter_rs::EventEmitter;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub enum Wallet {
    Local,
    Phantom,
    Solflare,
}

pub trait WalletAdapter {
    fn public_key(&self) -> Option<Pubkey>;

    fn connecting(&self) -> bool;

    fn connected(&self) -> bool {
        self.public_key().is_some()
    }

    async fn connect(&mut self) -> bool;
    async fn disconnect(&mut self) -> bool;
}

pub trait WalletAdapterEvents {
    fn emit_connect(&mut self, public_key: Pubkey);
    fn emit_disconnect(&mut self);
}

// #[cfg((target_arch = "wasm32"))]
pub mod browser {
    use std::str::FromStr;

    use super::*;
    use crate::adaptor::phantom;

    pub struct BrowserWallet {
        name: Wallet,
        public_key: Option<Pubkey>,
        connecting: bool,
        pub emitter: EventEmitter,
    }

    impl BrowserWallet {
        pub fn new(name: Wallet) -> Self {
            Self {
                name,
                public_key: None,
                connecting: false,
                emitter: EventEmitter::new(),
            }
        }
    }

    impl WalletAdapter for BrowserWallet {
        fn public_key(&self) -> Option<Pubkey> {
            self.public_key
        }

        fn connecting(&self) -> bool {
            self.connecting
        }

        async fn connect(&mut self) -> bool {
            log::info!("Connecting to {:?}", self.name);

            if self.connecting {
                return false;
            }

            self.connecting = true;

            let options = js_sys::Object::new();
            js_sys::Reflect::set(
                &options,
                &serde_wasm_bindgen::to_value("onlyIfTrusted").unwrap(),
                &serde_wasm_bindgen::to_value(&true).unwrap(),
            )
            .unwrap();

            let promise = match self.name {
                Wallet::Phantom => phantom::sign_in(&options),
                _ => unimplemented!(),
            };

            let result = JsFuture::from(promise).await;
            match result {
                Ok(resp) => {
                    log::info!("Wallet connected: {:?}", resp);

                    let key: JsValue = match self.name {
                        Wallet::Phantom => phantom::SOLANA.public_key(),
                        _ => unimplemented!(),
                    };

                    if key.is_undefined() {
                        log::error!("Public key is undefined");
                    } else {
                        let key_str: String = key.as_string().unwrap();
                        let public_key = Pubkey::from_str(&key_str).unwrap();
                        log::info!("Connected to wallet with public key: {:?}", public_key);
                        self.public_key = Some(public_key);

                        self.emit_connect(public_key);
                        self.connecting = false;
                    }
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }

            !self.connecting
        }

        async fn disconnect(&mut self) -> bool {
            let mut confirmed = false;

            let promise = match self.name {
                Wallet::Phantom => phantom::disconnect(),
                _ => unimplemented!(),
            };

            let result = JsFuture::from(promise).await;
            match result {
                Ok(resp) => {
                    log::info!("Wallet disconnected: {:?}", resp);
                    confirmed = true;
                    self.public_key = None;
                    self.emit_disconnect();
                }
                Err(e) => {
                    log::error!("{:?}", e);
                }
            }

            confirmed
        }
    }

    impl WalletAdapterEvents for BrowserWallet {
        fn emit_connect(&mut self, public_key: Pubkey) {
            self.emitter.emit("connect", public_key);
        }

        fn emit_disconnect(&mut self) {
            self.emitter.emit("disconnect", ());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod local {
    use crate::solana_sdk::{
        signature::{Keypair, Signer},
        signer::keypair::read_keypair_file,
    };

    use super::*;

    pub struct LocalWallet {
        keypair: Keypair,
        pubkey: Option<Pubkey>,
        pub emitter: EventEmitter,
    }

    impl std::fmt::Debug for LocalWallet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("LocalWallet")
                .field("pubkey", &self.pubkey)
                .finish()
        }
    }

    impl LocalWallet {
        pub fn read_keypair_file(path: &str) -> Self {
            let keypair = read_keypair_file(path).unwrap();
            Self {
                keypair,
                pubkey: None,
                emitter: EventEmitter::new(),
            }
        }

        pub fn pubkey(&self) -> Pubkey {
            self.keypair.pubkey()
        }
    }

    impl WalletAdapter for LocalWallet {
        fn public_key(&self) -> Option<Pubkey> {
            self.pubkey
        }

        fn connecting(&self) -> bool {
            false
        }

        async fn connect(&mut self) -> bool {
            let pubkey = self.keypair.pubkey();
            self.pubkey = Some(pubkey);
            self.emit_connect(pubkey);
            true
        }

        async fn disconnect(&mut self) -> bool {
            self.pubkey = None;
            self.emit_disconnect();
            true
        }
    }

    impl WalletAdapterEvents for LocalWallet {
        fn emit_connect(&mut self, public_key: Pubkey) {
            self.emitter.emit("connect", public_key);
        }

        fn emit_disconnect(&mut self) {
            self.emitter.emit("disconnect", ());
        }
    }
}
