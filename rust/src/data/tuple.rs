#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    items: [f32; 4],
}

impl Tuple {
    pub fn zero() -> Tuple {
        Tuple {
            items: [0.0, 0.0, 0.0, 0.0],
        }
    }
    pub fn init(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            items: [x, y, z, w],
        }
    }
    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            items: [x, y, z, 1.0],
        }
    }
    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple {
            items: [x, y, z, 0.0],
        }
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

    pub fn magnitude(&self) -> f32 {
        (self.x().powf(2.0) + self.y().powf(2.0) + self.z().powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let mag = self.magnitude();
        Tuple {
            items: [
                self.x() / mag,
                self.y() / mag,
                self.z() / mag,
                self.w() / mag,
            ],
        }
    }

    pub fn dot(&self, v: &Tuple) -> f32 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z() + self.w() * v.w()
    }
}

use std::ops::{AddAssign, MulAssign, SubAssign};

impl AddAssign<Tuple> for Tuple {
    fn add_assign(&mut self, other: Tuple) {
        self.items[0] += other.items[0];
        self.items[1] += other.items[1];
        self.items[2] += other.items[2];
        self.items[3] += other.items[3];
        assert!(self.items[3] == 0.0 || self.items[3] == 1.0);
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

impl MulAssign<f32> for Tuple {
    fn mul_assign(&mut self, rhs: f32) {
        self.items[0] *= rhs;
        self.items[1] *= rhs;
        self.items[2] *= rhs;
        self.items[3] *= rhs;
    }
}
