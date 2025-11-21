use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut, Mul},
};

use approx::AbsDiffEq;

use crate::engine::maths::tuple::Tuple;

#[derive(PartialEq, Debug, Clone)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Matrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        Self(matrix)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self(vec![
            vec![1.0, 0.0, 0.0, x],
            vec![0.0, 1.0, 0.0, y],
            vec![0.0, 0.0, 1.0, z],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self(vec![
            vec![x, 0.0, 0.0, 0.0],
            vec![0.0, y, 0.0, 0.0],
            vec![0.0, 0.0, z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_x(rad: f64) -> Self {
        Self(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, f64::cos(rad), -f64::sin(rad), 0.0],
            vec![0.0, f64::sin(rad), f64::cos(rad), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y(rad: f64) -> Self {
        Self(vec![
            vec![f64::cos(rad), 0.0, f64::sin(rad), 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-f64::sin(rad), 0.0, f64::cos(rad), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z(rad: f64) -> Self {
        Self(vec![
            vec![f64::cos(rad), -f64::sin(rad), 0.0, 0.0],
            vec![f64::sin(rad), f64::cos(rad), 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self(vec![
            vec![1.0, xy, xz, 0.0],
            vec![yx, 1.0, yz, 0.0],
            vec![zx, zy, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn identity() -> Self {
        Self(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transpose(&mut self) {
        let mut rows = vec![];
        for i in 0..self.len() {
            let mut row = vec![];
            for j in 0..self.len() {
                row.push(self[j][i]);
            }
            rows.push(row);
        }
        self.0 = rows;
    }

    pub fn determinant(&self) -> f64 {
        if self.len() == 2 {
            return (self[0][0] * self[1][1]) - (self[1][0] * self[0][1]);
        }
        let mut det = 0.0;
        for col in 0..self.len() {
            det += self[0][col] * self.cofactor(0, col);
        }
        det
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut matrix = vec![vec![0.0; self.len() - 1]; self.len() - 1];
        for i in 0..(self.len() - 1) {
            let mut target_row = i;
            if i >= row {
                target_row = i + 1
            }
            for j in 0..(self.len() - 1) {
                let mut target_col = j;
                if j >= col {
                    target_col = j + 1;
                }
                matrix[i][j] = self[target_row][target_col];
            }
        }
        Matrix(matrix)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) as f32 % 2f32 != 0f32 {
            return -minor;
        }
        minor
    }

    pub fn inverse(&self) -> Result<Self, String> {
        // let start = Instant::now();
        let mut inv = Matrix::identity();
        let mut temp_matrix: Matrix = self.clone();
        if self._is_pivot_zero() {
            temp_matrix._arrange_non_zero_row_pivots(&mut inv)?;
        }
        // TODO - make bespoke method for adding row in inverse operation
        // because this can be more efficient than the normal reo_add method,
        // as we know we want every non-pivot element in the column to become 0
        // so don't need to perform calculation, and can just set it to 0.
        for i in 0..self.len() {
            let pivot = temp_matrix[i][i];
            let pivot_inv = 1.0 / pivot;

            for row in 0..self.len() {
                if temp_matrix[row][i] == 0.0 || row == i {
                    continue;
                }

                let k = -temp_matrix[row][i] * pivot_inv;
                temp_matrix.reo_add(row, i, k);
                inv.reo_add(row, i, k);
            }

            temp_matrix.reo_multiply_by_k(i, pivot_inv);
            inv.reo_multiply_by_k(i, pivot_inv);
        }
        // let end = Instant::now();
        // println!("DURATION {:?}", end - start);
        Ok(inv)
    }

    fn _arrange_non_zero_row_pivots(&mut self, inv: &mut Matrix) -> Result<(), String> {
        // The list of indices for the pivot in each row, these start as 4,
        // as this is out of range for the expected values (0..4), so if these values are
        // unchanged, no valid pivot is found, and the matrix is most likely singular,
        // so we can error out
        let mut row_pivots = vec![self.len(); self.len()];

        // create list of the index of non zero elements for each row
        let mut non_zero_row_els: Vec<VecDeque<usize>> = vec![
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
            VecDeque::new(),
        ];

        for row in 0..self.len() {
            for column in 0..self.len() {
                if self[row][column] != 0.0 {
                    non_zero_row_els[row].push_back(column);
                }
            }
            if non_zero_row_els[row].len() == 0 {
                return Err(format!(
                    "Matrix is not invertible, it is singular. Determinant of a matrix should be non-zero if it is invertible. Matrix: {:?}",
                    self
                ));
            }
        }

        // select the index of the element that should be the pivot for each row and assign it
        // to the correct index in row_pivots
        for column in 0..self.len() {
            let mut non_zero_rows: Vec<usize> = vec![];
            for row in 0..self.len() {
                if row_pivots[row] != self.len() {
                    continue;
                }
                if self[row][column] != 0.0 {
                    non_zero_rows.push(row);
                }
            }
            if non_zero_rows.len() == 0 {
                return Err(format!(
                    "Matrix is not invertible, it is singular. Determinant of a matrix should be non-zero if it is invertible. Matrix: {:?}",
                    self
                ));
            }
            let shortest_non_zero_row = non_zero_rows
                .iter()
                .reduce(|shortest_row, row| {
                    if non_zero_row_els[*row].len() < non_zero_row_els[*shortest_row].len() {
                        non_zero_row_els[*shortest_row].pop_front();
                        return row;
                    }
                    non_zero_row_els[*row].pop_front();
                    shortest_row
                })
                .unwrap();
            if &column > shortest_non_zero_row {
                let target_index = row_pivots[(shortest_non_zero_row + 1)..]
                    .iter()
                    .position(|el| el == shortest_non_zero_row)
                    .unwrap()
                    + shortest_non_zero_row
                    + 1;

                row_pivots[*shortest_non_zero_row] = row_pivots[target_index];
                row_pivots[target_index] = non_zero_row_els[*shortest_non_zero_row][0];

                self.reo_swap(target_index, *shortest_non_zero_row);
                inv.reo_swap(target_index, *shortest_non_zero_row);
            } else {
                row_pivots[*shortest_non_zero_row] = non_zero_row_els[*shortest_non_zero_row][0];
                non_zero_row_els[*shortest_non_zero_row].pop_front();
            }
        }

        if row_pivots.contains(&self.len()) {
            return Err(format!(
                "Matrix is not invertible, it is singular. Determinant of a matrix should be non-zero if it is invertible. Matrix: {:?}",
                self
            ));
        }

        Ok(())
    }

    fn _is_pivot_zero(&self) -> bool {
        for i in 0..self.len() {
            if self[i][i] == 0.0 {
                return true;
            }
        }
        false
    }

    fn reo_add(&mut self, lhs: usize, rhs: usize, k: f64) {
        for column in 0..self.len() {
            self[lhs][column] += self[rhs][column] * k;
        }
    }

    fn reo_swap(&mut self, row_one: usize, row_two: usize) {
        self.swap(row_one, row_two);
    }

    fn reo_multiply_by_k(&mut self, row: usize, k: f64) {
        for column in 0..self.len() {
            self[row][column] *= k;
        }
    }
}

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result_matrix = vec![];

        for i in 0..self.0.len() {
            let mut row = vec![];
            for j in 0..self.0.len() {
                let cell_value = self[i][0] * rhs[0][j]
                    + self[i][1] * rhs[1][j]
                    + self[i][2] * rhs[2][j]
                    + self[i][3] * rhs[3][j];
                row.push(cell_value);
            }
            result_matrix.push(row);
        }

        Matrix(result_matrix)
    }
}
impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut result_tuple = vec![];

        for i in 0..self.0.len() {
            result_tuple.push(
                self[i][0] * rhs[0]
                    + self[i][1] * rhs[1]
                    + self[i][2] * rhs[2]
                    + self[i][3] * rhs[3],
            );
        }

        Tuple(result_tuple)
    }
}

impl Deref for Matrix {
    type Target = Vec<Vec<f64>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AbsDiffEq for Matrix {
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        1e-5
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        for i in 0..self.len() {
            for j in 0..self.len() {
                if !f64::abs_diff_eq(&self[i][j], &other[i][j], epsilon) {
                    return false;
                }
            }
        }
        true
    }
}
