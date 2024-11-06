use crate::Math::Float3;

// Do we separate bounds from collider?
pub struct ColliderBounds
{
    _origin: Float3,
    _size: Float3,
    _extents: Float3,
    _tag: ECollisionTag,
    _collisionType: ECollisionType,
}

impl ColliderBounds
{
    pub fn Create(
        position: Float3,
        size: Float3,
        collisionType: ECollisionType,
        tag: ECollisionTag
    ) -> Self
    {
        Self
        {
            _origin: position,
            _size: size,
            _extents: Float3::scale_value(size, 0.5),
            _collisionType: collisionType,
            _tag: tag
        }
    }

    pub fn Size(&self) -> Float3
    {
        self._size
    }

    pub fn Tag(&self) -> ECollisionTag {self._tag.clone()}

    pub fn GetMinMaxBounds(&self) -> (f32, f32, f32, f32)
    {
        let corners = self.GetCorners();

        (
            // min x
            corners.0.x(),

            // max x
            corners.1.x(),

            // min y
            corners.3.y(),

            // max y
            corners.1.y()
        )
    }

    pub fn GetCorners(&self) -> (Float3, Float3, Float3, Float3)
    {
        (
            // Top Left
            self._origin + (self._extents * Float3::new(-1.0, 1.0,1.0)),

            // Top Right
            self._origin + (self._extents),

            // Bottom Right
            self._origin + (self._extents * Float3::new(1.0, -1.0,1.0)),

            // Bottom Left
            self._origin + (self._extents * Float3::new(-1.0, -1.0,1.0)),
        )
    }

    pub fn UpdateOrigin(&mut self, newPosition: Float3)
    {
        self._origin = newPosition;
    }

    pub fn UpdateSize(&mut self, newSize: Float3)
    {
        self._size = newSize;
        self._extents = Float3::scale_value(newSize, 0.5);
    }

    pub fn DoesPointIntersectBounds(point: Float3, collider: &ColliderBounds) -> bool
    {
        let minMaxValues = collider.GetMinMaxBounds();
        let x = point.x();
        let y = point.y();

        x >= minMaxValues.0 && x <= minMaxValues.1 &&
            y >= minMaxValues.2 && y <= minMaxValues.3
    }

    pub fn DoBoundsCollide(a: &ColliderBounds, b: &ColliderBounds) -> bool
    {
        let corners = Self::GetCorners(a);

        Self::DoesPointIntersectBounds(corners.0, b) ||
        Self::DoesPointIntersectBounds(corners.1, b) ||
        Self::DoesPointIntersectBounds(corners.2, b) ||
        Self::DoesPointIntersectBounds(corners.3, b)
    }
}

#[derive(Copy, Clone)]
pub enum ECollisionTag
{
    None,
}

#[derive(Copy, Clone)]
pub enum ECollisionType
{
    /// Used to restrict physical movement and prevent clipping.
    Solid,

    /// Used to listen for object entering or exiting bounds.
    Trigger,
}
