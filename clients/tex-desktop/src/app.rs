use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::tari::{exchange::ActionsView, CoinsView, PoolsView, TransactionsView};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> View {
    let local_storage = window()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");
    view! {
        main(class="container") {
            div(class="row") {
                h1 {
                    "Welcome to "
                    a(href="https://tari.com", target="_blank") {
                        img(src="public/tari-logo.svg", class="logo tari", alt="Tari logo")
                    }
                    " Exchange 🪙"
                }
            }
            div(class="row") {
                section(class="column column-80") {
                    ActionsView()
                }
                aside(class="column column-20") {
                    //TODO: AccountInfo()
                    div(class="account-info") {
                        h4 { "Account Info" }
                        ul {
                            li { strong { "Name: " } "HumbleLiquidityProvider" }
                            li { strong { "Address: " } "component_ee782cca1916e53f5af32bb5015166a86291561c18bd2f959feccce6d0217f6d" }
                            li { strong { "Public Key: " } "76a3241081442c42c0a6bcad3c5fba3a26b683816fad9175cde3023cbc62f53f" }
                        }
                    }
                }
            }
            div(class="row") {
                aside(class="column column-20") {
                    CoinsView()
                }
                section(class="column column-80") {
                    div(class="pools") {
                        PoolsView()
                    }
                }
            }
            div(class="row") {
                aside(class="column column-100") {
                    div(class="transactions") {
                        TransactionsView()
                    }
                }
            }
        }
    }
}
