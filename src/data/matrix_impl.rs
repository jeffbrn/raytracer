use super::matrix::Matrix;
use super::tuple::Tuple;

impl Matrix {
    fn row_tuple(&self, row: i8) -> Tuple {
        assert_eq!(self.side(), 4);
        Tuple::init(self.e(row, 0), self.e(row, 1), self.e(row, 2), self.e(row, 3))
    }
    fn col_tuple(&self, col:i8) -> Tuple {
        assert_eq!(self.side(), 4);
        Tuple::init(self.e(0, col), self.e(1, col), self.e(2, col), self.e(3, col))
    }

    fn matrix4_rows(row1:&Tuple, row2:&Tuple, row3:&Tuple, row4:&Tuple) -> Matrix {
        Matrix::matrix4(row1.x(), row1.y(), row1.z(), row1.w(),
                        row2.x(), row2.y(), row2.z(), row2.w(),
                        row3.x(), row3.y(), row3.z(), row3.w(),
                        row4.x(), row4.y(), row4.z(), row4.w())
    }

    pub fn transpose4(&self) -> Matrix {
        assert_eq!(self.side(), 4);
        let row1 = self.col_tuple(0);
        let row2 = self.col_tuple(1);
        let row3 = self.col_tuple(2);
        let row4 = self.col_tuple(3);
        Matrix::matrix4_rows(&row1, &row2, &row3, &row4)
    }
}

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
                let other = rhs.col_tuple(col);
                vals[i] = self.row_tuple(row).dot(&other);
                i += 1;
            }
        };
        let r = Matrix::matrix(self.side(), &vals);
        r
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut vals = [0.0; 4];
        for row in 0..self.side() {
            vals[row as usize] = self.row_tuple(row).dot(&rhs);
        };
        Tuple::init(vals[0], vals[1], vals[2], vals[3])
    }
}
