use rusoto_lambda::{LambdaClient, GetFunctionError, GetFunctionRequest, Lambda};
use rusoto_core::RusotoError;

mod test;
/// Extracts the name of the Lambda function being updated from the code zip's full path and filename.
pub fn bare_name(path: &str) -> Option<String> {
    // explicit type needed here because CLion can't handle Splits
    let x: Option<&str> = path.split("/").into_iter().last();
    x
        .and_then(|s| s.strip_suffix(".zip"))
        .map(|s| s.to_owned())
}

pub async fn function_exists(name: &str, client: &LambdaClient) -> Result<bool, RusotoError<GetFunctionError>> {
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