use std::{result::Result as StdResult, str::FromStr};

use derive_more::derive::{Display, Error as DeriveError, From, FromStr, Into};
use serde::{Deserialize, Serialize};
use tari_engine_types::{
    commit_result::FinalizeResult,
    substate::{InvalidSubstateIdVariant, SubstateId},
};
use tari_template_lib::prelude::{
    Amount, ComponentAddress, NonFungibleId, ResourceAddress, ResourceType, VaultId,
};
// use tari_template_lib::prelude::{NonFungibleId, VaultId};
use tari_transaction::{Transaction, TransactionId, UnsignedTransaction};
use tari_wallet_daemon_client::{
    ComponentAddressOrName, WalletDaemonClient,
    error::WalletDaemonClientError,
    types::{
        AccountsGetBalancesRequest, AuthLoginAcceptRequest, AuthLoginRequest, AuthLoginResponse,
        ListAccountNftRequest, TransactionGetAllRequest, TransactionSubmitRequest,
        TransactionWaitResultRequest,
    },
};

const DEFAULT_WALLET_DEAMON_ENDPOINT: &str = "http://127.0.0.1:12011";
const DEFAULT_TRANSACTION_TIMEOUT_SECONDS: u64 = 10;
const ADMIN_PERMISSION: &str = "Admin";

pub type Result<T> = StdResult<T, Error>;

pub async fn client_connect_and_login(
    wallet_daemon_endpoint: Option<&str>,
) -> Result<WalletDaemonClient> {
    let mut wallet_daemon_client = WalletDaemonClient::connect(
        wallet_daemon_endpoint.unwrap_or(DEFAULT_WALLET_DEAMON_ENDPOINT),
        None,
    )
    .unwrap();
    let AuthLoginResponse { auth_token, .. } = wallet_daemon_client
        .auth_request(AuthLoginRequest {
            permissions: vec![ADMIN_PERMISSION.to_string()],
            duration: None,
        })
        .await?;
    let auth_response = wallet_daemon_client
        .auth_accept(AuthLoginAcceptRequest {
            auth_token,
            name: "Testing Token".to_string(),
        })
        .await?;
    wallet_daemon_client.set_auth_token(auth_response.permissions_token);
    Ok(wallet_daemon_client)
}

pub async fn transaction_call(
    key_index: u64,
    transaction: UnsignedTransaction,
    client: &mut WalletDaemonClient,
) -> Result<TransactionId> {
    Ok(client
        .submit_transaction(TransactionSubmitRequest {
            transaction,
            signing_key_index: Some(key_index),
            autofill_inputs: vec![],
            detect_inputs: true,
            detect_inputs_use_unversioned: true,
            proof_ids: vec![],
        })
        .await?
        .transaction_id)
}

pub async fn transaction_call_and_wait(
    key_index: u64,
    transaction: UnsignedTransaction,
    client: &mut WalletDaemonClient,
) -> Result<FinalizeResult> {
    let transaction_id = client
        .submit_transaction(TransactionSubmitRequest {
            transaction,
            signing_key_index: Some(key_index),
            autofill_inputs: vec![],
            detect_inputs: true,
            detect_inputs_use_unversioned: false,
            proof_ids: vec![],
        })
        .await?
        .transaction_id;
    client
        .wait_transaction_result(TransactionWaitResultRequest {
            transaction_id,
            timeout_secs: Some(DEFAULT_TRANSACTION_TIMEOUT_SECONDS),
        })
        .await?
        .result
        .ok_or_else(|| Error::TransactionNotFinilized)
}

pub async fn accounts_nfts(
    account_name: &str,
    limit: u64,
    offset: u64,
    client: &mut WalletDaemonClient,
) -> Result<Nfts> {
    let component_address_or_name = ComponentAddressOrName::Name(account_name.to_string());
    Ok(Nfts(
        client
            .list_account_nfts(ListAccountNftRequest {
                account: Some(component_address_or_name.clone()),
                limit,
                offset,
            })
            .await?
            .nfts
            .into_iter()
            .map(|inner_nft| NonFungibleToken {
                vault_id: inner_nft.vault_id,
                nft_id: inner_nft.nft_id,
                data: inner_nft.data,
                mutable_data: inner_nft.mutable_data,
                is_burned: inner_nft.is_burned,
            })
            .collect(),
    ))
}

pub async fn accounts_tokens(
    account_name: &str,
    client: &mut WalletDaemonClient,
) -> Result<Tokens> {
    let component_address_or_name = ComponentAddressOrName::Name(account_name.to_string());
    Ok(Tokens(
        client
            .get_account_balances(AccountsGetBalancesRequest {
                account: Some(component_address_or_name.clone()),
                refresh: false,
            })
            .await?
            .balances
            .into_iter()
            .map(|balance| FungibleToken {
                vault_address: balance.vault_address,
                resource_address: balance.resource_address,
                balance: balance.balance,
                resource_type: balance.resource_type,
                confidential_balance: balance.confidential_balance,
                token_symbol: balance.token_symbol,
            })
            .collect(),
    ))
}

