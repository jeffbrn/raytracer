use crate::data::tuple::Tuple;
use super::matrix::Matrix;

#[test]
fn init() {
    let m1 = Matrix::matrix2(1.0, 2.0, 3.0, 4.0);
    assert_eq!(m1.e(0,0), 1.0);
    assert_eq!(m1.e(0,1), 2.0);
    assert_eq!(m1.e(1,0), 3.0);
    assert_eq!(m1.e(1,1), 4.0);

    let m2 = Matrix::matrix3(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
    assert_eq!(m2.e(0,0), -3.0);
    assert_eq!(m2.e(0,1), 5.0);
    assert_eq!(m2.e(0,2), 0.0);
    assert_eq!(m2.e(1,0), 1.0);
    assert_eq!(m2.e(1,1), -2.0);
    assert_eq!(m2.e(1,2), -7.0);
    assert_eq!(m2.e(2,0), 0.0);
    assert_eq!(m2.e(2,1), 1.0);
    assert_eq!(m2.e(2,2), 1.0);

    let m3 = Matrix::matrix4(1.0, 2.0, 3.0, 4.0,
                             5.5, 6.5, 7.5, 8.5,
                             9.0, 10.0, 11.0, 12.0,
                             13.5, 14.5, 15.5, 16.5);
    assert_eq!(m3.e(0,0), 1.0);
    assert_eq!(m3.e(0,1), 2.0);
    assert_eq!(m3.e(0,2), 3.0);
    assert_eq!(m3.e(0,3), 4.0);
    assert_eq!(m3.e(1,0), 5.5);
    assert_eq!(m3.e(1,1), 6.5);
    assert_eq!(m3.e(1,2), 7.5);
    assert_eq!(m3.e(1,3), 8.5);
    assert_eq!(m3.e(2,0), 9.0);
    assert_eq!(m3.e(2,1), 10.0);
    assert_eq!(m3.e(2,2), 11.0);
    assert_eq!(m3.e(2,3), 12.0);
    assert_eq!(m3.e(3,0), 13.5);
    assert_eq!(m3.e(3,1), 14.5);
    assert_eq!(m3.e(3,2), 15.5);
    assert_eq!(m3.e(3,3), 16.5);
}

#[test]
fn test_eq() {
    let a = Matrix::matrix4(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
    let b = Matrix::matrix4(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
    assert_eq!(a, b);
    let c = Matrix::matrix4(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.5);
    assert_ne!(b, c);
}

#[test]
fn test_matrix_mul() {
    let a = Matrix::matrix4(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
    let b = Matrix::matrix4(-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0);
    let c = a * b;
    assert_eq!(c, Matrix::matrix4(20.0, 22.0, 50.0, 48.0,
                                  44.0, 54.0,114.0, 108.0,
                                  40.0, 58.0, 110.0, 102.0,
                                  16.0, 26.0,46.0, 42.0));

    let d = Matrix::matrix4(0.0, 1.0, 2.0, 4.0,
                            1.0, 2.0, 4.0, 8.0,
                            2.0, 4.0, 8.0, 16.0,
                            4.0, 8.0, 16.0, 32.0);
    assert_eq!(d*Matrix::identity(4), d);
}

#[test]
fn test_tuple_mul() {
    let a = Matrix::matrix4(1.0, 2.0, 3.0, 4.0,
                            2.0, 4.0, 4.0, 2.0,
                            8.0, 6.0, 4.0, 1.0,
                            0.0, 0.0, 0.0, 1.0);
    let b = Tuple::init(1.0, 2.0, 3.0, 1.0);
    let c = a * b;
    assert_eq!(c, Tuple::init(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn test_transpose() {
    let a = Matrix::matrix4(0.0, 9.0, 3.0, 0.0,
                            9.0, 8.0, 0.0, 8.0,
                            1.0, 8.0, 5.0, 3.0,
                            0.0, 0.0, 5.0, 8.0);
    let b = Matrix::matrix4(0.0, 9.0, 1.0, 0.0,
                            9.0, 8.0, 8.0, 0.0,
                            3.0, 0.0, 5.0, 5.0,
                            0.0, 8.0, 3.0, 8.0);
    assert_eq!(a.transpose4(), b);

    let c = Matrix::identity(4).transpose4();
    assert_eq!(c, Matrix::identity(4));
}