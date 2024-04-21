# L(lambda, layer) yoink

Lyoink is a CLI tool which downloads the source zip for Lambda Functions or Lambda Layers by ARN.

## Installation

```bash
cargo build --release
./lyoink <arn> -d <output_dir/filename>
```

## Why Rust?
I wanted to try out the AWS SDK for Rust and I fetch a lot of lambda layers and function code.
