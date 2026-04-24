use algebraic_immunity_ext;
use pyo3::prelude::*;


#[pyclass]
pub struct AlgebraicImmunity;

#[pymethods]
impl AlgebraicImmunity {
    
    #[staticmethod]
    pub fn ai(truth_table: Vec<u8>, n: usize) -> usize{
        algebraic_immunity_ext::ai::AlgebraicImmunity::algebraic_immunity(truth_table, n)
    }

    #[staticmethod]
    pub fn restricted_ai(truth_table: Vec<u8>, restriction_domain: Vec<usize>, n: usize) -> usize{
        algebraic_immunity_ext::ai::RestrictedAlgebraicImmunity::algebraic_immunity(truth_table, restriction_domain ,n)
    }
}