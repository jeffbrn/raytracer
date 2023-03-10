#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    side: i8,
    items: [f32; 16],
}

impl Matrix {
    //pub fn len(&self) -> i8 {
    //  self.side * self.side
    pub fn item(&self, i: i8) -> f32 {
        assert!(i >= 0 && i < (self.side * self.side));
        self.items[i as usize]
    }
    pub fn e(&self, row: i8, col: i8) -> f32 {
        assert!(row >= 0 && row < self.side);
        assert!(col >= 0 && col < self.side);
        let idx: usize = (row * self.side + col) as usize;
        self.items[idx]
    }
    pub fn side(&self) -> i8 {
        self.side
    }

    pub fn zero(side: i8) -> Matrix {
        Matrix {
            side,
            items: [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        }
    }
    pub fn identity(side: i8) -> Matrix {
        assert!((2..5).contains(&side));
        match side {
            2 => Matrix::matrix2(1.0, 0.0, 0.0, 1.0),
            3 => Matrix::matrix3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            _ => Matrix::matrix4(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
        }
    }
    pub fn matrix2(p11: f32, p12: f32, p21: f32, p22: f32) -> Matrix {
        Matrix {
            side: 2,
            items: [
                p11, p12, p21, p22, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn matrix3(
        p11: f32,
        p12: f32,
        p13: f32,
        p21: f32,
        p22: f32,
        p23: f32,
        p31: f32,
        p32: f32,
        p33: f32,
    ) -> Matrix {
        Matrix {
            side: 3,
            items: [
                p11, p12, p13, p21, p22, p23, p31, p32, p33, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn matrix4(
        p11: f32,
        p12: f32,
        p13: f32,
        p14: f32,
        p21: f32,
        p22: f32,
        p23: f32,
        p24: f32,
        p31: f32,
        p32: f32,
        p33: f32,
        p34: f32,
        p41: f32,
        p42: f32,
        p43: f32,
        p44: f32,
    ) -> Matrix {
        Matrix {
            side: 4,
            items: [
                p11, p12, p13, p14, p21, p22, p23, p24, p31, p32, p33, p34, p41, p42, p43, p44,
            ],
        }
    }
    pub fn new(side: i8, array: &[f32]) -> Matrix {
        assert!((2..5).contains(&side));
        match side {
            2 => Matrix::matrix2(array[0], array[1], array[2], array[3]),
            3 => Matrix::matrix3(
                array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7],
                array[8],
            ),
            _ => Matrix::matrix4(
                array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7],
                array[8], array[9], array[10], array[11], array[12], array[13], array[14],
                array[15],
            ),
        }
    }
}
