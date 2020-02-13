#![feature(str_strip)]
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
        Err(_) => return Err(())
    };
    // AWS region. Also mandatory.
    let region = match std::env::var("AWS_REGION") {
        Ok(s) => match Region::from_str(&s) {
            Ok(reg) => reg,
            Err(_) => {
                eprintln!("Could not parse the given AWS region.");
                return Err(())
            }
        },
        Err(_) => return Err(())
    };
    // Which functions to auto-update. If not provided, will try to update every .zip matching the name of a function.
    let monitored_names = std::env::var("FUNCTION_NAMES").ok();
    // Which folder to watch for changes in. If not provided, all new .zips will be checked.
    let monitored_folder = std::env::var("DIRECTORY").ok();
    if monitored_names.is_none() && monitored_folder.is_none() {
        eprintln!("You must provide at least one of FUNCTION_NAMES and DIRECTORY.");
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
    let result = handler();
    dbg!(result);
}
