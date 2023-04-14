use std::path::PathBuf;
use std::sync::Arc;

use aws_sdk_kinesis::primitives::Blob;
use aws_sdk_kinesis::{Client, Error};

pub async fn put_record(
    client: Arc<Client>,
    stream_name: String,
    path: PathBuf,
) -> Result<(), Error> {
    // TODO add param to specify shard key
    let key = "random";
    let content = std::fs::read(path).expect("Could not read file");
    if content.is_empty() {
        panic!("File is empty")
    }
    let blob = Blob::new(content);
    client
        .put_record()
        .data(blob)
        .partition_key(key)
        .stream_name(stream_name)
        .send()
        .await
        .expect("Could not push data into stream");

    Ok(())
}
