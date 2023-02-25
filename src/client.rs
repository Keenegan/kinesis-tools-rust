use std::env;
use std::sync::Arc;

use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::sts::AssumeRoleProvider;
use aws_sdk_kinesis::{Client, Region};

use crate::Args;

pub struct ClientConfig {
    pub region: String,
    pub profile: String,
    pub role_arn: String,
    pub session_name: String,
}

pub async fn get_client(args: Args) -> (Client, ClientConfig) {
    let _ = dotenv::dotenv().is_ok();
    let client_config = ClientConfig {
        region: args.aws_region.unwrap_or(env::var("AWS_REGION").unwrap_or(String::from("eu-west-3"))),
        profile: args.aws_profile.unwrap_or(env::var("AWS_PROFILE_NAME").unwrap()),
        role_arn: args.aws_role_arn.unwrap_or(env::var("AWS_ROLE_ARN").unwrap()),
        session_name: args.aws_session_name.unwrap_or(env::var("AWS_SESSION_NAME").unwrap()),
    };

    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(client_config.profile.clone())
        .build();
    let provider = AssumeRoleProvider::builder(client_config.role_arn.clone())
        .region(Region::new(client_config.region.clone()))
        .session_name(client_config.session_name.clone())
        .build(Arc::new(credentials_provider) as Arc<_>);
    let shared_config = aws_config::from_env()
        .credentials_provider(provider)
        .load()
        .await;
    (Client::new(&shared_config), client_config)
}