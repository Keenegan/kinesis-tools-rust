extern crate serde_json;

use aws_sdk_kinesis::Error;
use clap::{Parser, Subcommand};

use crate::client::get_client;
use crate::list_streams::list_streams;
use crate::read_stream::read_stream;

mod client;
mod read_stream;
mod list_streams;

#[derive(Parser, Clone, Debug)]
#[command(name = "ktr")]
#[command(about = "Kinesis Tools Rust allow to read/write/create a kinesis stream", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    region: Option<String>,
    #[arg(long)]
    profile: Option<String>,
    #[arg(long)]
    role_arn: Option<String>,
    #[arg(long)]
    session_name: Option<String>,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    /// List all kinesis streams
    List {},
    /// Read a kinesis stream
    Read {
        /// The stream name to read
        #[arg(long)]
        stream: String
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let (client, client_config) = get_client(args.clone()).await;
    match args.command {
        Commands::Read { stream} => {
            let _ = read_stream(client, &client_config, &stream).await;
        },
        Commands::List {} => {
            list_streams(client, &client_config).await;
        }
    }
    Ok(())
}
