mod ai;
mod boolean_function;
use pyo3::prelude::*;


#[pymodule]
fn algebraic_immunity(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ai::PyAlgebraicImmunity>()?;
    m.add_class::<boolean_function::PyBooleanFunction>()?;
    Ok(())
}
