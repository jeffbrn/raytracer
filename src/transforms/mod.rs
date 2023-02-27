pub mod transform_impl;
#[cfg(test)]
mod transform_tests;

use super::data::matrix::Matrix;
use super::data::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    tm: Matrix,
}

impl Transform {
    pub fn trans(rot: &Matrix, pnt: Tuple) -> Transform {
        assert_eq!(rot.side(), 3);
        assert_eq!(pnt.w(), 1.0);
        Transform {
            tm: Matrix::matrix4(
                rot.e(0, 0),
                rot.e(0, 1),
                rot.e(0, 2),
                pnt.x(),
                rot.e(1, 0),
                rot.e(1, 1),
                rot.e(1, 2),
                pnt.y(),
                rot.e(2, 0),
                rot.e(2, 1),
                rot.e(2, 2),
                pnt.z(),
                0.0,
                0.0,
                0.0,
                pnt.w(),
            ),
        }
    }
    pub fn scaling(pnt: Tuple) -> Transform {
        assert_eq!(pnt.w(), 1.0);
        Transform {
            tm: Matrix::matrix4(
                pnt.x(),
                0.0,
                0.0,
                0.0,
                0.0,
                pnt.y(),
                0.0,
                0.0,
                0.0,
                0.0,
                pnt.z(),
                0.0,
                0.0,
                0.0,
                0.0,
                pnt.w(),
            ),
        }
    }
    pub fn inverse(&self) -> Option<Transform> {
        let inv = self.tm.inverse();
        inv?;
        Some(Transform { tm: inv.unwrap() })
    }
    pub fn rotation_x(radians: f32) -> Transform {
        Transform {
            tm: Matrix::matrix4(
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                radians.cos(),
                -radians.sin(),
                0.0,
                0.0,
                radians.sin(),
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ),
        }
    }
    pub fn rotation_y(radians: f32) -> Transform {
        Transform {
            tm: Matrix::matrix4(
                radians.cos(),
                0.0,
                radians.sin(),
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                -radians.sin(),
                0.0,
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ),
        }
    }
    pub fn rotation_z(radians: f32) -> Transform {
        Transform {
            tm: Matrix::matrix4(
                radians.cos(),
                -radians.sin(),
                0.0,
                0.0,
                radians.sin(),
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ),
        }
    }
}
