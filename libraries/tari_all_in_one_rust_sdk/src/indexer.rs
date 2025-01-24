use std::result::Result as StdResult;

use derive_more::derive::{Display, Error as DeriveError, From};
use tari_engine_types::{TemplateAddress, substate::Substate, vault::Vault};
use tari_indexer_client::{
    error::IndexerClientError,
    json_rpc_client::IndexerJsonRpcClient,
    types::{GetSubstateRequest, ListSubstatesRequest},
};
use tokio::task::JoinError;

pub type Result<T> = StdResult<T, Error>;

const DEFAULT_INDEXER_ENDPOINT: &str = "http://127.0.0.1:12008";

pub async fn client_connect(indexer_endpoint: Option<&str>) -> Result<IndexerJsonRpcClient> {
    let indexer_client =
        IndexerJsonRpcClient::connect(indexer_endpoint.unwrap_or(DEFAULT_INDEXER_ENDPOINT))
            .unwrap();
    Ok(indexer_client)
}
pub async fn substates(
    template_address: TemplateAddress,
    client: &mut IndexerJsonRpcClient,
) -> Result<Vec<Substate>> {
    let substate_ids = client
        .list_substates(ListSubstatesRequest {
            filter_by_template: Some(template_address),
            filter_by_type: None,
            limit: None,
            offset: None,
        })
        .await?
        .substates
        .into_iter()
        .map(|list_item| (list_item.substate_id, list_item.version));
    let mut result = Vec::with_capacity(substate_ids.len());
    let mut tasks = Vec::with_capacity(substate_ids.len());
    for (substate_id, version) in substate_ids {
        let mut client_clone = client.clone();
        tasks.push(tokio::spawn(async move {
            client_clone
                .get_substate(GetSubstateRequest {
                    address: substate_id,
                    version: Some(1),
                    local_search_only: false,
                })
                .await
                .map(|response| response.substate)
        }));
    }
    for task in tasks {
        result.push(task.await??);
    }
    Ok(result)
}

#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    TransactionNotFinilized,
    ClientFailed(IndexerClientError),
    TaskPanicked(JoinError),
}

pub async fn get_vault(
    vault_id: tari_template_lib::prelude::VaultId,
    indexer_client: &mut IndexerJsonRpcClient,
) -> Vault {
    indexer_client
        .get_substate(GetSubstateRequest {
            address: tari_engine_types::substate::SubstateId::Vault(vault_id),
            version: None,
            local_search_only: false,
        })
        .await
        .unwrap()
        .substate
        .into_substate_value()
        .into_vault()
        .unwrap()
}
