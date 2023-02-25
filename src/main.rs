#![allow(dead_code, unused_variables, unused)]

use std::env;
use std::io::prelude::*;
use std::str;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::sts::AssumeRoleProvider;
use aws_sdk_kinesis::{Client, Error, Region};
use aws_sdk_kinesis::model::{Shard, ShardIteratorType};
use clap::{arg, Parser};
use flate2::read::ZlibDecoder;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task;

use crate::client::{ClientConfig, get_client};

#[macro_use]
extern crate serde_json;

mod client;

#[derive(Parser, Clone)]
pub struct Args {
    #[arg(long)]
    stream: String,
    #[arg(long)]
    aws_region: Option<String>,
    #[arg(long)]
    aws_profile: Option<String>,
    #[arg(long)]
    aws_role_arn: Option<String>,
    #[arg(long)]
    aws_session_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let stream = &args.stream;
    let (client, client_config) = get_client(args.clone()).await;
    read_stream(&client, &client_config, stream).await;
    Ok(())
}

async fn read_stream(client: &Client, client_config: &ClientConfig, stream: &String) -> Result<(), Error> {
    let resp = client.describe_stream().stream_name(stream).send().await.expect("No stream found.");
    let desc = resp.stream_description.unwrap();
    let shards = desc.shards.unwrap();
    let shard_count = shards.len();

    println!("========================================================================================");
    println!("|                                    Context loaded                                    |");
    println!("========================================================================================");
    println!("AWS_REGION                     | {}", client_config.region);
    println!("AWS_PROFILE                    | {}", client_config.profile);
    println!("AWS_ROLE                       | {}", client_config.role_arn);
    println!("AWS_SESSION_NAME               | {}", client_config.session_name);

    println!("========================================================================================");
    println!("|                                    Stream description                                |");
    println!("========================================================================================");
    println!("Name:              {}:", desc.stream_name.unwrap());
    println!("ARN:               {}:", desc.stream_arn.unwrap());
    println!("Status:            {:?}", desc.stream_status.unwrap());
    println!("Open shards:       {:?}", shards.len());
    shards.iter()
        .map(|shard| println!("    {}", shard.shard_id().unwrap()))
        .collect::<Vec<_>>();
    println!("Encryption:        {:?}", desc.encryption_type.unwrap());
    let (tx, mut rx) = mpsc::channel(shards.len());

    println!("========================================================================================");
    println!("|                 Listening kinesis events from {shard_count} shards                               |");
    println!("========================================================================================");
    for shard in shards {
        let client_clone = client.clone();
        let tx_clone = tx.clone();
        let stream_clone = stream.clone();
        task::spawn(async move {
            listen_to_shard(shard, client_clone, stream_clone).await;
            tx_clone.send(()).await.unwrap();
        });
    }

    for _ in 0..shard_count {
        rx.recv().await.unwrap();
    }
    Ok(())
}

async fn listen_to_shard(shard: Shard, client: Client, stream: String) {
    let shard_id = shard.shard_id().unwrap();
    let shard_iter_output = client.get_shard_iterator()
        .stream_name(stream)
        .shard_id(shard_id)
        .shard_iterator_type(ShardIteratorType::Latest)
        .send().await.unwrap();
    let mut shard_iter = shard_iter_output.shard_iterator();
    let mut get_records;
    let mut records;

    loop {
        get_records = client.get_records()
            .shard_iterator(shard_iter.unwrap())
            .send()
            .await
            .unwrap();
        shard_iter = get_records.next_shard_iterator();
        records = get_records.records().unwrap();
        if !records.is_empty() {
            let data = records.first().unwrap().data().unwrap().as_ref();
            let mut decoder = ZlibDecoder::new(data);
            let mut result = String::new();
            decoder.read_to_string(&mut result).unwrap();

            let value:Value = serde_json::from_str(&*result).unwrap();
            let pretty = serde_json::to_string_pretty(&value);
            println!("{}", pretty.unwrap());
        }
        sleep(Duration::from_secs(1));
    }
}
