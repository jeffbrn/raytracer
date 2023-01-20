use std::cmp::{PartialEq, Eq};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Tuple {
    pub items: [f32; 4],
}

impl Tuple {
    pub fn zero() -> Tuple {
        Tuple { items: [0.0, 0.0, 0.0, 0.0] }
    }
    pub fn init(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { items: [x, y, z, w] }
    }
    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { items: [x, y, z, 1.0] }
    }
    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple { items: [x, y, z, 0.0] }
    }

    pub fn x(&self) -> f32 {
        self.items[0]
    }

    pub fn y(&self) -> f32 {
        self.items[1]
    }

    pub fn z(&self) -> f32 {
        self.items[2]
    }

    pub fn w(&self) -> f32 {
        self.items[3]
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.items[0] == other.items[0] && self.items[1] == other.items[1] && self.items[2] == other.items[2] && self.items[3] == other.items[3]
    }
}

impl Eq for Tuple {}

use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign};

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        let tp = self.items[3] + other.items[3];
        assert!(tp == 0.0 || tp == 1.0);
        Tuple { items: [self.items[0] + other.items[0], self.items[1] + other.items[1], self.items[2] + other.items[2], tp] }
    }
}

impl AddAssign<Tuple> for Tuple {
    fn add_assign(&mut self, other: Tuple) {
        self.items[0] += other.items[0];
        self.items[1] += other.items[1];
        self.items[2] += other.items[2];
        self.items[3] += other.items[3];
        assert!(self.items[3] == 0.0 || self.items[3] == 1.0);
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        let tp = self.items[3] - other.items[3];
        assert!(tp == 0.0 || tp == 1.0);
        Tuple { items: [self.items[0] - other.items[0], self.items[1] - other.items[1], self.items[2] - other.items[2], tp] }
    }
}

impl SubAssign<Tuple> for Tuple {
    fn sub_assign(&mut self, other: Tuple) {
        self.items[0] -= other.items[0];
        self.items[1] -= other.items[1];
        self.items[2] -= other.items[2];
        self.items[3] -= other.items[3];
        assert!(self.items[3] == 0.0 || self.items[3] == 1.0);
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple { items: [-self.items[0], -self.items[1], -self.items[2], self.items[3]] }
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, scaler: f32) -> Tuple {
        Tuple { items: [scaler*self.items[0], scaler*self.items[1], scaler*self.items[2], scaler*self.items[3]] }
    }
}

impl MulAssign<f32> for Tuple {
    fn mul_assign(&mut self, rhs: f32) {
        self.items[0] *= rhs;
        self.items[1] *= rhs;
        self.items[2] *= rhs;
        self.items[3] *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::data::Tuple;

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

}
