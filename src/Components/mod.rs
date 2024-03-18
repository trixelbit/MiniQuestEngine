pub mod PlayerController;
pub mod Renderer2D;
mod RenderUtilities;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;

/// Behavior that is attached to entities.
///
///
pub trait Component
{
    /// Returns the name of the components concrete type.
    fn name(&self) -> String
    {
        std::any::type_name::<Self>().to_string()
    }

    /// Called at the start of the object lifetime.
    fn start(&mut self);

    /// Called every frame while the object is alive.
    fn update(&mut self, entity: Rc<RefCell<&mut Entity>>,  frame: Rc<GameFrame>);

    fn render(&self, entity: &Entity, display: &Display<WindowSurface>);
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

