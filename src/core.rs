use crate::util::{bare_name, function_exists, get_env_var, opt_env_var, dir_name};
use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{Context, error::HandlerError};
use rusoto_core::{Region, credential::EnvironmentProvider, HttpClient};
use rusoto_lambda::{LambdaClient, Lambda, UpdateFunctionCodeRequest};
use std::str::FromStr;

#[tokio::main]
pub async fn handler(ev: S3Event, _c: Context) -> Result<String, HandlerError> {
    // AWS region. Mandatory.
    let region = match Region::from_str(&get_env_var("AWS_REGION")?) {
        Ok(reg) => reg,
        Err(_) => {
            error!("Could not parse the given AWS region.");
            return Err(HandlerError::from("Invalid AWS region."))
        }
    };
    /*
     Which functions to auto-update, separated by a comma.
     If not provided, will try to update every .zip matching the name of an existent function
     in the chosen directory.
     */
    let monitored_names: Option<Vec<String>> = opt_env_var("FUNCTION_NAMES").map(|s| {
        s
            .split(":")
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    }
    );
    monitored_names.as_ref().and_then(|s| {
        info!("Names to be watched for:");
        for each in s {
            info!("{}", each);
        }
        Some(())
    });
    // Which directory to watch for changes in. If not provided, all new .zips in the bucket matching monitored_names will be checked for.
    let monitored_dir = opt_env_var("DIRECTORY");
    // Either of the above must be present.
    if monitored_names.is_none() && monitored_dir.is_none() {
        error!("You must provide at least one of FUNCTION_NAMES and DIRECTORY.");
        return Err(HandlerError::from("Neither function names nor source directory given."))
    }

    let lambda_client = LambdaClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        region
    );

    for each in ev.records {
        let key = match each.s3.object.key {
            Some(s) => {
                info!("Received S3 event with key {}", &s);
                s
            }
            None => {
                info!("Received S3 event with no key, ignoring.");
                continue
            }
        };
        // Whether or not the file is in the directory we're interested in. Defaults to true if we're interested in all of them.
        let is_in_dir = match &monitored_dir {
            Some(dir) => &dir_name(&key) == dir,
            _ => true
        };
        // Whether or not the file is named correctly. If no names were specified, all .ZIP files are allowed.
        let is_named = match bare_name(&key) {
            None => false,
            Some(name) => match &monitored_names {
                None => true,
                Some(names) => names.contains(&name)
            }
        };
        let update = is_in_dir && is_named;
        if update {
            info!("The file matches the criteria and is a candidate for a function update.");
            let name = &bare_name(&key).unwrap();
            let function_exists = match function_exists(name, &lambda_client).await {
                Err(e) => {
                    error!("Could not check if the function named {} exists!", name);
                    error!("{}", e);
                    return Err(HandlerError::from("Lambda existence check failure"))
                },
                Ok(x) => x
            };
            if function_exists {
                info!("Attempting to update the function {}...", name);
                let mut req = UpdateFunctionCodeRequest::default();
                req.function_name = name.clone();
                req.s3_bucket = each.s3.bucket.name.clone();
                req.s3_key = Some(key);
                let response = lambda_client.update_function_code(req).await;
                match response {
                    Ok(_) => {
                        info!("Function successfully updated!");
                    },
                    Err(e) => {
                        error!("Could not update the function {}!", name);
                        error!("{}", e);
                        return Err(HandlerError::from("Lambda code update failure"))
                    }
                }
            } else {
                info!("There is no function named {} that would correspond to the code archive.", name);
            }
        } else {
            info!("Ignoring file {}", key);
        }
    }
    Ok("Function successfully executed".to_owned())
}