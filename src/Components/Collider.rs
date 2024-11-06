

use crate::GameEntity::Entity;
use crate::Components::Component;
use crate::Frame::GameFrame;
use crate::GameAPI::GameAPI;
use crate::Math::Float3;


use std::sync::{RwLock, Mutex, Arc};
use std::rc::Rc;

use crate::Collision::collider::{ColliderData, ECollisionType, ECollisionTag};

/// This component reports current collision data to collision module for most recent information
pub struct Collider
{
    _data: ColliderData,
    // TODO: Add Offset
}

impl Collider
{
    pub fn Create(position: Float3, size: Float3, collisionType: ECollisionType, tag: ECollisionTag) -> Rc<RwLock<Self>>
    {
        Rc::new(
            RwLock::new(
                Self
                {
                    _data: ColliderData::Create(
                        position, 
                        size,
                        collisionType,
                        tag
                    )
                }
            )
        )
    }
}

impl Component for Collider
{
    fn start(&mut self, 
        entity: &mut Entity, 
        api: Arc<Mutex<GameAPI>>)
    {
        api.lock().unwrap().Collision.Add(entity.ID(), self._data);
        api.lock().unwrap().Collision.UpdateOrigin(entity.ID(), entity.world_position);
    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        api.lock().unwrap().Collision.UpdateOrigin(entity.ID(), entity.world_position);
    }

    fn OnDestroy(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>) 
    {
        api.lock().unwrap().Collision.Remove(entity.ID());
    }
}



