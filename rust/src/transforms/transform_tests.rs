use super::super::data::tuple::Tuple;
use super::Transform;
use approx::assert_relative_eq;

impl Tuple {
    fn assert_approx_eq(&self, other: &Tuple) {
        assert_relative_eq!(self.x(), other.x(), epsilon = 1e-4);
        assert_relative_eq!(self.y(), other.y(), epsilon = 1e-4);
        assert_relative_eq!(self.z(), other.z(), epsilon = 1e-4);
        assert_relative_eq!(self.w(), other.w(), epsilon = 1e-4);
    }
}

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

#[test]
fn scale_reflection() {
    let t = Transform::scaling(Tuple::point(-1.0, 1.0, 1.0));
    let p = Tuple::point(2.0, 3.0, 4.0);
    assert_eq!(t * p, Tuple::point(-2.0, 3.0, 4.0));
}

#[test]
fn x_axis_rotation() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let pi = std::f32::consts::PI;
    let root2 = 2.0_f32.sqrt();
    let half_qtr = Transform::rotation_x(pi / 4.0);
    assert_eq!(half_qtr * p, Tuple::point(0.0, root2 / 2.0, root2 / 2.0));
    let full_qtr = Transform::rotation_x(pi / 2.0);
    let result = full_qtr * p;
    let expected = Tuple::point(0.0, 0.0, 1.0);
    result.assert_approx_eq(&expected);
    let inv = half_qtr.inverse().expect("transform inverse is undefined");
    let expected2 = Tuple::point(0.0, root2 / 2.0, -root2 / 2.0);
    (inv * p).assert_approx_eq(&expected2);
}

#[test]
fn y_axis_rotation() {
    let p = Tuple::point(0.0, 0.0, 1.0);
    let pi = std::f32::consts::PI;
    let root2 = 2.0_f32.sqrt();
    let half_qtr = Transform::rotation_y(pi / 4.0);
    assert_eq!(half_qtr * p, Tuple::point(root2 / 2.0, 0.0, root2 / 2.0));
    let full_qtr = Transform::rotation_y(pi / 2.0);
    let result = full_qtr * p;
    let expected = Tuple::point(1.0, 0.0, 0.0);
    result.assert_approx_eq(&expected);
}

#[test]
fn z_axis_rotation() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    let pi = std::f32::consts::PI;
    let root2 = 2.0_f32.sqrt();
    let half_qtr = Transform::rotation_z(pi / 4.0);
    assert_eq!(half_qtr * p, Tuple::point(-root2 / 2.0, root2 / 2.0, 0.0));
    let full_qtr = Transform::rotation_z(pi / 2.0);
    let result = full_qtr * p;
    let expected = Tuple::point(-1.0, 0.0, 0.0);
    result.assert_approx_eq(&expected);
}

#[test]
fn shearing() {
    let t1 = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let p = Tuple::point(2.0, 3.0, 4.0);
    assert_eq!(t1 * p, Tuple::point(5.0, 3.0, 4.0));
    let t1 = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(t1 * p, Tuple::point(6.0, 3.0, 4.0));
    let t1 = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    assert_eq!(t1 * p, Tuple::point(2.0, 5.0, 4.0));
    let t1 = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    assert_eq!(t1 * p, Tuple::point(2.0, 7.0, 4.0));
    let t1 = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    assert_eq!(t1 * p, Tuple::point(2.0, 3.0, 6.0));
    let t1 = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    assert_eq!(t1 * p, Tuple::point(2.0, 3.0, 7.0));
}

#[test]
fn individual_multiple_transforms() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let a = Transform::rotation_x(std::f32::consts::PI / 2.0);
    let b = Transform::scaling(Tuple::point(5.0, 5.0, 5.0));
    let c = Transform::translation(Tuple::point(10.0, 5.0, 7.0));
    // Apply rotation first
    let p2 = a * p;
    p2.assert_approx_eq(&Tuple::point(1.0, -1.0, 0.0));
    // then apply scaling
    let p3 = b * p2;
    p3.assert_approx_eq(&Tuple::point(5.0, -5.0, 0.0));
    let p4 = c * p3;
    p4.assert_approx_eq(&Tuple::point(15.0, 0.0, 7.0));
}

#[test]
fn chained_multiple_transform() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let a = Transform::rotation_x(std::f32::consts::PI / 2.0);
    let b = Transform::scaling(Tuple::point(5.0, 5.0, 5.0));
    let c = Transform::translation(Tuple::point(10.0, 5.0, 7.0));
    let t = c * b * a;
    assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
}
