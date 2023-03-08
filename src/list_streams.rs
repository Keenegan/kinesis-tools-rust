use std::sync::Arc;

use aws_sdk_kinesis::{Client, Error};

pub async fn list_streams(client: Arc<Client>) -> Result<(), Error> {
    let resp = client.list_streams().send().await.expect("No stream found");
    let streams = resp.stream_names.unwrap();

    println!("========================================================================================");
    println!("|                                   {} Streams found                                   |", streams.len());
    println!("========================================================================================");

    for stream in &streams {
        println!("  {}", stream);
    }

    Ok(())
}
