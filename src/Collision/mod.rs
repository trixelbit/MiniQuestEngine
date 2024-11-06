pub mod collider;

use uuid::Uuid;
use crate::Collision::collider::ColliderData;
use crate::Math::Float3;


pub struct CollisionModule
{
    _colliders: Vec<ColliderData>,
    _ids: Vec<Uuid>
}

impl CollisionModule
{
    pub fn Create() -> Self
    {
        Self
        {
            _colliders: Vec::new(),
            _ids: Vec::new()
        }
    }

    pub fn IsThereACollisionAt(&self, id: Uuid, position: Float3) -> bool
    { 
        // should cache to prevent double traversal  
        let indexOption = self.FindIndex(id);

        if indexOption.is_none()
        {
            println!("Warning: ID {} not found in collision system. ", id);
            return false;
        }

        let index = indexOption.unwrap();
        let mut collider = self._colliders[index].clone();
        collider.UpdateOrigin(position);

        for i in 0..self._colliders.len()
        {
            if self._ids[i].eq(&id)
            {
                continue;
            }

            if ColliderData::DoBoundsCollide(&collider, &self._colliders[i])
            {
                return true;
            }

        }

        false
    }

    pub fn Add(&mut self, id: Uuid, collider: ColliderData)
    {
        self._ids.push(id);
        self._colliders.push(collider);
    }

    pub fn UpdateOrigin(&mut self, id: Uuid, position: Float3)
    {
        let index = self.FindIndex(id).unwrap();

        self._colliders[index].UpdateOrigin(position);
    }

    pub fn UpdateSize(&mut self, id: Uuid, size: Float3)
    {
        let index = self.FindIndex(id).unwrap();

        self._colliders[index].UpdateSize(size);
    }

    pub fn Remove(&mut self, id: Uuid)
    {
        let index = self.FindIndex(id).unwrap();

        self._colliders.remove(index);
        self._ids.remove(index);
    }

    fn FindIndex(&self, id: Uuid) -> Option<usize>
    {
        for i in 0..self._ids.len()
        {
            if id.eq(&self._ids[i])
            {
                return Some(i);
            }
        }
    
        None
    }
}
