use aws_config::meta::region::RegionProviderChain;
use std::env;
use std::error::Error;
use std::sync::Arc;

use aws_sdk_kinesis::types::SdkError;
use aws_sdk_kinesis::Client;

use crate::Args;

pub async fn get_client(_args: Args) -> Arc<Client> {
    let _ = dotenv::dotenv().is_ok();
    env::var("AWS_PROFILE").expect("Missing AWS_PROFILE environment variable");
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-3");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    match client.list_streams().limit(1).send().await {
        Ok(_) => Arc::new(Client::new(&config)),
        Err(sdk_error) => match sdk_error {
            SdkError::ConstructionFailure(_) => {
                panic!("Could not construct AWS client. Do you have correct credentials ? Error message : {}", &sdk_error.source().unwrap().to_string());
            }
            _ => panic!(
                "Failed to connect to AWS client ? Error message : {}",
                &sdk_error.source().unwrap().to_string()
            ),
        },
    }
}
