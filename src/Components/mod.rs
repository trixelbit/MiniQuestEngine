pub mod PlayerController;
pub mod RenderComponents;
mod RenderUtilities;
pub(crate) mod Camera;

use std::any::Any;
use std::sync::{Arc, RwLock};

use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use downcast_rs::{impl_downcast, DowncastSync, Downcast};
/// Behavior that is attached to entities.
pub trait Component: Downcast
{
    /// Returns the name of the components concrete type.
    fn ComponentTypeName(&self) -> String
    {
        std::any::type_name::<Self>().to_string()
    }

    /// Called at the start of the object lifetime.
    fn start(&mut self, entity: &mut Entity);

    /// Called every frame while the object is alive.
    fn update(&mut self, entity: &mut Entity,  frame: &GameFrame);

}
impl_downcast!(Component);
