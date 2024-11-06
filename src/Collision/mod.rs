mod collider;

use uuid::Uuid;
use crate::Collision::collider::ColliderBounds;
use crate::Math::Float3;


pub struct CollisionModule
{
    _colliders: Vec<ColliderBounds>,
    _ids: Vec<Uuid>
}

impl CollisionModule
{
    pub fn Add(&mut self, id: Uuid, collider: ColliderBounds)
    {
        self._ids.push(id);
        self._colliders.push(collider);
    }

    pub fn UpdateOrigin(&mut self, id: Uuid, position: Float3)
    {
        let index = self.FindIndex(id);

        self._colliders[index].UpdateOrigin(position);
    }

    pub fn UpdateSize(&mut self, id: Uuid, size: Float3)
    {
        let index = self.FindIndex(id);

        self._colliders[index].UpdateSize(size);
    }

    pub fn Remove(&mut self, id: Uuid)
    {
        let index = self.FindIndex(id);

        self._colliders.remove(index);
        self._ids.remove(index);
    }

    fn FindIndex(&self, id: Uuid) -> usize
    {
        for i in 0..self._ids.len()
        {
            if id.eq(&self._ids[i])
            {
                return i;
            }
        }

        panic!("ID {} not found in collision system.", id);
    }
}
