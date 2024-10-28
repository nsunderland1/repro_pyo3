#![deny(unsafe_op_in_unsafe_fn)]

use pyo3::prelude::*;

#[pyclass]
struct Foo;

#[pymethods]
impl Foo {
    fn do_nothing(&self) {}
}
