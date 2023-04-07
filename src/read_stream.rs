use std::io::prelude::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use aws_sdk_kinesis::model::{Shard, ShardIteratorType};
use aws_sdk_kinesis::{Client, Error};
use flate2::read::{GzDecoder, ZlibDecoder};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task;

pub async fn read_stream(client: Arc<Client>, stream: &String) -> Result<(), Error> {
    let resp = client
        .describe_stream()
        .stream_name(stream)
        .send()
        .await
        .expect("No stream found");
    let desc = resp.stream_description.unwrap();
    let shards = desc.shards.unwrap();
    let shard_count = shards.len();

    println!(
        "========================================================================================"
    );
    println!(
        "|                                    Stream description                                |"
    );
    println!(
        "========================================================================================"
    );
    println!("Name:              {}:", desc.stream_name.unwrap());
    println!("ARN:               {}:", desc.stream_arn.unwrap());
    println!("Status:            {:?}", desc.stream_status.unwrap());
    println!("Open shards:       {:?}", shards.len());
    shards
        .iter()
        .for_each(|shard| println!("    {}", shard.shard_id().unwrap()));
    println!("Encryption:        {:?}", desc.encryption_type.unwrap());
    let (tx, mut rx) = mpsc::channel(shards.len());

    println!(
        "========================================================================================"
    );
    println!("|                 Listening kinesis events from {shard_count} shard(s)                             |");
    println!(
        "========================================================================================"
    );
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
    let shard_iter_output = client
        .get_shard_iterator()
        .stream_name(stream)
        .shard_id(shard_id)
        .shard_iterator_type(ShardIteratorType::Latest)
        .send()
        .await
        .unwrap();
    let mut shard_iter = shard_iter_output.shard_iterator();
    let mut get_records;
    let mut records;

    loop {
        get_records = client
            .get_records()
            .shard_iterator(shard_iter.unwrap())
            .send()
            .await
            .unwrap();
        shard_iter = get_records.next_shard_iterator();
        records = get_records.records().unwrap();
        if !records.is_empty() {
            let data = records
                .first()
                .unwrap()
                .data()
                .expect("Error while reading data")
                .as_ref();
            let result =
                unzip_input(data).expect("Data was received from stream but could not be read.");
            println!("{}", format_result(result));
        }
        sleep(Duration::from_secs(1));
    }
}

fn unzip_input(input: &[u8]) -> Option<String> {
    let mut buffer = String::new();
    match ZlibDecoder::new(input).read_to_string(&mut buffer) {
        Ok(_) => return Some(buffer),
        Err(_e) => {
            // TODO add logging
        }
    }

    buffer.clear();
    match GzDecoder::new(input).read_to_string(&mut buffer) {
        Ok(_) => return Some(buffer),
        Err(_e) => {
            // TODO add logging
        }
    }

    buffer.clear();
    match std::io::Cursor::new(input).read_to_string(&mut buffer) {
        Ok(_) => return Some(buffer),
        Err(_e) => {
            // TODO add logging
        }
    }
    None
}

fn format_result(result: String) -> String {
    let value: Value = serde_json::from_str(&result).unwrap();
    serde_json::to_string_pretty(&value).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unzip_input_with_uncompressed_data() {
        let input = "This is an uncompressed string";
        let input_bytes = input.as_bytes();
        let output = unzip_input(input_bytes).unwrap();

        assert_eq!(output, input.clone());
    }
    // TODO add other tests
}
