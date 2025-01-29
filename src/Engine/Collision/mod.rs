pub mod collider;

use uuid::Uuid;
use crate::Engine::Collision::collider::{ColliderData, ECollisionType};
use crate::Engine::Math::Float3;


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

    /// Checks if the provided entity will collide with any solid objects at the given position.
    pub fn IsThereSolidCollisionAt(&self, id: &Uuid, position: Float3) -> bool
    {
        return self.IsThereCollisionAt(id, position, true)
    }
    pub fn IsThereAnyCollisionAt(&self, id: &Uuid, position: Float3) -> bool
    {
        return self.IsThereCollisionAt(id, position, false)
    }

    fn IsThereCollisionAt(&self, id: &Uuid, position: Float3, excludeTriggers: bool) -> bool
    {
        // maybe should cache to prevent double traversal
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

            if excludeTriggers
            {
                if self._colliders[i].Type() != ECollisionType::Solid
                {
                    continue;
                }
            }

            if ColliderData::DoBoundsCollide(&collider, &self._colliders[i])
            {
                return true;
            }
        }

        false
    }

    /// Adds a collider to Collision Module.
    pub fn Add(&mut self, id: Uuid, collider: ColliderData)
    {
        self._ids.push(id);
        self._colliders.push(collider);
    }

    /// Updates the position of the collider in module.
    pub fn UpdateOrigin(&mut self, id: Uuid, position: Float3)
    {
        let index = self.FindIndex(&id).unwrap();

        self._colliders[index].UpdateOrigin(position);
    }


    /// Updates the size of collision module.
    pub fn UpdateSize(&mut self, id: Uuid, size: Float3)
    {
        let index = self.FindIndex(&id).unwrap();

        self._colliders[index].UpdateSize(size);
    }

    /// Removes ColliderData from Module.
    pub fn Remove(&mut self, id: Uuid)
    {
        let index = self.FindIndex(&id).unwrap();

        self._colliders.remove(index);
        self._ids.remove(index);
    }

    fn FindIndex(&self, id: &Uuid) -> Option<usize>
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
