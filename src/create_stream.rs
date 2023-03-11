use std::collections::HashMap;
use std::sync::Arc;

use aws_sdk_kinesis::{Client, Error};

pub async fn create_stream(
    client: Arc<Client>,
    stream_name: String,
    shard_count: Option<i32>,
) -> Result<(), Error> {
    client
        .create_stream()
        .stream_name(&stream_name)
        .shard_count(shard_count.unwrap_or(1))
        .send()
        .await
        .expect("Error while creating the stream");

    client
        .add_tags_to_stream()
        .stream_name(&stream_name)
        .set_tags(Some(HashMap::from([(
            String::from("created-by"),
            String::from("kinesis-tools-rust"),
        )])))
        .send()
        .await
        .expect("Error while adding stream tag");

    print!("Stream {} successfully created", &stream_name);
    Ok(())
}
