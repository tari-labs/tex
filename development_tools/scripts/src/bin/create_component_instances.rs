use std::error::Error;

use scripts::templates;
use tari_all_in_one_rust_sdk::wallet_daemon;
use tari_engine_types::TemplateAddress;
use tari_exchange::{ADMIN, COIN_TEMPLATE_HEX, TEX_TEMPLATE_HEX};
use tari_template_lib::args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut wallet_daemon_client = wallet_daemon::client_connect_and_login(None).await?;
    let account_name = ADMIN.to_string();
    //TODO: add as env variable?
    let tex_template_address = TemplateAddress::from_hex(TEX_TEMPLATE_HEX).unwrap();
    let coin_template_address = TemplateAddress::from_hex(COIN_TEMPLATE_HEX).unwrap();
    {
        let (component_instance_address, _component_instance_version) = templates::call_new(
            account_name.clone(),
            tex_template_address,
            args![10],
            &mut wallet_daemon_client,
        )
        .await;
        println!("TEX: {component_instance_address}");
    }
    {
        let (component_instance_address, _component_instance_version) = templates::call_new(
            account_name.clone(),
            coin_template_address,
            args![10000, "BTR"],
            &mut wallet_daemon_client,
        )
        .await;
        println!("COIN: {component_instance_address}");
    }
    Ok(())
}
