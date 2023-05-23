use aws_config::meta::region::RegionProviderChain;
use std::env;
use std::error::Error;
use std::sync::Arc;

use aws_sdk_kinesis::error::SdkError;
use aws_sdk_kinesis::Client;

use crate::Args;

pub async fn get_client(args: Args) -> Arc<Client> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-3");
    let config = aws_config::from_env()
        .profile_name(get_aws_profile(args))
        .region(region_provider)
        .load()
        .await;
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

fn get_aws_profile(args: Args) -> String {
    if args.profile.is_some() {
        return args.profile.unwrap();
    }
    if env::var("AWS_PROFILE").is_ok() {
        return env::var("AWS_PROFILE").unwrap();
    }
    panic!("Profile not found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Commands;

    #[test]
    fn test_get_aws_profile_from_args_only() {
        let args = Args {
            profile: Some("profile".to_string()),
            command: Commands::List {},
        };
        assert_eq!(get_aws_profile(args), "profile".to_string());
    }

    #[test]
    fn test_get_aws_profile_from_env_only() {
        let args = Args {
            profile: None,
            command: Commands::List {},
        };
        env::set_var("AWS_PROFILE", "profile");
        assert_eq!(get_aws_profile(args), "profile".to_string());
        env::remove_var("AWS_PROFILE");
    }

    #[should_panic]
    #[test]
    fn test_get_aws_profile_with_no_profile() {
        let args = Args {
            profile: None,
            command: Commands::List {},
        };
        env::remove_var("AWS_PROFILE");
        get_aws_profile(args);
    }

    #[test]
    fn test_get_aws_profile_with_args_and_env_variable() {
        let args = Args {
            profile: Some("profile from args".to_string()),
            command: Commands::List {},
        };
        env::set_var("AWS_PROFILE", "profile from env");
        assert_eq!(get_aws_profile(args), "profile from args".to_string());
        env::remove_var("AWS_PROFILE");
    }
}
