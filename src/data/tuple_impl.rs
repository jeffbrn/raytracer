use super::tuple::Tuple;

use std::cmp::{PartialEq, Eq};

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z() && self.w() == other.w()
    }
}

impl Eq for Tuple {}

use std::ops::{Add, Sub, Neg, Mul};

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        let tp = self.w() + other.w();
        assert!(tp == 0.0 || tp == 1.0);
        Tuple::init(self.x() + other.x(), self.y() + other.y(), self.z() + other.z(), tp)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        let tp = self.w() - other.w();
        assert!(tp == 0.0 || tp == 1.0);
        Tuple::init(self.x() - other.x(), self.y() - other.y(), self.z() - other.z(), tp)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::init(-self.x(), -self.y(), -self.z(), self.w())
    }
}

impl Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, scaler: f32) -> Tuple {
        Tuple::init(scaler*self.x(), scaler*self.y(), scaler*self.z(), scaler*self.w())
    }
}

impl Mul for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        Tuple::vector(
            self.y()*rhs.z() - self.z()*rhs.y(),
            self.z()*rhs.x() - self.x()*rhs.z(),
            self.x()*rhs.y() - self.y()*rhs.x()
        )
    }
}
