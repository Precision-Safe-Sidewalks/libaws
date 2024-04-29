use std::path::Path;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

#[pyfunction]
fn check_exists(bucket: String, key: String) -> bool {
    Runtime::new()
        .unwrap()
        .block_on(async { acheck_exists(bucket, key).await })
}

async fn acheck_exists(bucket: String, key: String) -> bool {
    let client = get_s3_client().await;
    client
        .head_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .is_ok()
}

#[pyfunction]
fn upload_file(filename: String, bucket: String, key: String) -> bool {
    Runtime::new()
        .unwrap()
        .block_on(async { aupload_file(filename, bucket, key).await })
}

async fn aupload_file(filename: String, bucket: String, key: String) -> bool {
    let path = Path::new(&filename);
    let body = ByteStream::from_path(path).await;
    let client = get_s3_client().await;

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
        .is_ok()
}

async fn get_s3_client() -> aws_sdk_s3::Client {
    let region = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region).load().await;
    aws_sdk_s3::Client::new(&config)
}

#[pymodule]
fn libpss_aws(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_exists, m)?)?;
    m.add_function(wrap_pyfunction!(upload_file, m)?)?;

    Ok(())
}
