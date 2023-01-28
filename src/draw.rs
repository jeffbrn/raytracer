#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Clr {
    pub items: [f32; 3]
}

impl Clr {
    pub fn rgb(r: f32, g: f32, b: f32) -> Clr {
        Clr { items: [r,g,b] }.normalize()
    }
    pub fn r(&self) -> f32 {
        self.items[0]
    }
    pub fn g(&self) -> f32 {
        self.items[1]
    }
    pub fn b(&self) -> f32 {
        self.items[2]
    }
    pub fn normalize(&self) -> Clr {
        let r = match self.r() {
            n if n < 0.0 => 0.0,
            n if n > 1.0 => 1.0,
            _ => self.r()
        };
        let g = match self.g() {
            n if n < 0.0 => 0.0,
            n if n > 1.0 => 1.0,
            _ => self.g()
        };
        let b = match self.b() {
            n if n < 0.0 => 0.0,
            n if n > 1.0 => 1.0,
            _ => self.b()
        };
        Clr { items: [r,g,b] }
    }
}

use std::cmp::{PartialEq, Eq};

impl PartialEq for Clr {
    fn eq(&self, other: &Self) -> bool {
        self.items[0] == other.items[0] && self.items[1] == other.items[1] && self.items[2] == other.items[2]
    }
}

impl Eq for Clr {}

use std::ops::{Add, Sub, Mul};

impl Add for Clr {
    type Output = Clr;

    fn add(self, other: Clr) -> Clr {
        Clr { items: [self.items[0] + other.items[0], self.items[1] + other.items[1], self.items[2] + other.items[2]] }.normalize()
    }
}

impl Sub for Clr {
    type Output = Clr;

    fn sub(self, other: Clr) -> Clr {
        Clr { items: [self.items[0] - other.items[0], self.items[1] - other.items[1], self.items[2] - other.items[2]] }.normalize()
    }
}

impl Mul<f32> for Clr {
    type Output = Clr;

    fn mul(self, scaler: f32) -> Clr {
        Clr { items: [scaler*self.items[0], scaler*self.items[1], scaler*self.items[2]] }.normalize()
    }
}

impl Mul for Clr {
    type Output = Clr;

    fn mul(self, rhs: Clr) -> Clr {
        Clr { items: [self.items[0]*rhs.items[0], self.items[1]*rhs.items[1], self.items[2]*rhs.items[2]] }
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::Clr;
    use approx::{assert_relative_eq};

    #[test]
    fn add_colors() {
        let c1 = Clr::rgb(0.9, 0.6, 0.75);
        let c2 = Clr::rgb(0.7, 0.1, 0.25);
        let c3 = c1 + c2;
        assert_eq!(c3.r(), 1.0);
        assert_relative_eq!(c3.g(), 0.7, epsilon=1e-5);
        assert_eq!(c3.b(), 1.0);
    }

    #[test]
    fn sub_colors() {
        let c1 = Clr::rgb(0.9, 0.6, 0.75);
        let c2 = Clr::rgb(0.7, 0.8, 0.25);
        let c3 = c1 - c2;
        assert_relative_eq!(c3.r(), 0.2, epsilon=1e-5);
        assert_eq!(c3.g(), 0.0);
        assert_eq!(c3.b(), 0.5);
    }

    #[test]
    fn scalar_multiply() {
        let c1 = Clr::rgb(0.2, 0.3, 0.4);
        assert_eq!(c1*2.0, Clr::rgb(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_multiply() {
        let c1 = Clr::rgb(1.0, 0.2, 0.4);
        let c2 = Clr::rgb(0.9, 1.0, 0.1);
        let c3 = c1 * c2;
        assert_eq!(c3.r(), 0.9);
        assert_eq!(c3.g(), 0.2);
        assert_relative_eq!(c3.b(), 0.04, epsilon=1e-5);
    }
}
