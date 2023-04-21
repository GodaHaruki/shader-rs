use crate::point::*;
use crate::triangle::*;

pub trait Shader {
    fn new(x: usize, y: usize) -> Self;
    fn render(&mut self);
}

struct Pixel{
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
impl Color{
    fn new(r: u8, g: u8, b: u8) -> Self{
        Color{
            r,
            g,
            b,
        }
    }
}

struct Shader3D<T>{
    polygons: Vec<Triangle3D<T>>,
    display: Vec<Vec<Pixel>>
}

impl<T> Shader for Shader3D<T>{
    fn new(x: usize, y: usize) -> Self{
        Shader3D{
            polygons: Vec::<Triangle3D<T>>::new(),
            display: vec![vec![Pixel::new(0,0,0); x]; y],
        }
    }
    
    fn render(&mut self)
}

  
