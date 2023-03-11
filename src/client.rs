use aws_config::meta::region::RegionProviderChain;
use std::sync::Arc;

use aws_sdk_kinesis::Client;

use crate::Args;

pub async fn get_client(_args: Args) -> Arc<Client> {
    let _ = dotenv::dotenv().is_ok();
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-3");
    let config = aws_config::from_env().region(region_provider).load().await;
    Arc::new(Client::new(&config))
}
