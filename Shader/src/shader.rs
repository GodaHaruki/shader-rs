use crate::point::*;
use crate::triangle::*;
use crate::pixel::*;

pub trait Shader {
    fn new(x: usize, y: usize) -> Self;
    fn render(&mut self);
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
    
    fn render(&mut self){
    }
}

  
