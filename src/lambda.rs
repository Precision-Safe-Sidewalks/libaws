use aws_config::meta::region::RegionProviderChain;
use aws_sdk_lambda::types::InvocationType;
use aws_sdk_lambda::Client;
use aws_smithy_types::Blob;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

/// Invoke a Lambda function
#[pyfunction]
pub fn invoke_lambda_function(
    function_name: String,
    payload: String,
    endpoint_url: Option<String>,
    wait: Option<bool>,
) -> bool {
    Runtime::new().unwrap().block_on(async {
        ainvoke_lambda_function(function_name, payload, endpoint_url, wait).await
    })
}

/// Invoke a Lambda function asynchronously
async fn ainvoke_lambda_function(
    function_name: String,
    payload: String,
    endpoint_url: Option<String>,
    wait: Option<bool>,
) -> bool {
    let client = get_client(endpoint_url).await;

    let mut invocation_type = InvocationType::RequestResponse;

    if let Some(wait) = wait {
        if !wait {
            invocation_type = InvocationType::Event;
        }
    }

    client
        .invoke()
        .function_name(function_name)
        .payload(Blob::new(payload))
        .invocation_type(invocation_type)
        .send()
        .await
        .is_ok()
}

/// Get the Lambda client using environment variables
async fn get_client(endpoint_url: Option<String>) -> Client {
    let region = RegionProviderChain::default_provider().or_else("us-east-1");
    let mut params = aws_config::from_env().region(region);

    if let Some(url) = endpoint_url {
        params = params.endpoint_url(url);
    }

    let config = params.load().await;
    Client::new(&config)
}
