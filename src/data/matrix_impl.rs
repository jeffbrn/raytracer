use super::matrix::Matrix;

use std::cmp::{PartialEq, Eq};

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.side() != other.side() {
            return false;
        }
        for i in 0..self.len() {
            if self.item(i) != other.item(i) {
                return false;
            }
        }
        true
    }
}

impl Eq for Matrix {}

use std::ops::{Mul};

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut vals = [0.0; 16];
        let mut i: usize = 0;
        for row in 0..self.side() {
            for col in 0..self.side() {
                vals[i] = self.row_mult(row, &rhs, col);
                i += 1;
            }
        };
        let r = Matrix::matrix(self.side(), &vals);
        r
    }
}
