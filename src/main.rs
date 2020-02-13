#![feature(str_strip)]
#[macro_use]
extern crate log;
use rusoto_lambda::{LambdaClient, GetFunctionRequest, GetFunctionError, GetFunctionResponse, Lambda};
use rusoto_core::{HttpClient, RusotoError, credential::{EnvironmentProvider, ProvideAwsCredentials}, Region};
use std::convert::TryFrom;
use std::str::FromStr;

mod util;


#[tokio::main]
async fn handler() -> Result<(), ()> {
    // Which bucket to watch. Mandatory.
    let bucket = match std::env::var("BUCKET") {
        Ok(s) => s,
        Err(_) => {
            error!("A BUCKET environment variable must be provided.");
            return Err(())}
    };
    // AWS region. Also mandatory.
    let region = match std::env::var("AWS_REGION") {
        Ok(s) => match Region::from_str(&s) {
            Ok(reg) => reg,
            Err(_) => {
                error!("Could not parse the given AWS region.");
                return Err(())
            }
        },
        Err(_) => {
            error!("The AWS_REGION environment variable must be provided.");
            return Err(())
        }
    };
    /*
     Which functions to auto-update, separated by a comma.
     If not provided, will try to update every .zip matching the name of an existent function
     in the chosen directory.
     */
    let monitored_names: Option<Vec<String>> = std::env::var("FUNCTION_NAMES").ok().map(|s|
        s
            .split(":")
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    );
    monitored_names.as_ref().and_then(|s| {
        info!("Names to be watched for:");
        for each in s {
            info!("{}", each);
        }
        Some(())
    });
    // Which directory to watch for changes in. If not provided, all new .zips in the bucket matching monitored_names will be checked for.
    let monitored_dir = std::env::var("DIRECTORY").ok();
    if monitored_names.is_none() && monitored_dir.is_none() {
        error!("You must provide at least one of FUNCTION_NAMES and DIRECTORY.");
        return Err(())
    }
    let lambda_client = LambdaClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        region
    );
    Ok(())
}
fn main() {
    pretty_env_logger::init();
    let result = handler();
    dbg!(result);
}
