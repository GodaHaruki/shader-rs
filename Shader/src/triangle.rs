use crate::point::{Point, Point2D, Point3D};

pub trait Triangle<T: Point> {
    fn is_contained(&self, p: T) -> bool;
}

pub struct Triangle2D<T> {
    pub a: Point2D<T>,
    pub b: Point2D<T>,
    pub c: Point2D<T>,
}

pub struct Triangle3D<T> {
    pub a: Point3D<T>,
    pub b: Point3D<T>,
    pub c: Point3D<T>,
}

impl<T> Triangle<Point2D<T>> for Triangle2D<T> {
    fn is_contained(&self, p: Point2D<T>) -> bool {
        let vectol_a;
        let vectol_b;
        let vectol_c;
        unimplimented!()
    }
}
