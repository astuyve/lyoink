use crate::utils::arn;
use aws_config::BehaviorVersion;
use aws_sdk_lambda as lambda;
use clap::Parser;
use lambda::error::SdkError;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    arn: String,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, default_value = "output.zip")]
    dest: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let parsed_arn = arn::parse_arn(&args.arn).expect("Invalid ARN");
    let dest = args.dest.expect("no path");
    let resource_url = match parsed_arn.arn_type {
        arn::ArnType::Function => get_function_url(&parsed_arn.arn).await.expect("Failed to get function"),
        arn::ArnType::Layer => get_layer_url(&parsed_arn.arn).await.expect("Failed to get layer"),
    };

    let response = reqwest::get(&resource_url)
        .await
        .expect("Failed to download layer")
        .bytes()
        .await
        .expect("unexpected file type");
    let path = Path::new(&dest);
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    file.write_all(&response).expect("Failed to write zip");
}

async fn get_function_url(arn: &str) -> Result<String, String> {
    let region = arn::get_region(arn);
    let region_provider = aws_config::Region::new(region);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = lambda::Client::new(&config);

    let resp = client.get_function().function_name(arn).send().await;
    match resp {
        Ok(resp) => Ok(resp
            .code
            .expect("no code")
            .location
            .expect("no location")
            .to_string()),
        Err(e) => match e {
            SdkError::DispatchFailure(e) => {
                println!("Error as connector error {:?}", e.as_connector_error().expect("no connector error"));
                Err("No credentials found".to_string())
            },
            SdkError::ServiceError(e) => {
                println!("Error as service error {:?}", e.into_err());
                Err("Service error".to_string())
            },
            _ => Err(e.to_string())
        }
    }
}
async fn get_layer_url(arn: &str) -> Result<String, String> {
    let region = arn::get_region(arn);
    let region_provider = aws_config::Region::new(region);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = lambda::Client::new(&config);

    let resp = client.get_layer_version_by_arn().arn(arn).send().await;
    match resp {
        Ok(resp) => Ok(resp
            .content
            .expect("no content")
            .location
            .expect("no location")
            .to_string()),
        Err(e) => match e {
            SdkError::DispatchFailure(e) => {
                println!("Error as connector error {:?}", e.as_connector_error().expect("no connector error"));
                Err("No credentials found".to_string())
            },
            SdkError::ServiceError(e) => {
                println!("Error as service error {:?}", e.into_err());
                Err("Service error".to_string())
            },
            _ => Err(e.to_string())
        }
    }
}
