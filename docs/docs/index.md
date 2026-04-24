# algebraic_immunity

Rust implementation of the algebraic immunity and restricted algebraic immunita computation of Boolean functions.

This Rust implementation is wrapped as a Python package using PyO3 (https://pyo3.rs) and maturin (https://github.com/PyO3/maturin).

---

## Installation

You can install the package via **PyPI** or use the **pre-built wheels** provided in the GitHub releases.

### ✅ Installation via PyPI

To install the package directly from PyPI:

```bash
pip install algebraic_immunity
```

### Installation via Pre-built Wheels
If PyPI installation doesn't work for your platform, you can use the pre-built wheels:
1. Run the following script to determine the correct wheel for your platform:
```bash
python construct_wheel_url.py
```
2. Then install the wheel using:
```bash
pip install <output of the previous command>
```
---

## Usage Example (Python)
```python
# Algebraic immunity
truth_table = [0, 1, 1, 1, 1, 0, 0, 1]
n = 3
ai = AlgebraicImmunity.ai(truth_table, n)
print(f"Algebraic immunity: {ai}")

# Restricted Algebraic immunity
bf = BooleanFunction(truth_table)
ai = bf.algebraic_immunity()
print(f"Algebraic immunity: {ai}")
```

---

## Future Work

- Parts of this repository are more broadly relevant to operations over GF(2) matrices. These components could be extracted into a standalone Rust crate in the future.
- The current implementation sometimes uses the `clone` method unnecessarily to bypass Rust’s borrowing rules. These instances will be reviewed and optimized in future updates.
 
