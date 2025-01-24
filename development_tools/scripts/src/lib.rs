use tari_template_lib::prelude::Amount;
use tari_wallet_daemon_client::{
    types::{AccountsCreateFreeTestCoinsRequest, KeyBranch},
    ComponentAddressOrName, WalletDaemonClient,
};

//TODO: migrate everything to SDK

pub async fn create_new_account(account_name: &str, client: &mut WalletDaemonClient) {
    let key = client.create_key(KeyBranch::Transaction).await.unwrap();
    let resp = client
        .create_free_test_coins(AccountsCreateFreeTestCoinsRequest {
            account: Some(ComponentAddressOrName::Name(account_name.to_string())),
            amount: Amount::new(10_000),
            max_fee: None,
            key_id: Some(key.id),
        })
        .await
        .unwrap();
    println!(
        "{account_name} creation accepted: {}",
        resp.result.result.is_accept()
    );
}

pub mod templates {

    use tari_engine_types::{substate::SubstateId, TemplateAddress};
    use tari_template_lib::{
        args::Arg,
        prelude::{Amount, ComponentAddress},
    };
    use tari_wallet_daemon_client::{
        types::{AccountGetResponse, TransactionSubmitRequest, TransactionWaitResultRequest},
        ComponentAddressOrName, WalletDaemonClient,
    };

    pub async fn call_new(
        account_name: String,
        template_address: TemplateAddress,
        args: Vec<Arg>,
        client: &mut WalletDaemonClient,
    ) -> (ComponentAddress, u32) {
        let AccountGetResponse { account, .. } = client
            .accounts_get(ComponentAddressOrName::Name(account_name.clone()))
            .await
            .unwrap();
        let transaction = tari_transaction::Transaction::builder()
            .fee_transaction_pay_from_component(
                account.address.as_component_address().unwrap(),
                Amount(2000),
            )
            .call_function(template_address, String::from("new"), args)
            .build_unsigned_transaction();
        let transaction_submit_req = TransactionSubmitRequest {
            transaction,
            signing_key_index: Some(account.key_index),
            detect_inputs: true,
            detect_inputs_use_unversioned: false,
            proof_ids: vec![],
            autofill_inputs: vec![],
        };
        let resp = client
            .submit_transaction(transaction_submit_req)
            .await
            .unwrap();
        let wait_req = TransactionWaitResultRequest {
            transaction_id: resp.transaction_id,
            timeout_secs: Some(120),
        };
        let wait_resp = client.wait_transaction_result(wait_req).await.unwrap();
        let component_substate_diff = wait_resp
            .result
            .expect("No result")
            .result
            .expect("Failed to obtain substate diffs");
        let mut component_substate_id: Option<ComponentAddress> = None;
        let mut component_version: Option<u32> = None;
        for (addr, data) in component_substate_diff.up_iter() {
            if let SubstateId::Component(_) = addr {
                component_substate_id = Some(ComponentAddress::try_from(addr.clone()).unwrap());
                component_version = Some(data.version());
                break;
            }
        }
        (component_substate_id.unwrap(), component_version.unwrap())
    }
}
