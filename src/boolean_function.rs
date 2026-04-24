use algebraic_immunity_ext;
use pyo3::prelude::*;

/// Boolean funcrion object.
#[pyclass(name = "BooleanFunction", module = "algebraic_immunity.algebraic_immunity")]
pub struct PyBooleanFunction {
    inner: algebraic_immunity_ext::boolean_function::BooleanFunction
}

#[pymethods]
impl PyBooleanFunction{

    /// Create a Boolean function from a truth table.
    ///
    /// Args:
    ///     truth_table: Truth table of the Boolean function. Each entry must be
    ///         either 0 or 1.
    ///
    /// Returns:
    ///     A new Boolean function instance.
    /// 
    /// Examples:
    ///     >>> from algebraic_immunity import BooleanFunction
    ///     >>> bf = BooleanFunction([0, 1, 1, 0])
    #[new]
    pub fn new(truth_table: Vec<u8>) -> Self{
        let inner = algebraic_immunity_ext::boolean_function::BooleanFunction::from_truth_table(truth_table);
        Self {inner: inner}
    }

    /// Compute the algebraic immunity of the Boolean function.
    ///
    /// Returns:
    ///     The algebraic immunity of the Boolean function.
    /// 
    /// Examples:
    ///     >>> from algebraic_immunity import BooleanFunction
    ///     >>> bf = BooleanFunction([0, 1, 1, 0])
    ///     >>> bf.algebraic_immunity()
    pub fn algebraic_immunity(&self) -> usize {
        self.inner.algebraic_immunity()
    }

    /// Compute the restricted algebraic immunity of the Boolean function.
    ///
    /// Args:
    ///     restriction_domain: Subset of input indices on which to restrict the
    ///         Boolean function.
    ///
    /// Returns:
    ///     The algebraic immunity of the restricted Boolean function.
    /// 
    /// Examples:
    ///     >>> from algebraic_immunity import BooleanFunction
    ///     >>> bf = BooleanFunction([0, 1, 1, 0])
    ///     >>> bf.restricted_algebraic_immunity([0,2])
    pub fn restricted_algebraic_immunity(&self, restriction_domain: Vec<usize>) -> usize {
        self.inner.restricted_algebraic_immunity(restriction_domain)
    }

}