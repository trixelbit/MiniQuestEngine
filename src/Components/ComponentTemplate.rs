
use crate::GameEntity::Entity;
use crate::Components::Component;
use crate::Frame::GameFrame;
use crate::GameAPI::GameAPI;


use std::sync::{RwLock, Mutex, Arc};
use std::rc::Rc;

/// General template for creating new components
pub struct ComponentTemplate
{
}

impl ComponentTemplate
{
    pub fn Create() -> Rc<RwLock<Self>>
    {
        Rc::new(
            RwLock::new(
                Self
                {
                }
            )
        )
    }
}

impl Component for ComponentTemplate
{
    fn start(&mut self, 
        entity: &mut Entity, 
        api: Arc<Mutex<GameAPI>>)
    {
        // Add start life-time
    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        // Add update logic
    }
}
