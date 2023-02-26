use super::Transform;

use super::super::data::tuple::Tuple;

use crate::data::matrix::Matrix;
use std::ops::Mul;

impl Transform {
    pub fn translation(tr: Tuple) -> Transform {
        Transform::trans(&Matrix::identity(3), tr)
    }
}

impl Mul<Tuple> for Transform {
    type Output = Tuple;

    fn mul(self, pnt: Tuple) -> Tuple {
        self.tm * pnt
    }
}
