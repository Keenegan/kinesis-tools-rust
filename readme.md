# kinesis-tools-rust

This project allows you to interact with AWS Kinesis stream


## Run Locally

```bash
  git clone https://github.com/Keenegan/kinesis-tools-rust
  cd kinesis-tools-rust
```

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

You can override env variable
```bash
  AWS_PROFILE=<your profile> cargo run -- list
```