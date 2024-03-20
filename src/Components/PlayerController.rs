use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use winit::keyboard::KeyCode::*;
use crate::Components::Component;
use crate::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use crate::Math::Vector3;

#[derive(Copy, Clone)]
enum EPlayerState
{
    idle = 0,
    run = 1,
}

#[derive(Copy, Clone)]
enum EDirection
{
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3
}

pub struct PlayerController
{
    pub SpriteRenderer: Rc<RefCell<Renderer2D>>,
    pub _speed : f32,
    pub _velocity: Vector3,

    _spriteTable: [Rc<Sprite>; 8],

    _state : EPlayerState,
    _direction :  EDirection
}

const RUN_SPEED: f32 = 0.01;

impl PlayerController
{
    pub fn new(movementSpeed: f32, renderer: Rc<RefCell<Renderer2D>>) -> Self
    {
        let display = &renderer.borrow().Display;
        Self
        {
            SpriteRenderer: renderer.clone(),
            _speed: movementSpeed,
            _velocity: Vector3::zero(),
            _spriteTable:
                [
                    Sprite::new_simple("Images/idle_down.png", display),
                    Sprite::new_simple("Images/idle_up.png", display),
                    Sprite::new_simple("Images/idle_left.png", display),
                    Sprite::new_simple("Images/idle_right.png", display),
                    Sprite::new("Images/run_down.png", display, 4,(2,2), RUN_SPEED),
                    Sprite::new("Images/run_up.png", display, 4, (2,2), RUN_SPEED),
                    Sprite::new("Images/run_left.png", display, 4, (2,2), RUN_SPEED),
                    Sprite::new("Images/run_right.png", display, 4, (2,2), RUN_SPEED)
                ],

            _state: EPlayerState::idle,
            _direction: EDirection::Up
        }
    }

    fn animation_update(&self, state: EPlayerState, direction: EDirection)
    {
        let mut renderer = self.SpriteRenderer.borrow_mut();


        let index = (state as usize  * 4) + direction as usize;

        renderer.set_new_sprite(self._spriteTable[index].clone());
    }
}

impl Component for PlayerController
{
    fn start(&mut self)
    {

    }

    fn update(&mut self, entity: Rc<RefCell<&mut Entity>>,  frame: &GameFrame)
    {
        let leftVector = if frame.Input.IsKeyDown(KeyA) {-1.0f32} else {0.0};
        let rightVector = if frame.Input.IsKeyDown(KeyD) {1.0f32} else {0.0};
        let upVector = if frame.Input.IsKeyDown(KeyW) {1.0f32} else {0.0};
        let downVector = if frame.Input.IsKeyDown(KeyS) {-1.0f32} else {0.0};
        let forwardVector = if frame.Input.IsKeyDown(KeyU) {1.0f32} else {0.0};
        let backVector = if frame.Input.IsKeyDown(KeyJ) {-1.0f32} else {0.0};

        let movementVector =
            Vector3::new(leftVector + rightVector, upVector + downVector, forwardVector + backVector)
            .normalized();

        let targetVector = Vector3::scale_value(movementVector, self._speed);

        let mut damping : f32 = 0.0;

        if movementVector.magnitude() > 0.0001
        {
            damping = 1.0;
            self._state = EPlayerState::run;
        }
        else
        {
            damping = 0.1;
            self._state = EPlayerState::idle;
        }

        self._velocity = Vector3::Lerp(self._velocity, targetVector, damping);

        if self._velocity.x() < 0.0
        {
            self._direction = EDirection::Left;
        }
        else if self._velocity.x() > 0.0
        {
            self._direction = EDirection::Right;
        }

        if self._velocity.y() < 0.0
        {
            self._direction = EDirection::Up;
        }
        else if self._velocity.y() > 0.0
        {
            self._direction = EDirection::Down;
        }

        entity.borrow_mut().world_position.add(self._velocity);
        self.animation_update(self._state, self._direction);
    }

    fn render(&self, entity: &Entity, frame: &GameFrame)
    {

    }
}



