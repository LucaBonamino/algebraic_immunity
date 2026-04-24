use algebraic_immunity_ext;
use pyo3::prelude::*;

#[pyclass]
pub struct BooleanFunction {
    inner: algebraic_immunity_ext::boolean_function::BooleanFunction
}

#[pymethods]
impl BooleanFunction{

    #[new]
    pub fn new(truth_table: Vec<u8>) -> Self{
        let inner = algebraic_immunity_ext::boolean_function::BooleanFunction::from_truth_table(truth_table);
        Self {inner: inner}
    }

    pub fn algebraic_immunity(&self) -> usize {
        self.inner.algebraic_immunity()
    }

    pub fn restricted_algebraic_immunity(&self, restriction: Vec<usize>) -> usize {
        self.inner.restricted_algebraic_immunity(restriction)
    }

}