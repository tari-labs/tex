use std::{error::Error, str::FromStr};

use tari_template_lib::prelude::*;
use tari_transaction::Transaction;
use tari_wallet_daemon_client::ComponentAddressOrName;
use tari_all_in_one_rust_sdk::wallet_daemon;

async fn prepare() {
    // use tari_engine_types::{TemplateAddress, substate::SubstateId};
    // let nft_template_address = TemplateAddress::from_hex(
    //     "5db0a04c7655697e637bea39de4c3707e757e63c0196f0a33b13121d7802788e",
    // )?;
    // let mkt_template_address = TemplateAddress::from_hex(
    //     "f30e59dfc26199ac3f421d630574ad903a41835adb5295a16aaa91492ca59d60",
    // )?;
    // let (nft_component_instance_address, _component_instance_version) =
    //     wallet_daemon::transaction_call_and_wait(
    //         seller_account.key_index,
    //         Transaction::builder()
    //             .fee_transaction_pay_from_component(
    //                 seller_account.address.as_component_address().unwrap(),
    //                 Amount(2000),
    //             )
    //             .call_function(nft_template_address, String::from("new"), vec![])
    //             .build_unsigned_transaction(),
    //         &mut wallet_daemon_client,
    //     )
    //     .await?
    //     .result
    //     .expect("failed to get new nft component result")
    //     .up_iter()
    //     .find_map(|(addr, data)| {
    //         if let SubstateId::Component(_) = addr {
    //             let component_substate_id = ComponentAddress::try_from(addr.clone()).unwrap();
    //             let component_version = data.version();
    //             Some((component_substate_id, component_version))
    //         } else {
    //             None
    //         }
    //     })
    //     .unwrap();
    // let (mkt_component_instance_address, _component_instance_version) =
    //     wallet_daemon::transaction_call_and_wait(
    //         seller_account.key_index,
    //         Transaction::builder()
    //             .fee_transaction_pay_from_component(
    //                 seller_account.address.as_component_address().unwrap(),
    //                 Amount(2000),
    //             )
    //             .call_function(mkt_template_address, String::from("new"), args![0, 0])
    //             .build_unsigned_transaction(),
    //         &mut wallet_daemon_client,
    //     )
    //     .await?
    //     .result
    //     .expect("failed to get new nft component result")
    //     .up_iter()
    //     .find_map(|(addr, data)| {
    //         if let SubstateId::Component(_) = addr {
    //             let component_substate_id = ComponentAddress::try_from(addr.clone()).unwrap();
    //             let component_version = data.version();
    //             Some((component_substate_id, component_version))
    //         } else {
    //             None
    //         }
    //     })
    //     .unwrap();
}

#[tokio::test]
async fn test_mkt() -> Result<(), Box<dyn Error>> {
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await?;
    let seller_account_name = String::from("SdkSeller");
    let seller_account = wallet_daemon_client
        .accounts_get(ComponentAddressOrName::Name(seller_account_name.clone()))
        .await?
        .account;
    let buyer_account_name = String::from("SdkBuyer");
    let buyer_account = wallet_daemon_client
        .accounts_get(ComponentAddressOrName::Name(buyer_account_name.clone()))
        .await?
        .account;
    let nft_component_instance_address = ComponentAddress::from_str(
        "component_58f6b90a5b2f3eba839fb2effc7ebae0d028e64712f6d1a200ee8d0260e42616",
    )?;
    let mkt_component_instance_address = ComponentAddress::from_str(
        "component_6d0fbda9c082f6587a7b317a01eebb52c13ad8dc7c656aa750c73ca42b39bd12",
    )?;
    let seller_address = seller_account.address.as_component_address().unwrap();
    let token_address = ResourceAddress::from_str(
        "resource_58f6b90a5b2f3eba839fb2effc7ebae0d028e6474b005bdad1c47b0aad723b66",
    )?;
    // let result = wallet_daemon::transaction_call_and_wait(
    //     seller_account.key_index,
    //     Transaction::builder()
    //         .fee_transaction_pay_from_component(seller_address, Amount(1000))
    //         .call_method(nft_component_instance_address, "mint", vec![])
    //         .put_last_instruction_output_on_workspace("minted_nft")
    //         .call_method(seller_address, "deposit", args![Workspace("minted_nft")])
    //         .build_unsigned_transaction(),
    //     &mut wallet_daemon_client,
    // )
    // .await?;

    let result = wallet_daemon::transaction_call_and_wait(
        seller_account.key_index,
        Transaction::builder()
            .fee_transaction_pay_from_component(seller_address, Amount(1000))
            // .call_method(nft_component_instance_address, "mint", vec![])
            // .put_last_instruction_output_on_workspace("nft_to_sell")
            .call_method(seller_address, "withdraw", args![token_address, 1])
            .put_last_instruction_output_on_workspace("nft_to_sell")
            .call_method(mkt_component_instance_address, "token_list", args![
                seller_address,
                Workspace("nft_to_sell"),
                10
            ])
            .build_unsigned_transaction(),
        &mut wallet_daemon_client,
    )
    .await?;
    // let token_address = ResourceAddress::from_str(
    //     result
    //         .events
    //         .iter()
    //         .find(|event| event.topic() == *"token_listed")
    //         .unwrap()
    //         .payload()
    //         .get("token")
    //         .unwrap(),
    // )?;
    // eprintln!("token: {token_address}");
    // let buyer_address = buyer_account.address.as_component_address().unwrap();
    // let result = wallet_daemon::transaction_call_and_wait(
    //     buyer_account.key_index,
    //     Transaction::builder()
    //         .fee_transaction_pay_from_component(buyer_address, Amount(1000))
    //         .call_method(buyer_address, "withdraw", args![XTR, 10])
    //         .put_last_instruction_output_on_workspace("coins")
    //         .call_method(mkt_component_instance_address, "token_buy", args![
    //             Workspace("coins"),
    //             token_address
    //         ])
    //         .put_last_instruction_output_on_workspace("bought_nft")
    //         .call_method(buyer_address, "deposit", args![Workspace("bought_nft")])
    //         .build_unsigned_transaction(),
    //     &mut wallet_daemon_client,
    // )
    // .await?;
    // eprintln!("{:#?}", result);
    Ok(())
}
