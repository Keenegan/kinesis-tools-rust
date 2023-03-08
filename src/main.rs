extern crate serde_json;

use aws_sdk_kinesis::Error;
use clap::{Parser, Subcommand};

use crate::client::{get_client};
use crate::list_streams::list_streams;
use crate::read_stream::read_stream;
use crate::create_stream::create_stream;

mod client;
mod read_stream;
mod list_streams;
mod create_stream;

#[derive(Parser, Clone, Debug)]
#[command(name = "ktr")]
#[command(about = "Kinesis Tools Rust allow to read/write/create a kinesis stream", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands
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
    /// Create a kinesis stream
    Create {
        /// The stream name to create
        #[arg(long)]
        stream: String,
        #[arg(long)]
        shard_count: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let client = get_client(args.clone()).await;

    match args.command {
        Commands::Read { stream} => {
            let _ = read_stream(client, &stream).await;
        },
        Commands::List {} => {
            let _ = list_streams(client).await;
        },
        Commands::Create { stream, shard_count} => {
            let _ = create_stream(client, stream, shard_count).await;
        },
    }
    Ok(())
}