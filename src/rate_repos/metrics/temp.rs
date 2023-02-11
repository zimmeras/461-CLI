use git2::Repository;
use std::fs;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::conversion::FromPyObject;

//git url here
pub fn clone_repo(url: &str) {
     let _file = match fs::create_dir_all("./src/rate_repos/metrics/repos") {
        Ok(_file) => _file,
        Err(e) => panic!("failed to create temporary directory: {}", e),
    };
    let _repo = match Repository::clone(url, "./src/rate_repos/metrics/repos") {
        Ok(_repo) => _repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

pub fn calculate_correctness(url: &str) -> i32 {
    clone_repo(url);
    let code = include_str!("./crt-analysis.py");
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let fun: Py<PyAny> = PyModule::from_code(py, code, "", "")?
            .getattr("calculate_correctness")?
            .into();
        fun.call1(py, PyString::new(py, url).into_py(py))
    });
    let val: pyo3::Py<PyAny> = from_python.unwrap();
    delete_repo();
    let num = FromPyObject::extract(&val);
    num.unwrap()
}

pub fn delete_repo() {
    let _file = match fs::remove_dir_all("./src/rate_repos/metrics/repos") {
        Ok(_file) => _file,
        Err(e) => panic!("failed to remove temporary directory: {}", e),
    };
}