use std::io::prelude::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use aws_sdk_kinesis::{Client, Error};
use aws_sdk_kinesis::model::{Shard, ShardIteratorType};
use flate2::read::ZlibDecoder;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task;

use crate::client::{ClientConfig};

pub async fn list_streams(client: Arc<Client>, client_config: &ClientConfig) -> Result<(), Error> {
    let resp = client.list_streams().send().await?;
    let streams = resp.stream_names.unwrap_or_default();

    println!("========================================================================================");
    println!("|                                    Context loaded                                    |");
    println!("========================================================================================");
    println!("AWS_REGION                     | {}", client_config.region);
    println!("AWS_PROFILE                    | {}", client_config.profile);
    println!("AWS_ROLE                       | {}", client_config.role_arn);
    println!("AWS_SESSION_NAME               | {}", client_config.session_name);

    println!("========================================================================================");
    println!("|                                   {} Streams found                                   |", streams.len());
    println!("========================================================================================");

    for stream in &streams {
        println!("  {}", stream);
    }

    Ok(())
}
