#![warn(clippy::all, clippy::pedantic)]

use pyo3::{
    PyResult, Python,
    ffi::c_str,
    types::{IntoPyDict, PyAnyMethods},
};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py)?;
        let code = c_str!("os.getenv('USER') or os.getenv('USERNAME')or 'Unknown'");
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {user}, from Python{version}");

        Ok(())
    })
}
