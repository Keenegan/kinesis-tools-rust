extern crate serde_json;

use aws_sdk_kinesis::{Error};
use clap::{arg, Parser};
use crate::client::{get_client};
use crate::read_stream::read_stream;

mod client;
mod read_stream;

#[derive(Parser, Clone, Debug)]
pub struct Args {
    #[arg(long)]
    stream: String,
    #[arg(long)]
    aws_region: Option<String>,
    #[arg(long)]
    aws_profile: Option<String>,
    #[arg(long)]
    aws_role_arn: Option<String>,
    #[arg(long)]
    aws_session_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let stream = &args.stream;
    let (client, client_config) = get_client(args.clone()).await;
    let _ = read_stream(client, &client_config, stream).await;
    Ok(())
}
