use std::sync::Arc;

use aws_sdk_kinesis::{Client, Error};

pub async fn list_streams(client: Arc<Client>) -> Result<(), Error> {
    let list_stream_output = match client.list_streams().send().await {
        Ok(list_stream_output) => list_stream_output,
        Err(error) => panic!("Problem listing the streams : {}", error)
    };

    let streams  = list_stream_output.stream_names.unwrap();
    // Fix partial move here
    if list_stream_output.has_more_streams().unwrap() {
        dbg!("more");
    }

    println!("========================================================================================");
    println!("|                                   {} Streams found                                   |", streams.len());
    println!("========================================================================================");

    for stream in &streams {
        println!("  {}", stream);
    }

    Ok(())
}
