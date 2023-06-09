use super::clr::Clr;

use std::cmp::{Eq, PartialEq};

impl PartialEq for Clr {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

impl Eq for Clr {}

use std::ops::{Add, Mul, Sub};

impl Add for Clr {
    type Output = Clr;

    fn add(self, other: Clr) -> Clr {
        Clr::rgb(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}

impl Sub for Clr {
    type Output = Clr;

    fn sub(self, other: Clr) -> Clr {
        Clr::rgb(
            self.r() - other.r(),
            self.g() - other.g(),
            self.b() - other.b(),
        )
    }
}

impl Mul<f32> for Clr {
    type Output = Clr;

    fn mul(self, scaler: f32) -> Clr {
        Clr::rgb(scaler * self.r(), scaler * self.g(), scaler * self.b())
    }
}

impl Mul for Clr {
    type Output = Clr;

    fn mul(self, other: Clr) -> Clr {
        Clr::rgb(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}
