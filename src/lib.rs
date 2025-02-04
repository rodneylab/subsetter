#![warn(clippy::all, clippy::pedantic)]

mod domain;

use pyo3::{
    exceptions::PyValueError,
    pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyErr, PyResult,
};

use domain::{font_face, hash, unicodes, FontFaceParameters};

#[pyfunction]
#[pyo3(name = "font_face")]
#[expect(clippy::needless_pass_by_value)]
fn font_face_py(
    font_family: &str,
    font_weight: u32,
    text: &str,
    font_file_paths: Vec<String>,
) -> Result<String, PyErr> {
    let Ok(parameters) = FontFaceParameters::new(
        font_family,
        font_weight,
        text,
        &font_file_paths,
        Some("/fonts/"),
    ) else {
        return Err(PyValueError::new_err("invalid font face parameters"));
    };
    Ok(font_face(&parameters))
}

#[pyfunction]
#[pyo3(name = "hash")]
fn hash_py(text: &str) -> String {
    hash(text)
}

#[pyfunction]
#[pyo3(name = "unicodes")]
fn unicodes_py(text: &str) -> String {
    unicodes(text)
}

/// A Python module implemented in Rust.
#[pymodule]
fn subsetter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(font_face_py, m)?)?;
    m.add_function(wrap_pyfunction!(hash_py, m)?)?;
    m.add_function(wrap_pyfunction!(unicodes_py, m)?)?;
    Ok(())
}
