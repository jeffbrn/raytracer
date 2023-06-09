use super::Transform;

use super::super::data::tuple::Tuple;

use crate::data::matrix::Matrix;
use std::ops::Mul;

impl Transform {
    pub fn translation(tr: Tuple) -> Transform {
        Transform::trans(&Matrix::identity(3), tr)
    }
    pub fn identity() -> Transform {
        Transform::trans2(Matrix::identity(4))
    }
    pub fn scaling2(&self, pnt: Tuple) -> Transform {
        let s = Transform::scaling(pnt);
        *self * s
    }
    pub fn rotation_x2(&self, radians: f32) -> Transform {
        let t = Transform::rotation_x(radians);
        *self * t
    }
    pub fn rotation_y2(&self, radians: f32) -> Transform {
        let t = Transform::rotation_y(radians);
        *self * t
    }
    pub fn rotation_z2(&self, radians: f32) -> Transform {
        let t = Transform::rotation_z(radians);
        *self * t
    }
    pub fn shearing2(
        &self,
        dxy: f32,
        dxz: f32,
        dyx: f32,
        dyz: f32,
        dzx: f32,
        dzy: f32,
    ) -> Transform {
        let s = Transform::shearing(dxy, dxz, dyx, dyz, dzx, dzy);
        *self * s
    }
}

impl Mul<Tuple> for Transform {
    type Output = Tuple;

    fn mul(self, pnt: Tuple) -> Tuple {
        self.tm * pnt
    }
}

impl Mul for Transform {
    type Output = Transform;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let t = self.tm * rhs.tm;
        Transform::trans2(t)
    }
}
