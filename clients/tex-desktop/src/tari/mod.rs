use serde_json::json;
use sycamore::{
    prelude::*,
    web::{create_client_resource, Resource},
};
use tari_exchange::{Exchange, LiquidityPool, TariCoin, TariTransaction, LIQUIDITY_PROVIDER};

use crate::app;

pub mod exchange;

pub async fn fetch_coins(name: String) -> Vec<TariCoin> {
    serde_wasm_bindgen::from_value(
        app::invoke(
            "coins_load",
            serde_wasm_bindgen::to_value(&json!({ "account_name": name})).unwrap(),
        )
        .await,
    )
    .unwrap()
}

pub async fn fetch_transactions(name: String) -> Vec<TariTransaction> {
    serde_wasm_bindgen::from_value(
        app::invoke(
            "transactions_load",
            serde_wasm_bindgen::to_value(&json!({ "account_name": name})).unwrap(),
        )
        .await,
    )
    .unwrap()
}

pub async fn fetch_exchange_state(name: String) -> Exchange {
    serde_wasm_bindgen::from_value(
        app::invoke(
            "exchange_state",
            serde_wasm_bindgen::to_value(&json!({ "account_name": name})).unwrap(),
        )
        .await,
    )
    .unwrap()
}

#[component]
pub fn CoinsView() -> View {
    //TODO: extract to properties
    let name = create_signal(String::from(LIQUIDITY_PROVIDER));
    //TODO: reload
    let coins = create_client_resource(on(name, move || async move {
        fetch_coins(name.get_clone()).await
    }));
    view! {
        div(class="token-list") {
            h3 { "Your Tokens" }
            TariCoinsListView(maybe_coins = coins)
        }
    }
}

#[component(inline_props)]
fn TariCoinView(model: TariCoin) -> View {
    let TariCoin { id, name, balance } = model;
    view! {
        li { (name) " = " (balance) b { (id.to_string()) } }
    }
}

#[component(inline_props)]
pub fn TariCoinsListView(maybe_coins: Resource<Vec<TariCoin>>) -> View {
    view! {
        (if let Some(coins) = maybe_coins.get_clone() {
            view! {
                ul {
                    Keyed(
                        list=coins,
                        view=move |coin| view! { TariCoinView(model=coin) },
                        key=|coin| coin.id.to_string(),
                    )
                }
            }
        } else {
            view! {
                p { ("Loading") }
            }
        })
    }
}

#[component]
pub fn TransactionsView() -> View {
    //TODO: extract to properties
    let name = create_signal(String::from(LIQUIDITY_PROVIDER));
    //TODO: reload
    let transactions = create_client_resource(on(name, move || async move {
        fetch_transactions(name.get_clone()).await
    }));
    view! {
        div(class="transactions-list") {
            h3 { "Recent Transactions" }
            TariTransactionsListView(maybe_transactions = transactions)
        }
    }
}

#[component(inline_props)]
fn TariTransactionView(model: TariTransaction) -> View {
    let TariTransaction {
        id,
        json,
        result,
        status,
        date_time,
    } = model;
    view! {
        tr {
            td { (id) }
            td { (date_time) }
            td { (status) }
        }
    }
}

#[component(inline_props)]
pub fn TariTransactionsListView(maybe_transactions: Resource<Vec<TariTransaction>>) -> View {
    view! {
        (if let Some(transactions) = maybe_transactions.get_clone() {
            view! {
                table {
                    thead {
                        tr {
                            th { "Id: " }
                            th { "Date: " }
                            th { "Status: " }
                        }
                    }
                    tbody {
                        Keyed(
                            list=transactions,
                            view=move |transaction| view! { TariTransactionView(model=transaction) },
                            key=|transaction| transaction.id.to_string(),
                        )
                    }
                }
            }
        } else {
            view! {
                p { ("Loading") }
            }
        })
    }
}

#[component]
pub fn PoolsView() -> View {
    //TODO: extract to properties
    let name = create_signal(String::from(LIQUIDITY_PROVIDER));
    //TODO: reload
    let exchange = create_client_resource(on(name, move || async move {
        fetch_exchange_state(name.get_clone()).await
    }));
    view! {
        div(class="exchange-state") {
            h3 { "Exchange State" }
            TariPoolsListView(maybe_exchange = exchange)
        }
    }
}

#[component(inline_props)]
fn TariPoolView(model: LiquidityPool) -> View {
    let LiquidityPool {
        a,
        b,
        lp_resource,
        fees_collected,
    } = model;
    view! {
        tr {
            td { (lp_resource.to_string()) }
            td { (a) }
            td { (b) }
            // td { (fees_collected.to_string()) }
        }
    }
}

#[component(inline_props)]
pub fn TariPoolsListView(maybe_exchange: Resource<Exchange>) -> View {
    view! {
        (if let Some(exchange) = maybe_exchange.get_clone() {
            view! {
                table {
                    thead {
                        tr {
                            th { "LP: " }
                            th { "A: " }
                            th { "B: " }
                            // th { "Fees: " }
                        }
                    }
                    tbody {
                        Keyed(
                            list=exchange.pools(),
                            view=move |transaction| view! { TariPoolView(model=transaction) },
                            key=|transaction| transaction.lp_resource.to_string(),
                        )
                    }
                }
            }
        } else {
            view! {
                p { ("Loading") }
            }
        })
    }
}
