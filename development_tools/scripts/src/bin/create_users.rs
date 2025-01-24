use std::error::Error;

use tari_all_in_one_rust_sdk::wallet_daemon;
use tari_exchange::{ADMIN, LIQUIDITY_PROVIDER, TRADER};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await?;
    scripts::create_new_account(ADMIN, &mut wallet_daemon_client).await;
    scripts::create_new_account(LIQUIDITY_PROVIDER, &mut wallet_daemon_client).await;
    scripts::create_new_account(TRADER, &mut wallet_daemon_client).await;
    Ok(())
}
