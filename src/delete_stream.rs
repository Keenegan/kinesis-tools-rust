use std::sync::Arc;

use aws_sdk_kinesis::{Client, Error};

pub async fn delete_stream(client: Arc<Client>, stream_name: String) -> Result<(), Error> {
    print!("Delete stream {}", &stream_name);
    let _ = client.delete_stream().stream_name(stream_name).send().await?;
    Ok(())
}
