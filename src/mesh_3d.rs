use nalgebra::Scalar;
use num_traits::Float;
use crate::face::Face;
use crate::is_3d::Is3D;
use crate::point_cloud_3d::PointCloud3D;

#[derive(Debug)]
pub struct  Mesh3D<T, U>
where
    T: Scalar + Float,
    U: Is3D<T>
{
    vertices: Vec<Face>,
    points: PointCloud3D<T, U>,
}

impl<T, U> Mesh3D<T, U>
where
    T: Scalar + Float,
    U: Is3D<T>
{
    fn new() -> Self {
        Mesh3D {
            vertices: Vec::new(),
            points: PointCloud3D::new(),
        }
    }
}