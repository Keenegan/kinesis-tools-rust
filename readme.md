# Kinesis-tools-rust

This project allow to read an AWS Kinesis stream


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
  list    List all kinesis streams
  read    Read a kinesis stream
  create  Create a kinesis stream
  delete  Delete a kinesis stream
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

You can override env variable
```bash
  AWS_PROFILE=<your profile> cargo run -- list
```