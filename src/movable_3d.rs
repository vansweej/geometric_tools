pub trait Movable3D<T> {
    fn move_by(&mut self, x: T, y: T, z: T);
}