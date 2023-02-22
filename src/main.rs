#![allow(dead_code, unused_variables, unused)]

use aws_sdk_kinesis::{Client, Error, Region};
use std::{env};
use std::str;
use std::sync::Arc;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::sts::AssumeRoleProvider;
use aws_sdk_kinesis::model::{Shard, ShardIteratorType};
use std::io::prelude::*;
use tokio::sync::mpsc;
use tokio::task;
use flate2::read::{ZlibDecoder};
use clap::{arg, Parser};

#[derive(Parser, Clone)]
struct Args {
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
    let client = get_client(args.clone()).await;
    read_stream(&client, args.clone()).await;
    Ok(())
}

async fn show_streams(client: &Client) -> Result<(), Error> {
    let resp = client.list_streams().send().await?;

    println!("Stream names:");

    let streams = resp.stream_names.unwrap_or_default();
    for stream in &streams {
        println!("  {}", stream);
    }

    println!("Found {} stream(s)", streams.len());

    Ok(())
}

async fn show_stream(client: &Client, stream: &String) -> Result<(), Error> {
    let resp = client.describe_stream().stream_name(stream).send().await?;
    let desc = resp.stream_description.unwrap();
    let shards = desc.shards.unwrap();

    println!("Stream description:");
    println!("  Name:              {}:", desc.stream_name.unwrap());
    println!("  Status:            {:?}", desc.stream_status.unwrap());
    println!("  Open shards:       {:?}", shards.len());
    shards.iter()
        .map(|shard| println!("    {}", shard.shard_id().unwrap()))
        .collect::<Vec<_>>();
    println!("  Retention (hours): {}", desc.retention_period_hours.unwrap());
    println!("  Encryption:        {:?}", desc.encryption_type.unwrap());
    println!("  ARN:               {:?}", desc.stream_arn.unwrap());
    Ok(())
}

async fn read_stream(client: &Client, args: Args) -> Result<(), Error> {
    let stream = &args.stream;
    show_stream(client, stream).await;
    let resp = client.describe_stream().stream_name(stream).send().await?;
    let desc = resp.stream_description.unwrap();
    let shards = desc.shards.unwrap();
    let shard_count = shards.len();
    let (tx, mut rx) = mpsc::channel(shards.len());

    println!("Listening kinesis events from {} shards", shard_count);
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
            println!("{}", result);
        }
    }
}

async fn get_client(args: Args) -> Client {
    dotenv::dotenv().is_ok();
    let region = args.aws_region.unwrap_or(env::var("AWS_REGION").unwrap_or(String::from("eu-west-3")));
    let profile = args.aws_profile.unwrap_or(env::var("AWS_PROFILE_NAME").unwrap());
    let role_arn = args.aws_role_arn.unwrap_or(env::var("AWS_ROLE_ARN").unwrap());
    let session_name = args.aws_session_name.unwrap_or(env::var("AWS_SESSION_NAME").unwrap());

    let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile)
        .build();
    let provider = AssumeRoleProvider::builder(role_arn)
        .region(Region::new(region))
        .session_name(session_name)
        .build(Arc::new(credentials_provider) as Arc<_>);
    let shared_config = aws_config::from_env()
        .credentials_provider(provider)
        .load()
        .await;
    Client::new(&shared_config)
}