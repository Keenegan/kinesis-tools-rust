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
#[command(about = "KTR (Kinesis Tools Rust) allow to interact with AWS Kinesis data streams", long_about = None)]
pub struct Args {
    /// AWS profile
    #[clap(required = true, name = "AWS_PROFILE")]
    profile: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    /// Lists your Kinesis data streams
    List {},
    /// Gets data records from a Kinesis data stream
    Read {
        /// The name of the stream to read
        #[arg(long, short)]
        stream: String,
        /// Disable the automatic unzip of received events
        #[arg(long, short)]
        disable_unzip: bool,
    },
    /// Creates a Kinesis data stream
    Create {
        /// The name of the stream to create
        #[arg(long, short)]
        stream: String,
        /// The number of shards that the stream will use
        #[arg(long)]
        shard_count: Option<i32>,
    },
    /// Deletes a Kinesis data stream and all its shards and data
    Delete {
        /// The name of the stream to delete
        #[arg(long, short)]
        stream: String,
    },
    /// Writes a single data record into an Amazon Kinesis data stream
    Put {
        /// The name of the stream to put the data record into
        #[arg(long, short)]
        stream: String,
        /// The path to a file containing the data record
        #[arg(long, short)]
        path: std::path::PathBuf,
        /// Determines which shard in the stream the data record is assigned to (Optional)
        #[arg(long, short = 'k')]
        shard_key: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    let args = Args::parse();
    let client = get_client(args.clone()).await;

    let _ = match args.command {
        //TODO add Describe command
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
        Commands::Put {
            stream,
            path,
            shard_key,
        } => put_record(client, stream, path, shard_key).await,
    };
    Ok(())
}
