#![allow(dead_code, unused_variables, unused)]
extern crate serde_json;

use std::io::prelude::*;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use aws_sdk_kinesis::{Client, Error};
use aws_sdk_kinesis::model::{Shard, ShardIteratorType};
use clap::{arg, Parser};
use flate2::read::ZlibDecoder;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::task;

use crate::client::{ClientConfig, get_client};
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
