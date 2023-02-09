#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Clr {
    items: [f32; 3]
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
