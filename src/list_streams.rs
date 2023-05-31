use std::sync::Arc;

use aws_sdk_kinesis::{Client, Error};

pub async fn list_streams(client: Arc<Client>) -> Result<(), Error> {
    let list_stream_output = client
        .list_streams()
        .send()
        .await
        .expect("Error listing the streams");

    println!("------------------------------------------------------------");
    println!(
        "Listing AWS Kinesis Data Streams in {}",
        client.conf().region().unwrap()
    );
    println!("------------------------------------------------------------");
    for stream_summary in list_stream_output.stream_summaries().unwrap() {
        println!("- Name: {}", stream_summary.stream_name().unwrap());
        println!("- ARN: {}", stream_summary.stream_arn().unwrap());
        println!(
            "- Status: {}",
            stream_summary.stream_status().unwrap().as_str()
        );
        println!(
            "- Mode: {}",
            stream_summary
                .stream_mode_details()
                .unwrap()
                .stream_mode()
                .unwrap()
                .as_str()
        );
        println!("------------------------------------------------------------");
    }

    Ok(())
}
