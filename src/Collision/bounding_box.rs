use crate::Math::Float3;


pub struct BoundingBox
{
    _origin: Float3,
    _size: Float3,
}

impl BoundingBox
{
    pub fn Size(&self) -> Float3
    {
        self._size
    }
}


pub enum ECollisionType
{
    /// Used to restrict phsyical movement and prevent clipping.
    Solid,

    /// Used to listen for object entering or exiting bounds.
    Trigger,
}
