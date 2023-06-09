use super::matrix::Matrix;
use crate::data::tuple::Tuple;
use approx::assert_relative_eq;

impl Matrix {
    fn assert_approx_eq(&self, other: &Matrix) {
        assert_eq!(self.side(), other.side());
        for row in 0..self.side() {
            for col in 0..self.side() {
                assert_relative_eq!(self.e(row, col), other.e(row, col), epsilon = 1e-4);
            }
        }
    }
}

#[test]
fn init() {
    let m1 = Matrix::matrix2(1.0, 2.0, 3.0, 4.0);
    assert_eq!(m1.e(0, 0), 1.0);
    assert_eq!(m1.e(0, 1), 2.0);
    assert_eq!(m1.e(1, 0), 3.0);
    assert_eq!(m1.e(1, 1), 4.0);

    let m2 = Matrix::matrix3(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
    assert_eq!(m2.e(0, 0), -3.0);
    assert_eq!(m2.e(0, 1), 5.0);
    assert_eq!(m2.e(0, 2), 0.0);
    assert_eq!(m2.e(1, 0), 1.0);
    assert_eq!(m2.e(1, 1), -2.0);
    assert_eq!(m2.e(1, 2), -7.0);
    assert_eq!(m2.e(2, 0), 0.0);
    assert_eq!(m2.e(2, 1), 1.0);
    assert_eq!(m2.e(2, 2), 1.0);

    let m3 = Matrix::matrix4(
        1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
    );
    assert_eq!(m3.e(0, 0), 1.0);
    assert_eq!(m3.e(0, 1), 2.0);
    assert_eq!(m3.e(0, 2), 3.0);
    assert_eq!(m3.e(0, 3), 4.0);
    assert_eq!(m3.e(1, 0), 5.5);
    assert_eq!(m3.e(1, 1), 6.5);
    assert_eq!(m3.e(1, 2), 7.5);
    assert_eq!(m3.e(1, 3), 8.5);
    assert_eq!(m3.e(2, 0), 9.0);
    assert_eq!(m3.e(2, 1), 10.0);
    assert_eq!(m3.e(2, 2), 11.0);
    assert_eq!(m3.e(2, 3), 12.0);
    assert_eq!(m3.e(3, 0), 13.5);
    assert_eq!(m3.e(3, 1), 14.5);
    assert_eq!(m3.e(3, 2), 15.5);
    assert_eq!(m3.e(3, 3), 16.5);
}

#[test]
fn eq() {
    let a = Matrix::matrix4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
    );
    let b = Matrix::matrix4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
    );
    assert_eq!(a, b);
    let c = Matrix::matrix4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.5,
    );
    assert_ne!(b, c);
}

#[test]
fn matrix_mul() {
    let a = Matrix::matrix4(
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
    );
    let b = Matrix::matrix4(
        -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
    );
    let c = a * b;
    assert_eq!(
        c,
        Matrix::matrix4(
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0
        )
    );

    let d = Matrix::matrix4(
        0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
    );
    assert_eq!(d * Matrix::identity(4), d);
}

#[test]
fn tuple_mul() {
    let a = Matrix::matrix4(
        1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
    );
    let b = Tuple::init(1.0, 2.0, 3.0, 1.0);
    let c = a * b;
    assert_eq!(c, Tuple::init(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn transpose() {
    let a = Matrix::matrix4(
        0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
    );
    let b = Matrix::matrix4(
        0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
    );
    assert_eq!(a.transpose4(), b);

    let c = Matrix::identity(4).transpose4();
    assert_eq!(c, Matrix::identity(4));
}

#[test]
fn submatrix() {
    let a1 = Matrix::matrix3(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0);
    let r1 = a1.submatrix(0, 2);
    assert_eq!(r1, Matrix::matrix2(-3.0, 2.0, 0.0, 6.0));
    let a2 = Matrix::matrix4(
        -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
    );
    let r2 = a2.submatrix(2, 1);
    assert_eq!(
        r2,
        Matrix::matrix3(-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0)
    );
}

#[test]
fn minor() {
    let a = Matrix::matrix3(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
    assert_eq!(a.minor(1, 0), 25.0);
}

#[test]
fn cofactor() {
    let a = Matrix::matrix3(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
    assert_eq!(a.minor(0, 0), -12.0);
    assert_eq!(a.cofactor(0, 0), -12.0);
    assert_eq!(a.minor(1, 0), 25.0);
    assert_eq!(a.cofactor(1, 0), -25.0);
}

#[test]
fn determinant() {
    let a1 = Matrix::matrix3(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);
    assert_eq!(a1.cofactor(0, 0), 56.0);
    assert_eq!(a1.cofactor(0, 1), 12.0);
    assert_eq!(a1.cofactor(0, 2), -46.0);
    assert_eq!(a1.determinant(), -196.0);

    let a2 = Matrix::matrix4(
        -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
    );
    assert_eq!(a2.cofactor(0, 0), 690.0);
    assert_eq!(a2.cofactor(0, 1), 447.0);
    assert_eq!(a2.cofactor(0, 2), 210.0);
    assert_eq!(a2.cofactor(0, 3), 51.0);
    assert_eq!(a2.determinant(), -4071.0);
}

#[test]
fn inverse() {
    let a1 = Matrix::matrix4(
        -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
    );
    let r1 = a1.inverse();
    assert!(r1.is_none());

    let a2 = Matrix::matrix4(
        -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
    );
    let r2 = a2.inverse().expect("Unable to find inverse of a2");
    let e2 = Matrix::matrix4(
        0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895,
        -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
    );
    r2.assert_approx_eq(&e2);

    let a3 = Matrix::matrix4(
        8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
    );
    let r3 = a3.inverse().expect("a3 inverse is undefined");
    let e3 = Matrix::matrix4(
        -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
        0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
    );
    r3.assert_approx_eq(&e3);

    let a4 = Matrix::matrix4(
        9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0,
    );
    let r4 = a4.inverse().expect("a4 inverse is undefined");
    let e4 = Matrix::matrix4(
        -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
        -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
    );
    r4.assert_approx_eq(&e4);
}

#[test]
fn multiply_inverse() {
    let a = Matrix::matrix4(
        3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0,
    );
    let b = Matrix::matrix4(
        8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
    );
    let c = a * b;
    let d = c * b.inverse().expect("b inverse is undefined");
    d.assert_approx_eq(&a);
}
