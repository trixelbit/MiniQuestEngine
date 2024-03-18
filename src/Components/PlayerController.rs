use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use winit::keyboard::KeyCode::*;
use crate::Components::Component;
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use crate::Math::Vector3;


pub struct PlayerController
{
    pub _entity :Rc<RefCell<Entity>>,
    pub _speed : f32,
    pub _velocity: Vector3
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
        let leftVector = if frame.Input.IsKeyDown(KeyA) {-1.0f32} else {0.0};
        let rightVector = if frame.Input.IsKeyDown(KeyD) {1.0f32} else {0.0};
        let upVector = if frame.Input.IsKeyDown(KeyW) {1.0f32} else {0.0};
        let downVector = if frame.Input.IsKeyDown(KeyS) {-1.0f32} else {0.0};
        let forwardVector = if frame.Input.IsKeyDown(KeyU) {1.0f32} else {0.0};
        let backVector = if frame.Input.IsKeyDown(KeyJ) {-1.0f32} else {0.0};

        let movementVector = Vector3::new(leftVector + rightVector, upVector + downVector, forwardVector + backVector);

        let targetVector = Vector3::scale_value(movementVector, self._speed);

        let mut damping : f32 = 0.0;

        if movementVector.magnitude() > 0.0001
        {
            damping = 1.0;
        }
        else
        {
            damping = 0.1;
        }

        self._velocity = Vector3::Lerp(self._velocity, targetVector, damping);

        entity.borrow_mut().world_position.add(self._velocity);

    }

    fn render(&self, entity: &Entity, display: &Display<WindowSurface>)
    {

    }
}



