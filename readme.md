# Kinesis-tools-rust

This project allow to read an AWS Kinesis stream


## Run Locally

Clone the project

```bash
  git clone https://github.com/Keenegan/kinesis-tools-rust
```

Go to the project directory

```bash
  cd kinesis-tools-rust
```

List streams

```bash
  cargo run -- list
```

Read stream

```bash
  cargo run -- read --stream <your_stream>
```

Print help
```bash
 cargo run -- --help
```

You can override env variable like this
```bash
  AWS_PROFILE=<your profile> cargo run -- list
```