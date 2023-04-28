# kinesis-tools-rust

This project allows you to interact with [AWS Kinesis stream](https://aws.amazon.com/fr/kinesis/data-streams/)  

## Usage

```bash
KTR (Kinesis Tools Rust) allows you to interact with AWS Kinesis data streams

Usage: ktr <AWS_PROFILE> <COMMAND>

Commands:
  list    Lists your Kinesis data streams
  read    Gets data records from a Kinesis data stream
  create  Creates a Kinesis data stream
  delete  Deletes a Kinesis data stream and all its shards and data
  put     Writes a single data record into an Amazon Kinesis data stream
  help    Print this message or the help of the given subcommand(s)

Arguments:
  <AWS_PROFILE>  Which AWS profile to use

Options:
  -h, --help     Print help
  -V, --version  Print version
```

 ## Download
Download the latest [release](https://github.com/Keenegan/kinesis-tools-rust/releases/latest) for your computer
Make the downloaded binary executable
Then you should be able to run `./ktr` to see help message

## Build from source
If you don't already have Rust on your computer go to https://www.rust-lang.org/tools/install to install it

```bash
git clone https://github.com/Keenegan/kinesis-tools-rust
cd kinesis-tools-rust
cargo build --release
./target/release/ktr help
```  

## Run from source
```bash
cargo run -- <AWS_PROFILE> list
```

## Currently supported targets
```
i686-unknown-linux-gnu - 32-bit Linux (kernel 3.2+, glibc 2.17+)
x86_64-unknown-linux-gnu - 64-bit Linux (kernel 3.2+, glibc 2.17+)
x86_64-apple-darwin - 64-bit macOS (10.7+, Lion+)
```
