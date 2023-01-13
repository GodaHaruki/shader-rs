pub struct Point3D<T: Sized> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct Point2D<T: Sized> {
    pub x: T,
    pub y: T,
}

pub trait Point {}

impl<T> Point for Point2D<T> {}

impl<T> Point for Point3D<T> {}
