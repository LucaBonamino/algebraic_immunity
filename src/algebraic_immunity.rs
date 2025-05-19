use itertools::Itertools;
use rayon::prelude::*;
use crate::vandermonde::{VanderMonde, str_ops, verify}; 
use std::collections::HashSet;

#[derive(Clone)]
pub struct AlgebraicImmunity {
    truth_table: Vec<u8>
}


impl AlgebraicImmunity {

    pub fn new(truth_table: Vec<u8>) -> Self {
        AlgebraicImmunity { truth_table }
    }

    fn compute_z(&self, subset: Vec<usize>, n: usize) -> (Vec<String>, Vec<String>, Vec<String>) {
        let mut true_idxs = Vec::new();
        let mut false_idxs = Vec::new();
        let mut s_bin = Vec::new();
        let s: HashSet<_> = subset.into_iter().collect();

        for i in 0..self.truth_table.len() {
            if !s.contains(&i) {
                continue;
            }

            let bin_str = format!("{:0width$b}", i, width = n);
            if self.truth_table[i] == 1 {
                true_idxs.push(bin_str.clone());
            } else {
                false_idxs.push(bin_str.clone());
            }
            s_bin.push(bin_str);
        }

        (true_idxs, false_idxs, s_bin)
    }

    pub fn algebraic_immunity(truth_table: Vec<u8>, subset: Vec<usize>, n: usize) -> usize {
        let restricted_ai = Self::new(truth_table);
        let (z, z_c, s_bin) = restricted_ai.compute_z(subset, n);

        if z.is_empty() || z_c.is_empty() {
            return 0;
        }

        let e = Self::generate_combinations(n, n);

        let args = vec![
            (z.clone(), z_c.clone(), e.clone(), s_bin.clone()),
            (z_c.clone(), z.clone(), e.clone(), s_bin.clone()),
        ];

        let results: Vec<Option<usize>> = args
            .par_iter()
            .map(|(z, z_c, e, s_bin)| {
                Self::find_min_annihilator(z.clone(), z_c.clone(), e.clone(), s_bin.clone())
            })
            .collect();

        match results.into_iter().flatten().min() {
            Some(min_val) => min_val,
            None => 0,
        }
    }

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
        z_c: Vec<String>,
        mut e: Vec<String>,
        s: Vec<String>,
    ) -> Option<usize> {
        let mut vander_monde = VanderMonde::new(vec![
            vec![str_ops(&z[0], &e[0])]
        ]);

        let mut idx = 0;
        let mut i = 1;
        let mut operations: Vec<(usize, usize)> = vec![];

        let n_iters = z.len();

        while i < n_iters {
            let vander_monde_old = vander_monde.clone();

            vander_monde = vander_monde.compute_next(e[..=i].to_vec(), z[..=i].to_vec(), i, operations.clone());
            let (new_matrix, operations_i) = vander_monde.row_echelon_form();
            vander_monde = new_matrix;

            if vander_monde.rank() < i + 1 {
                let kernel = vander_monde.kernel();
                let k = &kernel[0];

                let (vanish_on_z, vanish_index_opt) = verify(z[i + 1..].to_vec(), k.clone(), e[..=i].to_vec());
                if vanish_on_z {
                    let (vanish_on_s, _) = verify(z_c.clone(), k.clone(), e[..=i].to_vec());
                    if !vanish_on_s {
                        return Some(e[i].chars().filter(|c| *c == '1').count());
                    } else {
                        vander_monde = vander_monde_old;
                        e.remove(i);
                        continue;
                    }
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

        let mut vander_monde_s = VanderMonde::new(VanderMonde::compute_vandermonde(s[..=idx].to_vec(), e[..=idx].to_vec()));
        vander_monde_s = vander_monde_s.fill_rows(s[idx + 1..].to_vec(), e[..=idx].to_vec());

        let (vander_monde_s_reduced, mut operations_s) = vander_monde_s.row_echelon_form();
        let mut r_s = vander_monde_s_reduced.rank();

        if vander_monde.rank() < r_s {
            return Some(e[idx].chars().filter(|c| *c == '1').count());
        }

        i = idx + 1;
        let s_len = s.len();
        let mut vander_monde_s = vander_monde_s_reduced;

        while r_s <= (s_len + 1) / 2 {
            if i >= e.len() {
                break;
            }

            vander_monde = vander_monde.construct_and_add_column(
                z.clone(),
                e[i].clone(),
                operations.clone()
            );

            vander_monde_s = vander_monde_s.construct_and_add_column(
                s.clone(),
                e[i].clone(),
                operations_s.clone()
            );

            let (vander_monde_s_new, ops_s) = vander_monde_s.row_echelon_form();
            vander_monde_s = vander_monde_s_new;

            r_s = vander_monde_s.rank();

            if vander_monde.rank() < r_s {
                return Some(e[i].chars().filter(|c| *c == '1').count());
            }

            i += 1;
            operations_s.extend(ops_s);
        }


        None
    }


}


