use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use glium::Texture2d;
use winit::keyboard::KeyCode::*;
use crate::Frame::GameFrame;
use crate::Math::Vector3;

pub struct Entity
{
    pub texture: Texture2d,
    pub world_position: Vector3,
    pub scale: Vector3,
    _components : Vec<Rc<RefCell<dyn Component>>>
}
impl Entity
{
    pub fn new(position: Vector3, sprite: Texture2d) -> Self
    {
        Entity
        {
            world_position: position,
            texture: sprite,
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

    pub fn get_component(&self, componentType : &str) -> Option<Rc<RefCell<dyn Component>>>
    {
        for x in &self._components
        {
            if x.borrow().name() == componentType
            {
                return Some(x.clone());
            }
        }

        return None;
    }

    pub fn update(&mut self, frame: Rc<GameFrame>)
    {
        let components = &self._components.clone();

        for component in components
        {
            component.borrow_mut().update( Rc::new(RefCell::new(self)), frame.clone());
        }
    }
}

pub struct PlayerController
{
    pub _entity :Rc<RefCell<Entity>>,
    pub _speed : f32
}
impl PlayerController
{
}

impl Component for PlayerController
{
    fn start(&mut self)
    {

    }

    fn update(&mut self, entity: Rc<RefCell<&mut Entity>>,  frame: Rc<GameFrame>)
    {
        //let mut entity = self.entity().borrow_mut();

        let leftVector = if frame.Input.IsKeyDown(KeyA) {-1.0f32} else {0.0};
        let rightVector = if frame.Input.IsKeyDown(KeyD) {1.0f32} else {0.0};
        let upVector = if frame.Input.IsKeyDown(KeyW) {1.0f32} else {0.0};
        let downVector = if frame.Input.IsKeyDown(KeyS) {-1.0f32} else {0.0};

        let movementVector = Vector3::new(leftVector + rightVector, upVector + downVector, 0.0);

        entity.borrow_mut().world_position.add(
            Vector3::scale_value(movementVector, self._speed));

    }
}

pub trait Component
{
    /// Returns the name of the components concrete type.
    fn name() -> String
    {
        std::any::type_name::<Self>().to_string()
    }

    /// Called at the start of the object lifetime.
    fn start(&mut self);

    /// Called every frame while the object is alive.

    fn update(&mut self, entity: Rc<RefCell<&mut Entity>>,  frame: Rc<GameFrame>);
}



