use std::{error::Error, str::FromStr};

use tari_all_in_one_rust_sdk::{indexer, wallet_daemon};
use tari_exchange::{
    LiquidityPools, COIN_COMPONENT_INSTANCE_ADDRESS_STR, LIQUIDITY_PROVIDER,
    TEX_COMPONENT_INSTANCE_ADDRESS_STR, TEX_TEMPLATE_HEX, TRADER,
};
use tari_indexer_client::types::GetSubstateRequest;
use tari_template_lib::prelude::*;
use tari_transaction::Transaction;
use tari_wallet_daemon_client::ComponentAddressOrName;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //TODO: add as env variable?
    let tex_component_instance_address =
        ComponentAddress::from_str(TEX_COMPONENT_INSTANCE_ADDRESS_STR)?;
    let coin_component_instance_address =
        ComponentAddress::from_str(COIN_COMPONENT_INSTANCE_ADDRESS_STR)?;
    // <-- PREPARATION OF ACCOUNTS INFO --> //
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await?;
    let liquidity_provider_account_name = LIQUIDITY_PROVIDER.to_string();
    let liquidity_provider_account = wallet_daemon_client
        .accounts_get(ComponentAddressOrName::Name(
            liquidity_provider_account_name.clone(),
        ))
        .await?
        .account;
    // let trader_account_name = TRADER.to_string();
    // let trader_account = wallet_daemon_client
    //     .accounts_get(ComponentAddressOrName::Name(trader_account_name.clone()))
    //     .await?
    //     .account;
    let liquidity_provider_address =
    //TODO: change to `?` once https://github.com/tari-project/tari-dan/pull/1243 will be merged
        ComponentAddress::try_from(liquidity_provider_account.address).unwrap();

    // <-- PREPARATION OF COINS --> //
    let _result = wallet_daemon::transaction_call_and_wait(
        liquidity_provider_account.key_index,
        Transaction::builder()
            .fee_transaction_pay_from_component(liquidity_provider_address, Amount(1000))
            .call_method(
                coin_component_instance_address,
                "take_free_coins",
                args![1000],
            )
            .put_last_instruction_output_on_workspace("btr_coins")
            .call_method(
                liquidity_provider_address,
                "deposit",
                args![Workspace("btr_coins")],
            )
            // .add_input(
            //     ResourceAddress::from_str(
            //         "resource_9f6eeb00db28a00a4e499851ea2206c7c518783ca756168ccdca7faa802446a0",
            //     )
            //     .unwrap(),
            // )
            .build_unsigned_transaction(),
        &mut wallet_daemon_client,
    )
    .await;

    // <-- FIRST STEP: ADD LIQUIDITY TO THE POOL --> //
    // let _result = wallet_daemon::transaction_call(
    //     liquidity_provider_account.key_index,
    //     Transaction::builder()
    //         .fee_transaction_pay_from_component(liquidity_provider_address, Amount(1000))
    //         .call_method(liquidity_provider_address, "withdraw", args![XTR, 10])
    //         .put_last_instruction_output_on_workspace("a")
    //         .call_method(coin_component_instance_address, "vault_address", args![])
    //         .put_last_instruction_output_on_workspace("btr_resource_address")
    //         .call_method(
    //             liquidity_provider_address,
    //             "withdraw",
    //             args![Workspace("btr_resource_address"), 10],
    //         )
    //         .put_last_instruction_output_on_workspace("b")
    //         .call_method(
    //             tex_component_instance_address,
    //             "add_liquidity",
    //             args![Workspace("a"), Workspace("b")],
    //         )
    //         .put_last_instruction_output_on_workspace("liquidity_provided_token")
    //         .call_method(
    //             liquidity_provider_address,
    //             "deposit",
    //             args![Workspace("liquidity_provided_token")],
    //         )
    //         // .add_input(
    //         //     VaultId::from_str(
    //         //         "vault_c0346eb3a28f389495c062c234c2c4cd58733fed935b48aed8162d35b8c4bfef",
    //         //     )
    //         //     .unwrap(),
    //         // )
    //         .build_unsigned_transaction(),
    //     &mut wallet_daemon_client,
    // )
    // .await;

    // <-- SECOND STEP: CHECK POOLS --> //
    // let result = wallet_daemon::transaction_call_and_wait(
    //     liquidity_provider_account.key_index,
    //     Transaction::builder()
    //         .fee_transaction_pay_from_component(liquidity_provider_address, Amount(1000))
    //         .call_method(tex_component_instance_address, "pools", args![])
    //         .build_unsigned_transaction(),
    //     &mut wallet_daemon_client,
    // )
    // .await
    // .unwrap();
    // dbg!(&result);
    // let decode = result
    //     .execution_results
    //     .first()
    //     .map(|instruction_result| instruction_result.decode::<LiquidityPools>().unwrap())
    //     .unwrap_or_default();
    // dbg!(&decode);
    let mut indexer_client = indexer::client_connect(None).await?;
    let substates = indexer::substates(
        TemplateAddress::from_hex(TEX_TEMPLATE_HEX).unwrap(),
        &mut indexer_client,
    )
    .await?;
    for substate in substates {
        let state = &substate.substate_value().as_component().unwrap().body.state;
        dbg!(&state);
        let exchange: tari_exchange::Exchange = state.deserialized().unwrap();
        dbg!(exchange);
    }
    Ok(())
}
