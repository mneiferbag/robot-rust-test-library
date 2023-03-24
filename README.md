# Robot Framework - Test Library Example Rust

Licensed under the MIT License. See file [LICENSE](./LICENSE).

[Robot Framework](https://robotframework.org/) example for test library implemented in Rust. Example includes code written in Robot Framework, Python and Rust.

## How To Run

This example can be run as follows.

Create virtual environment with `python -m venv .venv`.

Activate virtual environment with `source .venv/bin/activate` for Linux bash. Or with `.venv\Scripts\activate.bat` for Windows command line.

Restore packages with `pip install -r requirements.txt`.

Check installation with `robot --version`.

Build Rust library with `maturin develop`. Note: PyO3 uses [maturin](https://www.maturin.rs/), a tool for building and publishing Rust-based Python packages.

Run test with Rust keywords using `robot --outputdir ./log ./rust_tests/rust_keywords.robot`.

For more information on extending Robot Framework with test libraries written in Python, see chapter [Creating test libraries](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#creating-test-libraries) in the user guide.

## Links

* Robot Framework
  * [FTP and Database Example](https://github.com/mneiferbag/robot-ftp-db)
  * [Test Library Example Python](https://github.com/mneiferbag/robot-python-test-library)
* Rust
  * [PyO3](https://pyo3.rs/) - Rust bindings for Python, including tools for creating native Python extension modules

## Tasks

- [ ] Evaluate keyword with dictionary argument
- [ ] Evaluate default argument values
- [x] Create hello world example
