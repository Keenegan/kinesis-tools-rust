use std::io::prelude::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use aws_sdk_kinesis::{Client, Error};
use aws_sdk_kinesis::types::{Shard, ShardIteratorType};
use base64::{engine::general_purpose, Engine as _};
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
            println!("{}", json_format(result));
        }
        sleep(Duration::from_secs(1));
    }
}

fn unzip_input(input: &[u8]) -> Option<String> {
    let mut buffer = String::new();
    match std::io::Cursor::new(input).read_to_string(&mut buffer) {
        Ok(_) => {
            if buffer.ends_with('=') {
                return match general_purpose::STANDARD.decode(&buffer) {
                    Ok(bytes) => {
                        let result = std::str::from_utf8(bytes.as_slice()).unwrap().to_string();
                        Some(result)
                    }
                    Err(_) => Some(buffer),
                };
            }
            return Some(buffer);
        }
        Err(_e) => {
            // TODO add logging
        }
    }

    buffer.clear();
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
    None
}

fn json_format(result: String) -> String {
    match serde_json::from_str::<Value>(&result) {
        Ok(mut value) => {
            // TODO check if there is no cleaner way to do this
            if !value.is_object() {
                value = serde_json::from_str(value.as_str().unwrap()).unwrap();
            }
            serde_json::to_string_pretty(&value).unwrap()
        }
        Err(_) => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unzip_input_with_uncompressed_data() {
        let input = "This is an uncompressed string";
        let input_bytes = input.as_bytes();
        let output = json_format(unzip_input(input_bytes).unwrap());

        assert_eq!(output, input);
    }

    #[test]
    fn test_unzip_and_format_gzipped_text() {
        // TODO don't use files for tests
        let input = std::fs::read("tests/gzipped-text.json").unwrap();
        let expect = std::fs::read_to_string("tests/text.json").unwrap();
        let output = json_format(unzip_input(input.as_slice()).unwrap());

        assert_eq!(output, expect);
    }

    #[test]
    fn test_unzip_and_format_base64_encoded_text() {
        // TODO ici
        let input = std::fs::read("tests/base64-encoded.json").unwrap();
        let expect = std::fs::read_to_string("tests/text.json").unwrap();
        let output = json_format(unzip_input(input.as_slice()).unwrap());

        assert_eq!(output, expect);
    }

    #[test]
    fn test_unzip_and_format_text_that_look_base64_encoded() {
        let input = "This is an uncompressed string that happen to end with an =";
        let input_bytes = input.as_bytes();
        let output = json_format(unzip_input(input_bytes).unwrap());

        assert_eq!(output, input);
    }
}
