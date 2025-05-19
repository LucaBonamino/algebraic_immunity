mod vandermonde;
mod ai;
use pyo3::prelude::*;


#[pymodule]
fn algebraic_immunity(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ai::AlgebraicImmunity>()?;
    Ok(())
}
