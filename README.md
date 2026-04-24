# algebraic_immunity
[![Documentation](https://img.shields.io/badge/docs-online-blue)](https://lucabonamino.github.io/algebraic_immunity/)

Rust implementation of the algebraic immunity abd restricted algebraic immunity computation of Boolean functions, based on the papers  
*"Efficient Computation of Algebraic Immunity for Algebraic and Fast Algebraic Attacks"*  
(Armknecht et al., 2006, DOI: https://doi.org/10.1007/11761679_8) and *"Computing the Restricted Algebraic Immunity, and Application to WPB Functions"*  
Luca Bonamino and Pierrick Méaux. [IACR ePrint 2025/1779](https://eprint.iacr.org/2025/1779) .

This Rust implementation is wrapped as a Python package using PyO3 (https://pyo3.rs) and maturin (https://github.com/PyO3/maturin).

## Why This Implementation?

This library was developed to provide a robust and reliable implementation of algebraic immunity computation. It addresses a correctness issue in SageMath’s `BooleanFunction.algebraic_immunity()` method, which can raise internal errors or produce incorrect results for certain Boolean functions — particularly those with two variables, such as `[1, 0, 0, 1]`.

This Rust implementation has been tested extensively and is suitable for both small and large truth tables, with a focus on correctness and Python accessibility.

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
from algebraic_immunity import AlgebraicImmunity, BooleanFunction

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
 
