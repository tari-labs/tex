use serde_json::json;
use sycamore::prelude::*;

use crate::app;

pub async fn add_liquidity(
    a_resource_address_str: String,
    a_amount: f64,
    b_resource_address_str: String,
    b_amount: f64,
) -> String {
    serde_wasm_bindgen::from_value(
        app::invoke(
            "add_liquidity",
            serde_wasm_bindgen::to_value(&json!(
               {
                   "a_resource_address_str": a_resource_address_str,
                   "a_amount": a_amount,
                   "b_resource_address_str": b_resource_address_str,
                   "b_amount": b_amount,
                }
            ))
            .unwrap(),
        )
        .await,
    )
    .unwrap()
}

pub async fn remove_liquidity(lp_resource_address_str: String, lp_amount: f64) -> String {
    serde_wasm_bindgen::from_value(
        app::invoke(
            "remove_liquidity",
            serde_wasm_bindgen::to_value(&json!(
               {
                   "lp_resource_address_str": lp_resource_address_str,
                   "lp_amount": lp_amount,
                }
            ))
            .unwrap(),
        )
        .await,
    )
    .unwrap()
}

pub async fn swap(
    a_resource_address_str: String,
    a_amount: f64,
    b_resource_address_str: String,
) -> String {
    serde_wasm_bindgen::from_value(
        app::invoke(
            "swap",
            serde_wasm_bindgen::to_value(&json!(
               {
                   "a_resource_address_str": a_resource_address_str,
                   "a_amount": a_amount,
                   "b_resource_address_str": b_resource_address_str,
                }
            ))
            .unwrap(),
        )
        .await,
    )
    .unwrap()
}

#[component]
pub fn ActionsView() -> View {
    let selected_menu_item = create_signal(None::<String>);
    let show_popup = create_signal(false);
    let open_popup = move |menu_item: &str| {
        selected_menu_item.set(Some(menu_item.to_string()));
        show_popup.set(true);
    };
    let close_popup = move || {
        show_popup.set(false);
        selected_menu_item.set(None);
    };
    view! {
        div(class="actions") {
            button(on:click=move |_| open_popup("add")) { "Add Liquidity" }
            button(on:click=move |_| open_popup("remove")) { "Remove Liquidity" }
            button(on:click=move |_| open_popup("swap")) { "Execute Swap" }
        }
         (if show_popup.get() {
            if let Some(menu_item) = selected_menu_item.get_clone() {
                match menu_item.as_str() {
                    "add" =>
                        view! {
                            div(class="popup") {
                                div(class="popup-content") {
                                    h3 { (selected_menu_item.get_clone().unwrap_or(String::from("UNKNOWN"))) }
                                    AddLiquidityPopupView()
                                    button(on:click=move |_| close_popup(), class="close-button") { "Close" }
                                }
                            }
                        },
                    "remove" =>
                        view! {
                            div(class="popup") {
                                div(class="popup-content") {
                                    h3 { (selected_menu_item.get_clone().unwrap_or(String::from("UNKNOWN"))) }
                                    RemoveLiquidityPopupView()
                                    button(on:click=move |_| close_popup(), class="close-button") { "Close" }
                                }
                            }
                        },
                    "swap" =>
                         view! {
                            div(class="popup") {
                                div(class="popup-content") {
                                    h3 { (selected_menu_item.get_clone().unwrap_or(String::from("UNKNOWN"))) }
                                    SwapPopupView()
                                    button(on:click=move |_| close_popup(), class="close-button") { "Close" }
                                }
                            }
                        },
                        _ => view! {}
                    }
                }
                else { view! {} }
        } else {
            view! {}
        })
    }
}

#[component]
pub fn AddLiquidityPopupView() -> View {
    let a_resource_address_str = create_signal(String::new());
    let a_amount = create_signal(0_f64);
    let b_resource_address_str = create_signal(String::new());
    let b_amount = create_signal(0_f64);
    let transaction_id = create_signal(String::new());
    let handle_add_liquidity = move |_| async move {
        transaction_id.set(
            add_liquidity(
                a_resource_address_str.get_clone(),
                a_amount.get_clone(),
                b_resource_address_str.get_clone(),
                b_amount.get_clone(),
            )
            .await,
        );
    };
    view! {
        input(id="a_resource_address_str",bind:value=a_resource_address_str,placeholder="A")
        input(id="a_amount",r#type="number", min="1", step="1", max="1000", bind:valueAsNumber=a_amount)
        input(id="b_resource_address_str",bind:value=b_resource_address_str,placeholder="B")
        input(id="b_amount",r#type="number", min="1", step="1", max="1000", bind:valueAsNumber=b_amount)
        button(on:click=handle_add_liquidity) { "Add" }
        p { (transaction_id) }
    }
}

#[component]
pub fn RemoveLiquidityPopupView() -> View {
    let lp_resource_address_str = create_signal(String::new());
    let lp_amount = create_signal(0_f64);
    let transaction_id = create_signal(String::new());
    let handle_remove_liquidity = move |_| async move {
        transaction_id.set(
            remove_liquidity(lp_resource_address_str.get_clone(), lp_amount.get_clone()).await,
        );
    };
    view! {
        input(id="lp_resource_address_str",bind:value=lp_resource_address_str,placeholder="LP")
        input(id="lp_amount",r#type="number", min="1", step="1", max="1000", bind:valueAsNumber=lp_amount)
        button(on:click=handle_remove_liquidity) { "Remove" }
        p { (transaction_id) }
    }
}

#[component]
pub fn SwapPopupView() -> View {
    let a_resource_address_str = create_signal(String::new());
    let a_amount = create_signal(0_f64);
    let b_resource_address_str = create_signal(String::new());
    let transaction_id = create_signal(String::new());
    let handle_swap = move |_| async move {
        transaction_id.set(
            swap(
                a_resource_address_str.get_clone(),
                a_amount.get_clone(),
                b_resource_address_str.get_clone(),
            )
            .await,
        );
    };
    view! {
        input(id="a_resource_address_str",bind:value=a_resource_address_str,placeholder="A")
        input(id="a_amount",r#type="number", min="1", step="1", max="1000", bind:valueAsNumber=a_amount)
        input(id="b_resource_address_str",bind:value=b_resource_address_str,placeholder="B")
        button(on:click=handle_swap) { "Swap" }
        p { (transaction_id) }
    }
}
