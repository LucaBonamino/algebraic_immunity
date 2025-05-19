#[derive(Clone)]
pub struct VanderMonde{
    pub elements: Vec<Vec<u8>>,
}

impl VanderMonde{

    pub fn new(elements: Vec<Vec<u8>>) -> Self{
        VanderMonde { elements }
    }

    pub fn __repr__(&self) -> String {
        let rows: Vec<String> = self
            .elements
            .iter()
            .map(|row| format!("{:?}", row))
            .collect();
        format!("[{}]", rows.join(", "))
    }

    fn nrows(&self) -> usize {
        self.elements.len()
    }

    fn ncols(&self) -> usize {
        if !self.elements.is_empty() {
            self.elements[0].len()
        } else {
            0
        }
    }


    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.elements.swap(row1, row2);
    }

    pub fn row_echelon_form(&self) -> (Self, Vec<(usize, usize)>) {
        let mut m_copy = self.clone();
        let rows = m_copy.nrows();
        let cols = m_copy.ncols();
        let mut operations: Vec<(usize, usize)> = Vec::new();
        let mut lead = 0;

        for r in 0..rows {
            if lead >= cols {
                break;
            }
            let mut i = r;
            while m_copy.elements[i][lead] == 0 {
                i += 1;
                if i == rows {
                    i = r;
                    lead += 1;
                    if lead == cols {
                        return (m_copy, operations);
                    }
                }
            }
            m_copy.swap_rows(r, i);
            if r != i {
                operations.push((r, i));
                operations.push((i, r));
                operations.push((r, i));
            }
            for i in 0..rows {
                if i != r && m_copy.elements[i][lead] == 1 {
                    for j in 0..cols {
                        m_copy.elements[i][j] = (m_copy.elements[i][j] + m_copy.elements[r][j]) % 2;
                    }
                    operations.push((i, r));
                }
            }
            lead += 1;
        }

        (m_copy, operations)
    }

    fn append_row(&mut self, v: Vec<u8>) {
        self.elements.push(v)
    }

    fn append_column(&mut self, v: Vec<u8>) {
        for i in 0..self.nrows() {
            self.elements[i].push(v[i]);
        }
    }

    pub fn rank(&self) -> usize {
        let mut count = 0;
        let mut pivot_columns = std::collections::HashSet::new();

        for i in 0..self.nrows() {
            let p = VanderMonde::get_pivot(&self.elements[i]);
            if let Some(col) = p {
                if pivot_columns.insert(col) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn kernel(&self) -> Vec<Vec<u8>> {
        let rows = self.nrows();
        let cols = self.ncols();

        let mut pivots = std::collections::HashMap::new();
        let mut kernel_base: Vec<Vec<u8>> = Vec::new();
        let mut free_columns: Vec<usize> = Vec::new();
        let mut row_index = 0;

        for j in 0..cols {
            if row_index < rows && self.elements[row_index][j] == 1 {
                pivots.insert(j, row_index);
                row_index += 1;
            } else {
                free_columns.push(j);
            }
        }

        for &free_col in &free_columns {
            let mut kernel_vector = vec![0; cols];
            kernel_vector[free_col] = 1;

            for (&p_index, &p_row) in &pivots {
                let mut sum = 0;

                for col in (0..cols).rev() {
                    if col != p_index {
                        sum = sum ^ (self.elements[p_row][col] * kernel_vector[col]);
                    }
                }

                kernel_vector[p_index] = sum;
            }

            kernel_base.push(kernel_vector);
        }

        kernel_base
    }

    fn get_pivot(row: &Vec<u8>) -> Option<usize> {
        row.iter().position(|&x| x == 1)
    }

    pub fn compute_next(
        &self,
        monom_slice: Vec<String>,
        support_slice: Vec<String>,
        idx: usize,
        operations: Vec<(usize, usize)>
    ) -> Self {
        let mut m_copy = self.clone();
        let row: Vec<u8> = (0..=idx)
            .map(|i| str_ops(&support_slice[support_slice.len() - 1], &monom_slice[i]) as u8)
            .collect();
        let column: Vec<u8> = (0..idx)
            .map(|i| str_ops(&support_slice[i], &monom_slice[monom_slice.len() - 1]) as u8)
            .collect();

        let n_vect: Vec<u8> = apply_operations(&operations, column);
        m_copy.append_column(n_vect);
        m_copy.append_row(row);

        m_copy
    }

    pub fn fill_rows(&self, support_slice: Vec<String>, monom_slice: Vec<String>) -> Self {
        let mut m_copy = self.clone();
        for j in 0..support_slice.len(){
            let row: Vec<u8> = (0..monom_slice.len())
                .map(|i| str_ops(&support_slice[j], &monom_slice[i]) as u8)
                .collect();
            m_copy.append_row(row)
        }

        m_copy
    }
}


pub fn str_ops(s1: &str, s2: &str) -> u8 {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| {
            let base = c1.to_digit(10).unwrap() as u8;
            let exp = c2.to_digit(10).unwrap() as u8;
            base.pow(exp as u32)
        })
        .product()
}

fn apply_operations(operations: &Vec<(usize, usize)>, v: Vec<u8>) -> Vec<u8> {
    let mut result = v.clone();
    for &(op1, op2) in operations.iter() {
        result[op1] = (result[op1] + result[op2]) % 2;
    }
    result
}


fn is_submonomial(sub_monom: &str, monom: &str) -> bool {
    assert_eq!(sub_monom.len(), monom.len(), "The lengths of sub_monom and monom must be equal");

    for (char1, char2) in sub_monom.chars().zip(monom.chars()) {
        if char1 > char2 {
            return false;
        }
    }
    true
}

pub fn verify(z: Vec<String>, g: Vec<u8>, mapping: Vec<String>) -> (bool, Option<(usize, String)>) {
    for (idx, item) in z.iter().enumerate() {
        let anf: Vec<u8> = (0..g.len())
            .filter(|&i| is_submonomial(&mapping[i], item))
            .map(|i| g[i])
            .collect();

        if anf.iter().copied().sum::<u8>() % 2 == 1 {
            return (false, Some((idx, item.clone())));
        }
    }
    (true, None)
}


