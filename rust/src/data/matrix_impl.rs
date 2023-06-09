use super::matrix::Matrix;
use super::tuple::Tuple;

impl Matrix {
    fn row_tuple(&self, row: i8) -> Tuple {
        assert_eq!(self.side(), 4);
        Tuple::init(
            self.e(row, 0),
            self.e(row, 1),
            self.e(row, 2),
            self.e(row, 3),
        )
    }
    fn col_tuple(&self, col: i8) -> Tuple {
        assert_eq!(self.side(), 4);
        Tuple::init(
            self.e(0, col),
            self.e(1, col),
            self.e(2, col),
            self.e(3, col),
        )
    }

    fn matrix4_rows(row1: &Tuple, row2: &Tuple, row3: &Tuple, row4: &Tuple) -> Matrix {
        Matrix::matrix4(
            row1.x(),
            row1.y(),
            row1.z(),
            row1.w(),
            row2.x(),
            row2.y(),
            row2.z(),
            row2.w(),
            row3.x(),
            row3.y(),
            row3.z(),
            row3.w(),
            row4.x(),
            row4.y(),
            row4.z(),
            row4.w(),
        )
    }

    pub fn transpose4(&self) -> Matrix {
        assert_eq!(self.side(), 4);
        let row1 = self.col_tuple(0);
        let row2 = self.col_tuple(1);
        let row3 = self.col_tuple(2);
        let row4 = self.col_tuple(3);
        Matrix::matrix4_rows(&row1, &row2, &row3, &row4)
    }

    pub fn determinant(&self) -> f32 {
        match self.side() {
            2 => self.e(0, 0) * self.e(1, 1) - self.e(0, 1) * self.e(1, 0),
            _ => {
                let mut sum = 0.0;
                for col in 0..self.side() {
                    sum += self.e(0, col) * self.cofactor(0, col);
                }
                sum
            }
        }
    }
    pub fn submatrix(&self, row: i8, col: i8) -> Matrix {
        let mut vals: [f32; 9] = [0.0; 9];
        let mut i: usize = 0;
        for r in 0..self.side() {
            if r == row {
                continue;
            }
            for c in 0..self.side() {
                if c == col {
                    continue;
                }
                vals[i] = self.e(r, c);
                i += 1;
            }
        }
        Matrix::new(self.side() - 1, &vals)
    }
    pub fn minor(&self, row: i8, col: i8) -> f32 {
        let sub = self.submatrix(row, col);
        sub.determinant()
    }
    pub fn cofactor(&self, row: i8, col: i8) -> f32 {
        let sub = self.submatrix(row, col);
        let minor = sub.determinant();
        match (row + col) % 2 {
            0 => minor,
            _ => -minor,
        }
    }
    pub fn inverse(&self) -> Option<Matrix> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }
        let mut vals = [0.0; 16];
        for row in 0..self.side() {
            for col in 0..self.side() {
                let c = self.cofactor(row, col);
                vals[self.get_array_idx(col, row)] = c / det;
            }
        }
        Some(Matrix::new(self.side(), &vals))
    }
    fn get_array_idx(&self, row: i8, col: i8) -> usize {
        (row * self.side() + col) as usize
    }
}

use std::cmp::{Eq, PartialEq};

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.side() != other.side() {
            return false;
        }
        let len = self.side() * self.side();
        for i in 0..len {
            if self.item(i) != other.item(i) {
                return false;
            }
        }
        true
    }
}

impl Eq for Matrix {}

use std::ops::Mul;

impl Mul for Matrix {
    type Output = Matrix;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut vals = [0.0; 16];
        let mut i: usize = 0;
        for row in 0..self.side() {
            for col in 0..self.side() {
                let other = rhs.col_tuple(col);
                vals[i] = self.row_tuple(row).dot(&other);
                i += 1;
            }
        }
        Matrix::new(self.side(), &vals)
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut vals = [0.0; 4];
        for row in 0..self.side() {
            vals[row as usize] = self.row_tuple(row).dot(&rhs);
        }
        Tuple::init(vals[0], vals[1], vals[2], vals[3])
    }
}
