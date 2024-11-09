pub mod PlayerController;
pub mod RenderComponents;
pub mod AudioSource;
pub mod Collider;
pub mod Camera;
pub mod Bullet;
mod RenderUtilities;
mod ComponentTemplate;


use std::sync::{Arc, Mutex};
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use downcast_rs::{impl_downcast, DowncastSync, Downcast};
use crate::GameAPI::GameAPI;


/// Behavior that is attached to entities.
pub trait Component: Downcast
{
    /// Returns the name of the components concrete type.
    fn ComponentTypeName(&self) -> String
    {
        std::any::type_name::<Self>().to_string()
    }

    /// Called at the start of the object lifetime.
    fn start(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>);

    /// Called every frame while the object is alive.
    fn update(&mut self, entity: &mut Entity,  frame: &GameFrame, api: Arc<Mutex<GameAPI>>);

    // Called when this entity is marked for deletion but hasn't been deleted yet.
    fn OnDestroy(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>){}

    /// Destroy the GameEntity that this is attachd too.
    fn DestroyEntity(&mut self, entity: &mut Entity, api: &mut GameAPI)
    {
        api.SceneManager.DestroyEntity(entity.ID());
    }
}
impl_downcast!(Component);
