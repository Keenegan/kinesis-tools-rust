use std::sync::Arc;

use aws_sdk_kinesis::{Client, Error};

pub async fn create_stream(client: Arc<Client>, stream_name: String, shard_count: Option<i32>) -> Result<(), Error> {
    print!("Create stream {}", &stream_name);
    let stream = client
        .create_stream()
        .stream_name(stream_name)
        .shard_count(shard_count.unwrap_or(1))
        .send()
        .await?;
    dbg!(&stream);
    Ok(())
}
