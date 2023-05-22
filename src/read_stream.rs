use std::fmt::Display;
use std::io::prelude::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use aws_sdk_kinesis::types::{Shard, ShardIteratorType, StreamDescription};
use aws_sdk_kinesis::{Client, Error};
use base64::{engine::general_purpose, Engine as _};
use flate2::read::{GzDecoder, ZlibDecoder};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task;

pub async fn read_stream(
    client: Arc<Client>,
    disable_unzip: bool,
    stream: &String,
) -> Result<(), Error> {
    let describe = client
        .describe_stream()
        .stream_name(stream)
        .send()
        .await
        .expect("No stream found");
    let stream_description = describe.stream_description.unwrap();
    let shards = stream_description.shards.clone().unwrap();
    let shard_count = shards.len();
    print_stream_header(&stream_description, client.clone());

    let (tx, mut rx) = mpsc::channel(shards.len());

    for shard in shards {
        let client_clone = client.clone();
        let tx_clone = tx.clone();
        let stream_clone = stream.clone();
        task::spawn(async move {
            listen_to_shard(shard, client_clone, disable_unzip, stream_clone).await;
            tx_clone.send(()).await.unwrap();
        });
    }

    for _ in 0..shard_count {
        rx.recv().await.unwrap();
    }
    Ok(())
}

fn print_stream_header(stream_description: &StreamDescription, client: Arc<Client>) {
    let stream_name = stream_description.stream_name().unwrap();
    println!("Reading Kinesis stream '{}'", stream_name);
    println!();
    println!("Stream information for '{}'", stream_name);
    println!("- Name: {}:", stream_name);
    println!("- ARN: {}:", stream_description.stream_arn.clone().unwrap());
    println!("- Region: {}", client.conf().region().unwrap());
    println!(
        "- Status: {:?}",
        stream_description.stream_status.clone().unwrap()
    );
    println!(
        "- Number of Shards: {:?}",
        stream_description.shards.clone().unwrap().len()
    );
    println!(
        "- Encryption: {:?}",
        stream_description.encryption_type.clone().unwrap()
    );
    println!();
}

async fn listen_to_shard(shard: Shard, client: Arc<Client>, _disable_unzip: bool, stream: String) {
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
            for record in records {
                let data = record.data().expect("Error while reading data").as_ref();
                match record_to_string(data) {
                    Ok(result) => {
                        println!("------------------------------------------------------------");
                        println!("Event received from shard '{}':", &shard_id);
                        println!("Partition Key '{}':", record.partition_key().unwrap());
                        println!("Sequence '{}':", record.sequence_number().unwrap());
                        println!(
                            "Received at '{:?}':",
                            record.approximate_arrival_timestamp().unwrap()
                        );
                        println!("{}", result);
                        println!("------------------------------------------------------------");
                        println!();
                    }
                    Err(error) => eprintln!("{}", error),
                };
            }
        }
        sleep(Duration::from_secs(1));
    }
}

#[derive(Debug, PartialEq)]
pub enum FormatRecordError {
    Default,
    EmptyRecord,
}

impl Display for FormatRecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatRecordError::Default => write!(f, "Default error"),
            FormatRecordError::EmptyRecord => write!(f, "Empty record received"),
        }
    }
}

fn record_to_string(record: &[u8]) -> Result<String, FormatRecordError> {
    if record.is_empty() {
        return Err(FormatRecordError::EmptyRecord);
    }
    let mut buffer = String::new();

    if std::io::Cursor::new(record)
        .read_to_string(&mut buffer)
        .is_ok()
    {
        return match general_purpose::STANDARD.decode(&buffer) {
            Ok(value) => Ok(format_record(
                std::str::from_utf8(value.as_slice()).unwrap().to_string(),
            )),
            Err(_) => Ok(format_record(buffer)),
        };
    }
    buffer.clear();

    if ZlibDecoder::new(record).read_to_string(&mut buffer).is_ok() {
        return Ok(format_record(buffer));
    }
    buffer.clear();

    if GzDecoder::new(record).read_to_string(&mut buffer).is_ok() {
        return Ok(format_record(buffer));
    }
    buffer.clear();

    Err(FormatRecordError::Default)
}

fn format_record(record: String) -> String {
    let result;
    match serde_json::from_str::<Value>(&record) {
        Ok(mut value) => {
            if !value.is_object() {
                value = serde_json::from_str(value.as_str().unwrap()).unwrap();
            }
            result = serde_json::to_string_pretty(&value).unwrap();
        }
        Err(_) => result = record,
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::{GzEncoder, ZlibEncoder};
    use flate2::Compression;
    use std::fs;
    use std::fs::File;

    fn get_test_input(path: &str) -> String {
        fs::read_to_string(path).unwrap().trim().to_string()
    }

    #[test]
    fn test_record_to_string_success_with_json_input() {
        let expected = get_test_input("tests/valid-payload");

        let result = record_to_string(
            fs::read_to_string("tests/valid-payload")
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(result, expected);

        let result = record_to_string(
            fs::read_to_string("tests/inline-payload")
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(result, expected);

        let base64_encoded_input =
            general_purpose::STANDARD.encode(fs::read_to_string("tests/valid-payload").unwrap());
        let result = record_to_string(base64_encoded_input.as_bytes()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_record_to_string_success_with_zlib_encoded_json_input() {
        let expected = get_test_input("tests/valid-payload");

        let mut zlib_encoder = ZlibEncoder::new(
            File::open("tests/valid-payload").unwrap(),
            Compression::fast(),
        );
        let mut buffer = Vec::new();
        zlib_encoder.read_to_end(&mut buffer).unwrap();
        let result = record_to_string(buffer.as_slice()).unwrap();
        assert_eq!(result, expected);

        let mut zlib_encoder = ZlibEncoder::new(
            File::open("tests/inline-payload").unwrap(),
            Compression::fast(),
        );
        buffer.clear();
        zlib_encoder.read_to_end(&mut buffer).unwrap();
        let result = record_to_string(buffer.as_slice()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_record_to_string_success_with_gz_encoded_json_input() {
        let expected = get_test_input("tests/valid-payload");

        let mut zlib_encoder = GzEncoder::new(
            File::open("tests/valid-payload").unwrap(),
            Compression::fast(),
        );
        let mut buffer = Vec::new();
        zlib_encoder.read_to_end(&mut buffer).unwrap();
        let result = record_to_string(buffer.as_slice()).unwrap();
        assert_eq!(result, expected);

        let mut zlib_encoder = GzEncoder::new(
            File::open("tests/inline-payload").unwrap(),
            Compression::fast(),
        );
        buffer.clear();
        zlib_encoder.read_to_end(&mut buffer).unwrap();
        let result = record_to_string(buffer.as_slice()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_record_to_string_error_with_empty_input() {
        let result = record_to_string("".to_string().as_bytes()).err().unwrap();
        assert_eq!(result, FormatRecordError::EmptyRecord);
    }
}
