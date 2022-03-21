use std::{
    hash::{Hash, Hasher},
};
use std::ops::AddAssign;

extern crate nalgebra as na;
use na::{Point3, Scalar};
use num_traits::Float;
use num_traits::identities::Zero;
use crate::is_3d::Is3D;
use crate::movable_3d::Movable3D;
use crate::util::ConvertFp;

#[repr(transparent)]
#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Point3D<T>
where T: Scalar + Zero + Float {
    point: Point3<T>,
}

impl<T: Scalar + Float> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3D {
            point: Point3::new(x, y, z),
        }
    }
}

impl<T: Scalar + Float> From<[T; 3]> for Point3D<T> {
    fn from(coords: [T; 3]) -> Self {
        Point3D {
            point: Point3::from(coords),
        }
    }
}

// remove !! worthless !!!
impl<T: Scalar + Float + ConvertFp> Hash for Point3D<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.coords[0].to_bits1().hash(state);
        self.point.coords[1].to_bits1().hash(state);
        self.point.coords[2].to_bits1().hash(state);
    }
}

impl<T: Scalar + Float> Is3D<T> for Point3D<T> {

    #[inline(always)]
    fn x(&self) -> T {
        self.point.x
    }

    #[inline(always)]
    fn y(&self) -> T {
        self.point.y
    }

    #[inline(always)]
    fn z(&self) -> T {
        self.point.z
    }
}

impl<T: Clone + Float + Scalar + AddAssign> Movable3D<T> for Point3D<T> {
    fn move_by(&mut self, x: T, y: T, z: T) {
        self.point.x += x;
        self.point.y += y;
        self.point.z += z;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    use super::*;

    #[test]
    fn test_point3d_constructor() {
        let f1 = Point3D::new(0.0, 1.0, 2.0);
        assert_eq!(f1.point.x, 0.0_f64);
        assert_eq!(f1.point.y, 1.0_f64);
        assert_eq!(f1.point.z, 2.0_f64);

        let f2: Point3D<f32> = Point3D::new(0.0, 1.0, 2.0);
        assert_eq!(f2.point.x, 0.0_f32);
        assert_eq!(f2.point.y, 1.0_f32);
        assert_eq!(f2.point.z, 2.0_f32);
    }

    #[test]
    fn test_point3d() {
        let f: Point3D<f32> = Point3D::new(0.0, 3.4, 2.3);
        println!("{:?}", f);

        let c = Point3D::from([1.0, 5.3, 2.3]);
        println!("{:?}", c);

        let d: Point3D<f32> = Default::default();
        println!("{:?}", d);
    }

    #[test]
    fn test_hasher() {
        let f1: Point3D<f32> = Point3D::new(0.0, 3.4, 2.3);
        println!("{:?}", f1);

        let f2: Point3D<f32> = Point3D::new(0.0, 3.4, 2.3000001);
        println!("{:?}", f2);


        let mut hasher1 = DefaultHasher::new();
        f1.hash(&mut hasher1);
        println!("{:?}", hasher1.finish());

        let mut hasher2 = DefaultHasher::new();
        f2.hash(&mut hasher2);
        println!("{:?}", hasher2.finish());

        assert_eq!(f1, f2);

    }

}
