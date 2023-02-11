use git2::Repository;
use std::fs;
use cpython::{PyResult, Python};
use cpython::ObjectProtocol;

//git url here
fn clone_repo(url: &str) {
     let _file = match fs::create_dir_all("./src/rate_repos/metrics/repos") {
        Ok(_file) => _file,
        Err(e) => panic!("failed to create temporary directory: {}", e),
    };
    let _repo = match Repository::clone(url, "./src/rate_repos/metrics/repos") {
        Ok(_repo) => _repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

pub fn calculate_correctness(url: &str) -> f32 {
    clone_repo(url);
    let gil = Python::acquire_gil();
    let py = gil.python();
    let result = run_python(py, url).unwrap();
    delete_repo();
    result
}

fn run_python(py: Python, url: &str) -> PyResult<f32> {
    let my_module = py.import("test")?;
    let my_function = my_module.get(py, "calculate")?;
    let result = my_function.call(py, (url,), None)?.extract(py)?;
    Ok(result)
}

fn delete_repo() {
    let _file = match fs::remove_dir_all("./src/rate_repos/metrics/repos") {
        Ok(_file) => _file,
        Err(e) => panic!("failed to remove temporary directory: {}", e),
    };
}