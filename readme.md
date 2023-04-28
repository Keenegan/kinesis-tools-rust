# kinesis-tools-rust

This project allows you to interact with [AWS Kinesis stream](https://aws.amazon.com/fr/kinesis/data-streams/)  

## Usage

```bash
Kinesis Tools Rust allow to read/write/create a kinesis stream

Usage: ktr <COMMAND>

Commands:
  list    List all streams
  read    Read upcoming events from a stream
  create  Create a new stream
  delete  Delete a stream
  put     Put record into a stream
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

You need to provide `AWS_PROFILE` as an environment variable
```bash
  AWS_PROFILE=<your profile> cargo run -- list
```

 ## Download
Download the latest [release](https://github.com/Keenegan/kinesis-tools-rust/releases/latest) for your computer

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
AWS_PROFILE=<your_profile> cargo run -- list
```

## Currently supported targets
```
i686-unknown-linux-gnu - 32-bit Linux (kernel 3.2+, glibc 2.17+)
x86_64-unknown-linux-gnu - 64-bit Linux (kernel 3.2+, glibc 2.17+)
x86_64-apple-darwin - 64-bit macOS (10.7+, Lion+)
```
