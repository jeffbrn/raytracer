use super::super::data::tuple::Tuple;
use super::Transform;

#[test]
fn translation() {
    let t = Transform::translation(Tuple::point(5.0, -3.0, 2.0));
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let result = t * p;
    assert_eq!(result, Tuple::point(2.0, 1.0, 7.0))
}

#[test]
fn transform_inverse() {
    let t = Transform::translation(Tuple::point(5.0, -3.0, 2.0));
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let result = t.inverse().expect("transform inverse is undefined") * p;
    assert_eq!(result, Tuple::point(-8.0, 7.0, 3.0));
}

#[test]
fn transform_vector() {
    let t = Transform::translation(Tuple::point(5.0, -3.0, 2.0));
    let v = Tuple::vector(-3.0, 4.0, 5.0);
    assert_eq!(t * v, v);
}

#[test]
fn scale_point() {
    let t = Transform::scaling(Tuple::point(2.0, 3.0, 4.0));
    let p = Tuple::point(-4.0, 6.0, 8.0);
    assert_eq!(t * p, Tuple::point(-8.0, 18.0, 32.0))
}

#[test]
fn scale_vector() {
    let t = Transform::scaling(Tuple::point(2.0, 3.0, 4.0));
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    assert_eq!(t * v, Tuple::vector(-8.0, 18.0, 32.0))
}

#[test]
fn scale_inverse() {
    let t = Transform::scaling(Tuple::point(2.0, 3.0, 4.0));
    let inv = t.inverse().expect("transform inverse is undefined");
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0))
}
