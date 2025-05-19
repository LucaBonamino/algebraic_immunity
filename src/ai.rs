use itertools::Itertools;
use rayon::prelude::*;
use crate::vandermonde::{VanderMonde, str_ops, verify}; 
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct AlgebraicImmunity {
    truth_table: Vec<u8>
}


#[pymethods]
impl AlgebraicImmunity {

    #[new]
    pub fn new(truth_table: Vec<u8>) -> Self {
        AlgebraicImmunity { truth_table }
    }

    fn compute_z(&self, n: usize) -> (Vec<String>, Vec<String>) {
        let mut true_idxs = Vec::new();
        let mut false_idxs = Vec::new();
     
        for i in 0..self.truth_table.len() {
           
            let bin_str = format!("{:0width$b}", i, width = n);
            if self.truth_table[i] == 1 {
                true_idxs.push(bin_str.clone());
            } else {
                false_idxs.push(bin_str.clone());
            }
            
        }

        (true_idxs, false_idxs)
    }

    #[staticmethod]
    pub fn algebraic_immunity(truth_table: Vec<u8>, n: usize) -> usize {
        let restricted_ai = Self::new(truth_table);
        let (z, z_c) = restricted_ai.compute_z(n);

        if z.is_empty() || z_c.is_empty() {
            return 0;
        }

        let r = (n+1) / 2;
        let e = Self::generate_combinations(n, r);

        let args = vec![
            (z.clone(), e.clone()),
            (z_c.clone(), e.clone()),
        ];

        let results: Vec<Option<usize>> = args
            .par_iter()
            .map(|(z, e)| {
                Self::find_min_annihilator(z.clone(), e.clone())
            })
            .collect();

        match results.into_iter().flatten().min() {
            Some(min_val) => min_val,
            None => 0,
        }
    }

}

impl AlgebraicImmunity {

    fn generate_combinations(n: usize, r: usize) -> Vec<String> {
        let mut all_combinations = Vec::new();

        for k in 0..=r {
            for ones_positions in (0..n).combinations(k) {
                let mut binary_string = vec!['0'; n];
                for &pos in &ones_positions {
                    binary_string[pos] = '1';
                }
                let combination: String = binary_string.iter().rev().collect(); // ✅ No reverse
                all_combinations.push(combination);
            }
        }

        all_combinations
    }

    pub fn find_min_annihilator(
        mut z: Vec<String>,
        e: Vec<String>
    ) -> Option<usize> {

        let max_number_of_monimials = e.len();
        if max_number_of_monimials == 0{
            return None;
        }

        let mut vander_monde = VanderMonde::new(vec![
            vec![str_ops(&z[0], &e[0])]
        ]);

        let mut idx = 0;
        let mut i = 1;
        let mut operations: Vec<(usize, usize)> = vec![];

        let n_iters = z.len();

        while i < n_iters {

            vander_monde = vander_monde.compute_next(e[..=i].to_vec(), z[..=i].to_vec(), i, operations.clone());
            let (new_matrix, operations_i) = vander_monde.row_echelon_form();
            vander_monde = new_matrix;

            if vander_monde.rank() < i + 1 {
                let kernel = vander_monde.kernel();
                let k = &kernel[0];

                let (vanish_on_z, vanish_index_opt) = verify(z[i + 1..].to_vec(), k.clone(), e[..=i].to_vec());
                if vanish_on_z {
                    return Some(e[i].chars().filter(|c| *c == '1').count());
                } else if let Some(vanish_index) = vanish_index_opt {
                    let new_index = i + vanish_index.0 + 1;
                    if new_index < z.len() {
                        z.swap(i + 1, new_index);
                    }
                }
            }

            i += 1;
            idx += 1;
            operations.extend(operations_i);
        }

        if vander_monde.rank() < i+1 {
            return Some(e[idx].chars().filter(|c| *c == '1').count());
        }

        if i < max_number_of_monimials{
            return Some(hamming_weight(&e[idx+1]));
            // return Some(e[idx+1].chars().filter(|c| *c == '1').count());
        }
        if let Some(last) = e.last() {
            return Some(last.chars().filter(|c| *c == '1').count());
        } else {
            return None;
        }

    }

    
}


fn hamming_weight(word: &str) -> usize{
    return word.chars().filter(|c| *c == '1').count();
}


