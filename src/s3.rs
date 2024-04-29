use std::path::Path;
use std::time::Duration;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

/// Check if a file exists on S3
#[pyfunction]
pub fn check_exists(bucket: String, key: String) -> bool {
    Runtime::new()
        .unwrap()
        .block_on(async { acheck_exists(bucket, key).await })
}

/// Check if a file exists on S3 asynchronously
async fn acheck_exists(bucket: String, key: String) -> bool {
    let client = get_client().await;

    client
        .head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .is_ok()
}

/// Upload a file to S3
#[pyfunction]
pub fn upload_file(filename: String, bucket: String, key: String) -> bool {
    Runtime::new()
        .unwrap()
        .block_on(async { aupload_file(filename, bucket, key).await })
}

/// Upload a file asynchronously to S3
async fn aupload_file(filename: String, bucket: String, key: String) -> bool {
    let path = Path::new(&filename);
    let body = ByteStream::from_path(path).await;
    let client = get_client().await;

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
        .is_ok()
}

/// Generate a presigned URL for an S3 file
#[pyfunction]
pub fn generate_presigned_url(bucket: String, key: String) -> String {
    Runtime::new()
        .unwrap()
        .block_on(async { agenerate_presigned_url(bucket, key).await })
}

/// Generate a presigned URL for an S3 file asynchronously
async fn agenerate_presigned_url(bucket: String, key: String) -> String {
    let client = get_client().await;
    let expires_in = Duration::from_secs(3600);
    let config = PresigningConfig::expires_in(expires_in).unwrap();

    client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(config)
        .await
        .unwrap()
        .uri()
        .to_string()
}

/// Get the S3 client using environment variables
async fn get_client() -> Client {
    let region = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region).load().await;
    Client::new(&config)
}
