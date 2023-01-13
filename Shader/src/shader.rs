use crate::point::*;
use crate::triangle::*;

pub trait Shader {
    fn new(x: usize, y: usize) -> Self;
}

struct Shader2D;

impl Shader for Shader2D {
    fn new(x: usize, y: usize) -> Self {
        unimplimented!()
    }
}
