/*
 * Copyright (c) 2023 Markus Neifer.
 * Licensed under the MIT License.
 * See LICENSE in the project root for license information.
 *
 * Adopted from https://pyo3.rs/v0.18.1/
 */
// The following line allows non snake-case name for Robot test library
#![allow(non_snake_case)]

use std::collections::HashMap;

use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: i32, b: i32) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn join_strings(a: Vec<String>) -> PyResult<String> {
    Ok(a.join(","))
}

#[pyfunction]
fn sum_values(a: HashMap<String, i32>) -> PyResult<i32> {
    let mut values_sum = 0;
    for (_key, value) in &a {
        values_sum += value;
    }
    Ok(values_sum)
}

#[pymodule]
fn RustyLibrary(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(join_strings, m)?)?;
    m.add_function(wrap_pyfunction!(sum_values, m)?)?;
    Ok(())
}
