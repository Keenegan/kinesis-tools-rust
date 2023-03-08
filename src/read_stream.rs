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

pub async fn read_stream(client: Arc<Client>, stream: &String) -> Result<(), Error> {
    let resp = client.describe_stream().stream_name(stream).send().await.expect("No stream found.");
    let desc = resp.stream_description.unwrap();
    let shards = desc.shards.unwrap();
    let shard_count = shards.len();

    println!("========================================================================================");
    println!("|                                    Stream description                                |");
    println!("========================================================================================");
    println!("Name:              {}:", desc.stream_name.unwrap());
    println!("ARN:               {}:", desc.stream_arn.unwrap());
    println!("Status:            {:?}", desc.stream_status.unwrap());
    println!("Open shards:       {:?}", shards.len());
    shards.iter().for_each(|shard| println!("    {}", shard.shard_id().unwrap()));
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

async fn listen_to_shard(shard: Shard, client: Arc<Client>, stream: String) {
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

            let value: Value = serde_json::from_str(&result).unwrap();
            let pretty = serde_json::to_string_pretty(&value);
            println!("{}", pretty.unwrap());
        }
        sleep(Duration::from_secs(1));
    }
}