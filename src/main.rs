extern crate serde_json;

use aws_sdk_kinesis::Error;
use clap::{Parser, Subcommand};

use crate::client::get_client;
use crate::create_stream::create_stream;
use crate::delete_stream::delete_stream;
use crate::list_streams::list_streams;
use crate::put_record::put_record;
use crate::read_stream::read_stream;

mod client;
mod create_stream;
mod delete_stream;
mod list_streams;
mod put_record;
mod read_stream;

#[derive(Parser, Clone, Debug)]
#[command(name = "ktr")]
#[command(about = "Kinesis Tools Rust allow to read/write/create a kinesis stream", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    /// List all streams
    List {},
    /// Read upcoming events from a stream
    Read {
        /// The stream name to read
        #[arg(long, short)]
        stream: String,
        /// Disable the automatic unzip of received events
        #[arg(long, short)]
        disable_unzip: bool,
    },
    /// Create a new stream
    Create {
        /// The stream name to create
        #[arg(long, short)]
        stream: String,
        #[arg(long)]
        shard_count: Option<i32>,
    },
    /// Delete a stream
    Delete {
        /// The stream name to delete
        #[arg(long, short)]
        stream: String,
    },
    /// Put record into a stream
    Put {
        /// The stream where data will be sent
        #[arg(long, short)]
        stream: String,
        /// The path to a file containing a json payload
        #[arg(long, short)]
        path: std::path::PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let args = Args::parse();
    let client = get_client(args.clone()).await;

    let _ = match args.command {
        Commands::Read {
            stream,
            disable_unzip,
        } => read_stream(client, disable_unzip, &stream).await,
        Commands::List {} => list_streams(client).await,
        Commands::Create {
            stream,
            shard_count,
        } => create_stream(client, stream, shard_count).await,
        Commands::Delete { stream } => delete_stream(client, stream).await,
        Commands::Put { stream, path } => put_record(client, stream, path).await,
    };
    Ok(())
}
