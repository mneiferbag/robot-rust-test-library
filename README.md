# Robot Framework - Test Library Example Rust

Licensed under the MIT License. See file [LICENSE](./LICENSE).

[Robot Framework](https://robotframework.org/) example for test library implemented in Rust. Example includes code written in Robot Framework, Python and Rust.

[![CodeQL](https://github.com/mneiferbag/robot-rust-test-library/actions/workflows/codeql.yml/badge.svg)](https://github.com/mneiferbag/robot-rust-test-library/actions/workflows/codeql.yml)

## How To Run

Create virtual environment with `python -m venv .venv`.

Restore packages with `pip install -r requirements.txt`.

Check installation with `robot --version`.

Build Rust library with `maturin develop`. Note: PyO3 uses [maturin](https://www.maturin.rs/), a tool for building and publishing Rust-based Python packages.

Run test with Rust keywords using `robot --outputdir ./log ./rust_tests/rust_keywords.robot`.

## Links

* Robot Framework
  * [FTP and Database Example](https://github.com/mneiferbag/robot-ftp-db)
* Rust
  * [PyO3](https://pyo3.rs/) - Rust bindings for Python, including tools for creating native Python extension modules

## Tasks

[x] Create hello world example
