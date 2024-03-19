use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use winit::keyboard::KeyCode::*;
use crate::Components;
use crate::Frame::GameFrame;
use crate::Math::Vector3;

use crate::Components::{AToAny, Component};

pub struct Entity
{
    pub world_position: Vector3,
    pub scale: Vector3,
    _components : Vec<Rc<RefCell<dyn Components::Component>>>
}
impl Entity
{
    pub fn new(position: Vector3) -> Self
    {
        Entity
        {
            world_position: position,
            scale: Vector3::one(),
            _components: Vec::new()
        }
    }

    pub fn move_to(&mut self, position: Vector3)
    {
        self.world_position = position;
    }

    pub fn add_component(&mut self, component : Rc<RefCell<dyn Component>>)
    {
        self._components.push(component);
    }

    pub fn get_component<T>(&self) -> Option<&Rc<RefCell<T>>>
    {
        panic!("");
        /*
        for x in &self._components
        {
            if x.borrow().name() == std::any::type_name::<T>() //componentType
            {
                let anyType: &dyn Any = x.clone().as_any();
                return match anyType.downcast_ref::<Rc<RefCell<T>>>()
                {
                    Some(i) => { Some(i) },
                    None => { None }
                }
            }
        }

        return None;*/
    }

    pub fn update(&mut self, frame: &GameFrame)
    {
        let components = &self._components.clone();

        for component in components
        {
            component.borrow_mut().update( Rc::new(RefCell::new(self)), &frame);
        }
    }

    pub fn render(&mut self, frame: &GameFrame)
    {
        let components = &self._components.clone();

        for component in components
        {
            component.borrow_mut().render(self, frame);
        }
    }
}

