use rusoto_lambda::{LambdaClient, GetFunctionError, GetFunctionRequest, Lambda};
use rusoto_core::RusotoError;
use std::env::var;
use lambda_runtime::error::HandlerError;

mod test;
/// Extracts the name of the Lambda function being updated from the code zip's full path and filename.
pub fn bare_name(path: &str) -> Option<String> {
    // explicit type needed here because CLion can't handle Splits
    let x: Option<&str> = path.split('/').into_iter().last();
    x
        .and_then(|s| s.strip_suffix(".zip"))
        .map(|s| s.to_owned())
}
pub async fn function_exists(name: &str, client: &LambdaClient) -> Result<bool, HandlerError> {
    let mut request = GetFunctionRequest::default();
    request.function_name = name.to_owned();
    let response = client.get_function(request).await;
    match response {
        Ok(_) => Ok(true),
        Err(e) => {
            match &e {
                RusotoError::Service(ser) => match ser {
                    GetFunctionError::ResourceNotFound(_) => Ok(false),
                    _ => Err(HandlerError::from("Function existence check failed."))
                },
                _ => Err(HandlerError::from("Function existence check failed."))
            }
        }
    }
}
/// Gets the given environment variable or displays an error and returns one.
pub fn get_env_var(key: &str) -> Result<String, HandlerError> {
    match var(key) {
        Ok(s) => Ok(s),
        Err(_) => {
            error!("The {} environment variable must be set.", key);
            Err(HandlerError::from(format!("{} environment variable not set", key).as_str()))
        }
    }
}
/// Gets the given environment variable if it exists, otherwise returns `None`.
pub fn opt_env_var(key: &str) -> Option<String> {
    var(key).ok()
}