use num_traits::Unsigned;

#[repr(transparent)]
#[derive(Debug)]
pub struct Face
{
    point_3d_indices: [usize; 3],
}