pub async fn accounts_transactions(
    account_name: &str,
    client: &mut WalletDaemonClient,
) -> Result<Transactions> {
    let component_address_or_name = ComponentAddressOrName::Name(account_name.to_string());
    let component_address: ComponentAddress = client
        .accounts_get(component_address_or_name)
        .await?
        .account
        .address
        .try_into()?;
    Ok(Transactions(
        client
            .get_transactions_all(TransactionGetAllRequest {
                component: Some(component_address),
                status: None,
            })
            .await?
            .transactions
            .into_iter()
            .map(
                |(transaction, result, status, date_time)| TransactionWithData {
                    transaction,
                    result,
                    status: TransactionStatus::from_str(status.as_key_str()).unwrap_or_default(),
                    date_time: date_time.to_string(),
                },
            )
            .collect(),
    ))
}

#[derive(Debug, Clone, From, Into)]
pub struct Nfts(pub Vec<NonFungibleToken>);

#[derive(Debug, Clone, From, Into)]
pub struct Tokens(pub Vec<FungibleToken>);

#[derive(Debug, Clone, From, Into)]
pub struct Transactions(pub Vec<TransactionWithData>);

#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    TransactionNotFinilized,
    ClientFailed(WalletDaemonClientError),
    AddressIncorrect(InvalidSubstateIdVariant),
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Default,
    FromStr,
    Display,
)]
pub enum TransactionStatus {
    #[default]
    New,
    DryRun,
    Pending,
    Accepted,
    Rejected,
    InvalidTransaction,
    OnlyFeeAccepted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionWithData {
    pub transaction: Transaction,
    pub result: Option<FinalizeResult>,
    pub status: TransactionStatus,
    pub date_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonFungibleToken {
    pub vault_id: VaultId,
    pub nft_id: NonFungibleId,
    #[serde(with = "cbor_value")]
    pub data: tari_bor::Value,
    #[serde(with = "cbor_value")]
    pub mutable_data: tari_bor::Value,
    pub is_burned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FungibleToken {
    pub vault_address: SubstateId,
    #[serde(with = "string")]
    pub resource_address: ResourceAddress,
    pub balance: Amount,
    pub resource_type: ResourceType,
    pub confidential_balance: Amount,
    pub token_symbol: Option<String>,
}

mod cbor_value {

    //   Copyright 2023 The Tari Project
    //   SPDX-License-Identifier: BSD-3-Clause

    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use tari_bor::json_encoding::{CborValueJsonDeserializeWrapper, CborValueJsonSerializeWrapper};

    pub fn serialize<S: Serializer>(v: &tari_bor::Value, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            CborValueJsonSerializeWrapper(v).serialize(s)
        } else {
            v.serialize(s)
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<tari_bor::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        if d.is_human_readable() {
            let wrapper = CborValueJsonDeserializeWrapper::deserialize(d)?;
            Ok(wrapper.0)
        } else {
            tari_bor::Value::deserialize(d)
        }
    }
}

mod string {

    //   Copyright 2022 The Tari Project
    //   SPDX-License-Identifier: BSD-3-Clause

    use std::str::FromStr;

    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};

    pub fn serialize<S: Serializer, T: ToString + Serialize>(
        v: &T,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            s.serialize_str(&v.to_string())
        } else {
            v.serialize(s)
        }
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr + DeserializeOwned,
        T::Err: std::fmt::Display,
    {
        if d.is_human_readable() {
            let s = <String as Deserialize>::deserialize(d)?;
            s.parse().map_err(serde::de::Error::custom)
        } else {
            T::deserialize(d)
        }
    }

    pub mod option {
        use super::*;

        pub fn serialize<S: Serializer, T: ToString + Serialize>(
            v: &Option<T>,
            s: S,
        ) -> Result<S::Ok, S::Error> {
            if s.is_human_readable() {
                match v {
                    Some(v) => s.serialize_some(&v.to_string()),
                    None => s.serialize_none(),
                }
            } else {
                v.serialize(s)
            }
        }

        pub fn deserialize<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
        where
            D: Deserializer<'de>,
            T: FromStr + DeserializeOwned,
            T::Err: std::fmt::Display,
        {
            if d.is_human_readable() {
                let s = <Option<String> as Deserialize>::deserialize(d)?;
                s.map(|s| s.parse())
                    .transpose()
                    .map_err(serde::de::Error::custom)
            } else {
                Option::<T>::deserialize(d)
            }
        }
    }
}
