use num_traits::Float;

pub trait Is3D<T>
where T: Float {
    /// Should return the x-coordinate
    fn x(&self) -> T;
    /// Should return the y-coordinate
    fn y(&self) -> T;
    /// Should return the z-coordinate
    fn z(&self) -> T;
}
