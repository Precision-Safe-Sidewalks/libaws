use pyo3::prelude::*;

pub mod lambda;
pub mod s3;

#[pymodule]
fn libpss_aws(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(s3::check_exists, m)?)?;
    m.add_function(wrap_pyfunction!(s3::upload_file, m)?)?;
    m.add_function(wrap_pyfunction!(s3::generate_presigned_url, m)?)?;

    m.add_function(wrap_pyfunction!(lambda::invoke_lambda_function, m)?)?;

    Ok(())
}
