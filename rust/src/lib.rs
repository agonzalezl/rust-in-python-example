use pyo3::prelude::*;

#[pyfunction]
fn fibonacci(n: usize) -> PyResult<usize> {
    Ok(_fibonacci(n))
}

fn _fibonacci(n: usize) -> usize {
    if n <= 1 {
        return n;
    } else {
        return _fibonacci(n - 1) + _fibonacci(n - 2);
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_fibonacci(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;

    Ok(())
}
