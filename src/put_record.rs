use std::path::PathBuf;
use std::sync::Arc;

use aws_sdk_kinesis::primitives::Blob;
use aws_sdk_kinesis::{Client, Error};

pub async fn put_record(
    client: Arc<Client>,
    stream_name: String,
    path: PathBuf,
    shard_key: Option<String>,
) -> Result<(), Error> {
    let content = std::fs::read(path).expect("Could not read file");
    if content.is_empty() {
        panic!("File is empty")
    }
    let blob = Blob::new(content);
    client
        .put_record()
        .data(blob)
        .partition_key(shard_key.unwrap_or("random".to_string()))
        .stream_name(stream_name)
        .send()
        .await
        .expect("Could not push data into stream");

    println!("Data successfully pushed into stream");

    Ok(())
}
