pub mod prelude;

use pyo3::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[pyfunction]
fn _version() -> PyResult<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn glama(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_version, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
