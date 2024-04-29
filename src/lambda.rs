use aws_config::meta::region::RegionProviderChain;
use aws_sdk_lambda::types::InvocationType;
use aws_sdk_lambda::Client;
use aws_smithy_types::Blob;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

/// Invoke a Lambda function
#[pyfunction]
pub fn invoke_lambda_function(function_name: String, payload: String) -> bool {
    Runtime::new()
        .unwrap()
        .block_on(async { ainvoke_lambda_function(function_name, payload).await })
}

/// Invoke a Lambda function asynchronously
async fn ainvoke_lambda_function(function_name: String, payload: String) -> bool {
    let client = get_client().await;

    client
        .invoke()
        .function_name(function_name)
        .payload(Blob::new(payload))
        .invocation_type(InvocationType::Event)
        .send()
        .await
        .is_ok()
}

/// Get the Lambda client using environment variables
async fn get_client() -> Client {
    let region = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region).load().await;
    Client::new(&config)
}
