mod point;
mod scanline;
mod triangle;

use crate::{Rgba, RgbaImage};
pub use point::*;
pub use scanline::*;
pub use triangle::*;

use rand::rngs::SmallRng;

pub trait Shape {
    fn rasterize(&self, w: u32, h: u32) -> Vec<Scanline>;
    fn random(w: u32, h: u32, rng: &mut SmallRng) -> Self;
    fn valid(&self) -> bool;
    fn mutate(&mut self, w: u32, h: u32, rng: &mut SmallRng);
    fn draw(&self, img: &mut RgbaImage, color: &Rgba<u8>);
}
