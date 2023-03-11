use std::sync::Arc;

use aws_sdk_kinesis::{Client, Error};

pub async fn list_streams(client: Arc<Client>) -> Result<(), Error> {
    let list_stream_output = client
        .list_streams()
        .send()
        .await
        .expect("Problem listing the streams");
    let streams = list_stream_output.stream_names().unwrap();

    println!(
        "========================================================================================"
    );
    println!(
        "|                                   {} Stream(s) found                                 |",
        streams.len()
    );
    println!(
        "========================================================================================"
    );

    if list_stream_output.has_more_streams().unwrap() {
        // TODO print all streams
        println!("There is more than {} streams.", streams.len());
    }
    for stream in streams {
        println!("  {}", stream);
    }

    Ok(())
}
