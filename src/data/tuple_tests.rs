use super::tuple::Tuple;
use approx::assert_relative_eq;

#[test]
fn init() {
    let t1 = Tuple::zero();
    assert_eq!(t1.x(), 0.0);
    assert_eq!(t1.y(), 0.0);
    assert_eq!(t1.z(), 0.0);
    assert_eq!(t1.w(), 0.0);
    let t2 = Tuple::init(1.0, -2.0, 3.0, -4.0);
    assert_eq!(t2.x(), 1.0);
    assert_eq!(t2.y(), -2.0);
    assert_eq!(t2.z(), 3.0);
    assert_eq!(t2.w(), -4.0);

    let p = Tuple::point(4.3, -4.2, 3.1);
    assert_eq!(p.x(), 4.3);
    assert_eq!(p.y(), -4.2);
    assert_eq!(p.z(), 3.1);
    assert_eq!(p.w(), 1.0);

    let v = Tuple::vector(4.3, -4.2, 3.1);
    assert_eq!(v.x(), 4.3);
    assert_eq!(v.y(), -4.2);
    assert_eq!(v.z(), 3.1);
    assert_eq!(v.w(), 0.0);
}

#[test]
fn equality() {
    let t1 = Tuple::zero();
    let t2 = Tuple::zero();
    assert_eq!(t1, t2);

    let p1 = Tuple::point(1.0, 2.0, 3.0);
    let p2 = Tuple::point(1.0, 2.0, 3.0);
    let p3 = Tuple::point(1.0, 3.0, 3.0);
    assert_eq!(p1, p2);
    assert_ne!(p1, p3);

    let v1 = Tuple::vector(1.0, 2.0, 3.0);
    assert_ne!(p1, v1);
    let v2 = Tuple::vector(1.0, 2.0, 3.0);
    let v3 = Tuple::vector(1.0, 2.0, 2.0);
    assert_eq!(v1, v2);
    assert_ne!(v1, v3);
}

#[test]
fn add_tuples() {
    let mut a1 = Tuple::point(3.0, -2.0, 5.0);
    let a2 = Tuple::vector(-2.0, 3.0, 1.0);
    assert_eq!(a1+a2, Tuple::point(1.0, 1.0, 6.0));
    a1 += a2;
    assert_eq!(a1, Tuple::point(1.0, 1.0, 6.0));
    let v1 = Tuple::vector(1.0, -1.0, -4.5);
    assert_eq!(a1+v1, Tuple::point(2.0, 0.0, 1.5));
    a1 += v1;
    assert_eq!(a1, Tuple::point(2.0, 0.0, 1.5));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn cannot_add_points() {
    let _v = Tuple::point(1.0, 2.0, 3.0) + Tuple::point(2.0, 3.0, 4.0);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn cannot_add_assign_points() {
    let mut v = Tuple::point(1.0, 2.0, 3.0);
    v += Tuple::point(2.0, 3.0, 4.0);
}

#[test]
fn subtract_tuples() {
    let mut p1 = Tuple::point(3.0, 2.0, 1.0);
    let p2 = Tuple::point(5.0, 6.0, 7.0);
    assert_eq!(p1-p2, Tuple::vector(-2.0, -4.0, -6.0));
    let v2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(p1-v2, Tuple::point(-2.0, -4.0, -6.0));
    p1 -= p2;
    assert_eq!(p1, Tuple::vector(-2.0, -4.0, -6.0));
    let mut v1 = Tuple::vector(3.0, 2.0, 1.0);
    assert_eq!(v1-v2, Tuple::vector(-2.0, -4.0, -6.0));
    v1 -= v2;
    assert_eq!(v1, Tuple::vector(-2.0, -4.0, -6.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn cannot_sub_binary_vp() {
    let _v = Tuple::vector(1.0, 2.0, 3.0) - Tuple::point(2.0, 3.0, 4.0);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn cannot_sub_mono_vp() {
    let mut v = Tuple::vector(1.0, 2.0, 3.0);
    v -= Tuple::point(2.0, 3.0, 4.0);
}

#[test]
fn negate_tuples() {
    let zero = Tuple::zero();
    let v = Tuple::vector(1.0, -2.0, 3.0);
    assert_eq!(zero-v, Tuple::vector(-1.0, 2.0, -3.0));
    assert_eq!(-v, zero-v);
}

#[test]
fn scalar_multiply() {
    let mut a = Tuple::init(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, Tuple::init(3.5, -7.0, 10.5, -14.0));
    a *= 0.5;
    assert_eq!(a, Tuple::init(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn magnitude() {
    let v1 = Tuple::vector(1.0, 0.0, 0.0);
    assert_eq!(v1.magnitude(), 1.0);
    let v2 = Tuple::vector(0.0, 1.0, 0.0);
    assert_eq!(v2.magnitude(), 1.0);
    let v3 = Tuple::vector(0.0, 0.0, 1.0);
    assert_eq!(v3.magnitude(), 1.0);
    let v4 = Tuple::vector(1.0, 2.0, 3.0);
    let expected : f32 = 14.0;
    assert_eq!(v4.magnitude(), expected.sqrt());
    let v5 = Tuple::vector(-1.0, -2.0, -3.0);
    assert_eq!(v5.magnitude(), expected.sqrt());
}

#[test]
fn normalize() {
    let v1 = Tuple::vector(4.0, 0.0, 0.0);
    assert_eq!(v1.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    let v2 = Tuple::vector(1.0, 2.0, 3.0);
    let mag = v2.magnitude();
    let v3 = v2.normalize();
    assert_eq!(v3, Tuple::vector(v2.x()/mag,v2.y()/mag,v2.z()/mag));
    assert_relative_eq!(v3.magnitude(), 1.0, epsilon=1e-5);
}

#[test]
fn dot_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert_eq!(a.dot(&b), 20.0);
}

#[test]
fn cross_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert_eq!(a*b, Tuple::vector(-1.0, 2.0, -1.0));
    assert_eq!(b*a, Tuple::vector(1.0, -2.0, 1.0));
}
