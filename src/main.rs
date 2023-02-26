pub mod data;
pub mod draw;
mod projectile;
pub mod transforms;

fn main() {
    println!("Hello, world!");
    projectile::run();
}
