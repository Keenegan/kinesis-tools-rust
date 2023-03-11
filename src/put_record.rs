use std::path::PathBuf;
use std::sync::Arc;

use aws_sdk_kinesis::types::Blob;
use aws_sdk_kinesis::{Client, Error};

pub async fn put_record(
    client: Arc<Client>,
    stream_name: String,
    path: PathBuf,
) -> Result<(), Error> {
    let key = "random";
    let content = std::fs::read_to_string(path).expect("Could not read file");
    dbg!(&content);
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
