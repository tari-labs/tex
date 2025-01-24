use tari_all_in_one_rust_sdk::wallet_daemon;
use tari_exchange::{TariCoin, TariTransaction};

pub mod exchange;

#[tauri::command(rename_all = "snake_case")]
pub async fn coins_load(account_name: &str) -> Result<Vec<TariCoin>, ()> {
    println!("{account_name} loads coins");
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await.unwrap();
    let tokens = wallet_daemon::accounts_tokens(account_name, &mut wallet_daemon_client)
        .await
        .unwrap()
        .0;
    Ok(tokens
        .into_iter()
        .map(|token| TariCoin {
            id: token.resource_address,
            name: token.token_symbol.unwrap_or_default(),
            balance: token.balance.to_string(),
        })
        .collect())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn transactions_load(account_name: &str) -> Result<Vec<TariTransaction>, ()> {
    println!("{account_name} loads transactions");
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await.unwrap();
    let transactions =
        wallet_daemon::accounts_transactions(account_name, &mut wallet_daemon_client)
            .await
            .unwrap()
            .0;
    Ok(transactions
        .into_iter()
        .map(|transaction| TariTransaction {
            id: transaction.transaction.id().to_string(),
            json: transaction.transaction.to_string(),
            result: transaction
                .result
                .as_ref()
                .and_then(|result| serde_json::to_string_pretty(&result).ok()),
            status: transaction.status.to_string(),
            date_time: transaction.date_time,
        })
        .collect())
}
