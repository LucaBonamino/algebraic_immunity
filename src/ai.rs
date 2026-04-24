use algebraic_immunity_ext;
use pyo3::prelude::*;

/// Class for computing algebraic immunity.
///
/// This class does not store any state. Use its static methods to compute
/// algebraic immunity directly from a truth table.
#[pyclass(name = "AlgebraicImmunity", module = "algebraic_immunity.algebraic_immunity")]
pub struct PyAlgebraicImmunity;

#[pymethods]
impl PyAlgebraicImmunity {
    
    /// Compute the algebraic immunity of a Boolean function.
    ///
    /// Args:
    ///     truth_table: Truth table of the Boolean function. Each entry must be
    ///         either 0 or 1.
    ///     n: Number of variables of the Boolean function.
    ///
    /// Returns:
    ///     The algebraic immunity of the Boolean function.
    ///
    /// Examples:
    ///     >>> from algebraic_immunity import AlgebraicImmunity
    ///     >>> AlgebraicImmunity.ai([0, 1, 1, 0], 2)
    #[staticmethod]
    pub fn ai(truth_table: Vec<u8>, n: usize) -> usize{
        algebraic_immunity_ext::ai::AlgebraicImmunity::algebraic_immunity(truth_table, n)
    }

    /// Compute the restricted algebraic immunity of a Boolean function.
    ///
    /// Args:
    ///     truth_table: Truth table of the Boolean function. Each entry must be
    ///         either 0 or 1.
    ///     restriction_domain: Subset of truth-table indices on which to compute
    ///         the restricted algebraic immunity.
    ///     n: Number of variables of the Boolean function.
    ///
    /// Returns:
    ///     The algebraic immunity of the restricted Boolean function.
    ///
    /// Examples:
    ///     >>> from algebraic_immunity import AlgebraicImmunity
    ///     >>> AlgebraicImmunity.restricted_ai([0, 1, 1, 0], [0, 1], 2)
    #[staticmethod]
    pub fn restricted_ai(truth_table: Vec<u8>, restriction_domain: Vec<usize>, n: usize) -> usize{
        algebraic_immunity_ext::ai::RestrictedAlgebraicImmunity::algebraic_immunity(truth_table, restriction_domain ,n)
    }
}