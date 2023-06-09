use crate::data::tuple::Tuple;

#[derive(Debug)]
pub struct Projectile {
    pub posn: Tuple,
    pub v: Tuple,
}

pub struct Environment {
    pub g: Tuple,
    pub wind: Tuple,
}

pub fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let next_posn = proj.posn + proj.v;
    let next_v = proj.v + env.g + env.wind;
    Projectile {
        posn: next_posn,
        v: next_v,
    }
}

pub fn run() {
    let e = Environment {
        g: Tuple::vector(0.0, -9.8, 0.0),
        wind: Tuple::vector(0.0, 0.0, 0.0),
    };
    let start_p = Tuple::point(0.0, 1.0, 0.0);
    let start_v = Tuple::vector(10.0, 10.0, 0.0);
    let mut p = Projectile {
        posn: start_p,
        v: start_v,
    };
    while p.posn.y() > 0.0 {
        println!("{:?}", p);
        p = tick(&e, p)
    }
    println!("{:?}", p);
}
