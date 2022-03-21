use std::marker::PhantomData;
use std::ops::AddAssign;
use std::ops::Index;
use nalgebra::Scalar;
use num_traits::{Float, Unsigned};
use crate::is_3d::Is3D;
use crate::movable_3d::Movable3D;
use crate::point_3d::Point3D;

#[repr(transparent)]
#[derive(Debug)]
pub struct PointCloud3D<T, U>
where
T: Scalar + Float,
U: Is3D<T>
{
    data: Vec<U>,
    _phantom: PhantomData<T>,
}

impl<T, U> PointCloud3D<T, U>
where
    T: Scalar + Float,
    U: Is3D<T>
{
    pub fn new() -> Self {
        PointCloud3D {
            data: Vec::new(),
            _phantom: Default::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, value: U) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<U> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
    }
}

impl<T, U> Index<usize> for PointCloud3D<T, U>
where
    T: Scalar + Float,
    U: Is3D<T>,
{
    type Output = U;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, U> IntoIterator for PointCloud3D<T, U>
where
    T: Scalar + Float,
    U: Is3D<T>
{
    type Item = U;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T, U> Movable3D<T> for PointCloud3D<T, U>
where
    T: Clone + Float + Scalar + AddAssign,
    U: Is3D<T> + Movable3D<T>,
{
    fn move_by(&mut self, x: T, y: T, z: T) {
        for point in self.data.iter_mut() {
            point.move_by(x, y, z);
        }
    }
}

type F32PointCloud3D = PointCloud3D::<f32, Point3D<f32>>;
type F64PointCloud3D = PointCloud3D::<f64, Point3D<f64>>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_point_cloud3d_constructor() {
        let _x = F32PointCloud3D::new();
    }

    #[test]
    fn test_f64_point_cloud3d_constructor() {
        let _x = F64PointCloud3D::new();
    }

    #[test]
    fn test_f32_point_cloud3d_iterator() {
        let x = F32PointCloud3D::new();

        assert!(x.is_empty());
    }

    #[test]
    fn test_f64_point_cloud3d_iterator() {
        let x = F64PointCloud3D::new();

        assert!(x.is_empty());
    }

    #[test]
    fn test_f32_point_cloud3d_push() {
        let mut x = F32PointCloud3D::new();
        x.push(Point3D::new(1.0, 2.0, 3.0));

        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);

        let r = x.pop();
        assert!(x.is_empty());
        assert_eq!(r, Some(Point3D::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn test_f32_point_cloud3d_movable3d() {
        let mut x = F32PointCloud3D::new();
        x.push(Point3D::new(1.0, 2.0, 3.0));
        x.push(Point3D::new(5.0, 6.0, 7.0));

        x.move_by(1.0, 1.0, 1.0);

        let point1 = x.pop().unwrap();
        assert_eq!(point1, Point3D::new(6.0, 7.0, 8.0));

        let point2 = x.pop().unwrap();
        assert_eq!(point2, Point3D::new(2.0, 3.0, 4.0));
    }

}