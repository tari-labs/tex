use std::str::FromStr as _;

use tari_all_in_one_rust_sdk::{indexer, wallet_daemon};
use tari_exchange::{
    Exchange, LIQUIDITY_PROVIDER, TEX_COMPONENT_INSTANCE_ADDRESS_STR, TEX_TEMPLATE_HEX,
};
use tari_template_lib::{
    args,
    prelude::{Amount, ComponentAddress, ResourceAddress, TemplateAddress},
};
use tari_transaction::Transaction;
use tari_wallet_daemon_client::ComponentAddressOrName;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_liquidity(
    a_resource_address_str: String,
    a_amount: f64,
    b_resource_address_str: String,
    b_amount: f64,
) -> Result<String, ()> {
    let a_resource_address = ResourceAddress::from_str(&a_resource_address_str).unwrap();
    let b_resource_address = ResourceAddress::from_str(&b_resource_address_str).unwrap();
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await.unwrap();
    let tex_component_instance_address =
        ComponentAddress::from_str(TEX_COMPONENT_INSTANCE_ADDRESS_STR).unwrap();
    let liquidity_provider_account_name = LIQUIDITY_PROVIDER.to_string();
    //TODO: would be nice to store in local storage or somewhere in backend?
    let liquidity_provider_account = wallet_daemon_client
        .accounts_get(ComponentAddressOrName::Name(
            liquidity_provider_account_name.clone(),
        ))
        .await
        .unwrap()
        .account;
    let liquidity_provider_address =
        ComponentAddress::try_from(liquidity_provider_account.address).unwrap();
    let transaction_id = wallet_daemon::transaction_call(
        liquidity_provider_account.key_index,
        Transaction::builder()
            .fee_transaction_pay_from_component(liquidity_provider_address, Amount(1000))
            .call_method(
                liquidity_provider_address,
                "withdraw",
                args![a_resource_address, a_amount.round() as i64],
            )
            .put_last_instruction_output_on_workspace("a")
            .call_method(
                liquidity_provider_address,
                "withdraw",
                args![b_resource_address, b_amount.round() as i64],
            )
            .put_last_instruction_output_on_workspace("b")
            .call_method(
                tex_component_instance_address,
                "add_liquidity",
                args![Workspace("a"), Workspace("b")],
            )
            .put_last_instruction_output_on_workspace("liquidity_provided_token")
            .call_method(
                liquidity_provider_address,
                "deposit",
                args![Workspace("liquidity_provided_token")],
            )
            .build_unsigned_transaction(),
        &mut wallet_daemon_client,
    )
    .await
    .unwrap();
    println!("{}", transaction_id);
    Ok(transaction_id.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn remove_liquidity(
    lp_resource_address_str: String,
    lp_amount: f64,
) -> Result<String, ()> {
    let lp_resource_address = ResourceAddress::from_str(&lp_resource_address_str).unwrap();
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await.unwrap();
    let tex_component_instance_address =
        ComponentAddress::from_str(TEX_COMPONENT_INSTANCE_ADDRESS_STR).unwrap();
    let liquidity_provider_account_name = LIQUIDITY_PROVIDER.to_string();
    //TODO: would be nice to store in local storage or somewhere in backend?
    let liquidity_provider_account = wallet_daemon_client
        .accounts_get(ComponentAddressOrName::Name(
            liquidity_provider_account_name.clone(),
        ))
        .await
        .unwrap()
        .account;
    let liquidity_provider_address =
        ComponentAddress::try_from(liquidity_provider_account.address).unwrap();
    let transaction_id = wallet_daemon::transaction_call(
        liquidity_provider_account.key_index,
        Transaction::builder()
            .fee_transaction_pay_from_component(liquidity_provider_address, Amount(1000))
            .call_method(
                liquidity_provider_address,
                "withdraw",
                args![lp_resource_address, lp_amount.round() as i64],
            )
            .put_last_instruction_output_on_workspace("lp")
            .call_method(
                tex_component_instance_address,
                "remove_liquidity",
                args![Workspace("lp")],
            )
            .put_last_instruction_output_on_workspace("a_and_b")
            .call_method(
                liquidity_provider_address,
                "deposit_all",
                args![Workspace("a_and_b")],
            )
            .build_unsigned_transaction(),
        &mut wallet_daemon_client,
    )
    .await
    .unwrap();
    println!("{}", transaction_id);
    Ok(transaction_id.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn swap(
    a_resource_address_str: String,
    a_amount: f64,
    b_resource_address_str: String,
) -> Result<String, ()> {
    let a_resource_address = ResourceAddress::from_str(&a_resource_address_str).unwrap();
    let b_resource_address = ResourceAddress::from_str(&b_resource_address_str).unwrap();
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await.unwrap();
    let tex_component_instance_address =
        ComponentAddress::from_str(TEX_COMPONENT_INSTANCE_ADDRESS_STR).unwrap();
    let liquidity_provider_account_name = LIQUIDITY_PROVIDER.to_string();
    //TODO: would be nice to store in local storage or somewhere in backend?
    let liquidity_provider_account = wallet_daemon_client
        .accounts_get(ComponentAddressOrName::Name(
            liquidity_provider_account_name.clone(),
        ))
        .await
        .unwrap()
        .account;
    let liquidity_provider_address =
        ComponentAddress::try_from(liquidity_provider_account.address).unwrap();
    let transaction_id = wallet_daemon::transaction_call(
        liquidity_provider_account.key_index,
        Transaction::builder()
            .fee_transaction_pay_from_component(liquidity_provider_address, Amount(1000))
            .call_method(
                liquidity_provider_address,
                "withdraw",
                args![a_resource_address, a_amount.round() as i64],
            )
            .put_last_instruction_output_on_workspace("a")
            .call_method(
                tex_component_instance_address,
                "swap",
                args![Workspace("a"), b_resource_address],
            )
            .put_last_instruction_output_on_workspace("b")
            .call_method(liquidity_provider_address, "deposit", args![Workspace("b")])
            .build_unsigned_transaction(),
        &mut wallet_daemon_client,
    )
    .await
    .unwrap();
    println!("{}", transaction_id);
    Ok(transaction_id.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn exchange_state(account_name: String) -> Exchange {
    println!("{account_name} loads exchange");
    let mut indexer_client = indexer::client_connect(None).await.unwrap();
    let substates = indexer::substates(
        TemplateAddress::from_hex(TEX_TEMPLATE_HEX).unwrap(),
        &mut indexer_client,
    )
    .await
    .unwrap();
    println!("exchange substates: {substates:?}");
    let deserialized = substates
        .first()
        .unwrap()
        .substate_value()
        .as_component()
        .unwrap()
        .body
        .state
        .deserialized::<low_level::Exchange>()
        .unwrap();
    println!("exchange deserialized: {deserialized:?}");
    let result = Exchange::from(deserialized);
    println!("exchange converted: {result:?}");
    result
}

mod low_level {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use tari_all_in_one_rust_sdk::indexer;
    use tari_template_lib::prelude::{ResourceAddress, Vault};
    use tokio::{runtime::Handle, task};

    pub type Pair = (ResourceAddress, ResourceAddress);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct LiquidityPool {
        a: Vault,
        b: Vault,
        lp_resource: ResourceAddress,
        fees_collected: HashMap<String, f64>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct LiquidityPools {
        inner: HashMap<Pair, LiquidityPool>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Exchange {
        liquidity_pools: LiquidityPools,
        fee: i64,
    }

    impl From<Exchange> for tari_exchange::Exchange {
        fn from(value: Exchange) -> Self {
            Self {
                liquidity_pools: value.liquidity_pools.into(),
                fee: value.fee,
            }
        }
    }
    impl From<LiquidityPools> for tari_exchange::LiquidityPools {
        fn from(value: LiquidityPools) -> Self {
            Self {
                inner: value
                    .inner
                    .into_iter()
                    .map(|(pair, pool)| (format!("{}:{}", pair.0, pair.1), pool.into()))
                    .collect(),
            }
        }
    }

    impl From<LiquidityPool> for tari_exchange::LiquidityPool {
        fn from(value: LiquidityPool) -> Self {
            task::block_in_place(move || {
                Handle::current().block_on(async {
                    let mut indexer_client = indexer::client_connect(None).await.unwrap();
                    Self {
                        a: indexer::get_vault(value.a.vault_id(), &mut indexer_client)
                            .await
                            .balance()
                            .0,
                        b: indexer::get_vault(value.b.vault_id(), &mut indexer_client)
                            .await
                            .balance()
                            .0,
                        lp_resource: value.lp_resource,
                        //TODO:
                        fees_collected: HashMap::new(),
                    }
                })
            })
        }
    }
}
