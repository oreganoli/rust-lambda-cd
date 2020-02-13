#![feature(str_strip)]
#[macro_use]
extern crate log;
use rusoto_lambda::{LambdaClient, GetFunctionRequest, GetFunctionError, GetFunctionResponse, Lambda};
use rusoto_core::{HttpClient, RusotoError, credential::{EnvironmentProvider, ProvideAwsCredentials}, Region};
use std::convert::TryFrom;
use std::str::FromStr;


mod test;
/// Extracts the name of the Lambda function being updated from the code zip's full path and filename.
fn bare_name(path: &str) -> Option<String> {
    // explicit type needed here because CLion can't handle Splits
    let x: Option<&str> = path.split("/").into_iter().last();
    x
        .and_then(|s| s.strip_suffix(".zip"))
        .map(|s| s.to_owned())
}

async fn function_exists(name: &str, client: &LambdaClient) -> Result<bool, RusotoError<GetFunctionError>> {
    let mut request = GetFunctionRequest::default();
    request.function_name = name.to_owned();
    let response = client.get_function(request).await;
    match response {
        Ok(_) => Ok(true),
        Err(e) => {
            match &e {
                RusotoError::Service(ser) => match ser {
                    GetFunctionError::ResourceNotFound(_) => Ok(false),
                    _ => Err(e)
                },
                _ => Err(e)
            }
        }
    }
}
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
    // Which functions to auto-update, separated by a comma. If not provided, will try to update every .zip matching the name of a function.
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
    // Which folder to watch for changes in. If not provided, all new .zips will be checked.
    let monitored_folder = std::env::var("DIRECTORY").ok();
    if monitored_names.is_none() && monitored_folder.is_none() {
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
