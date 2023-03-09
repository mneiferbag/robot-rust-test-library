/*
 * Copyright (c) 2023 Markus Neifer.
 * Licensed under the MIT License.
 * See LICENSE in the project root for license information.
 * 
 * Adopted from https://pyo3.rs/v0.18.1/
 */
use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn robot_rust_test_library(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

