use std::sync::{Arc};
use glium::Display;
use glium::glutin::surface::WindowSurface;
use winit::keyboard::KeyCode::*;
use crate::Components::{Component};
use crate::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use crate::Math::Float3;

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
    pub _movementSpeed: f32,
    pub _velocity: Float3,

    _spriteTable: [Arc<Sprite>; 8],

    _state : EPlayerState,
    _direction :  EDirection
}

const RUN_SPEED: f32 = 0.01;

impl PlayerController
{
    pub fn new(movementSpeed: f32, display: &Display<WindowSurface>) -> Self
    {
        Self
        {
            _movementSpeed: movementSpeed,
            _velocity: Float3::zero(),
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

    fn animation_update(&self, entity: &mut Entity, state: EPlayerState, direction: EDirection)
    {
        let componentOption = entity.get_component::<Renderer2D>(Some(self));

        if componentOption.is_none()
        {
            return;
        }

        let mut renderer = componentOption.unwrap();

        let index = (state as usize  * 4) + direction as usize;

        renderer.write().unwrap().set_new_sprite(self._spriteTable[index].clone());
    }
}

impl Component for PlayerController
{
    fn start(&mut self, entity: &mut Entity)
    {
    }

    fn update(&mut self, entity: &mut Entity,  frame: &GameFrame)
    {
        let leftVector = if frame.Input.IsKeyDown(KeyA) {-1.0f32} else {0.0};
        let rightVector = if frame.Input.IsKeyDown(KeyD) {1.0f32} else {0.0};
        let upVector = if frame.Input.IsKeyDown(KeyW) {1.0f32} else {0.0};
        let downVector = if frame.Input.IsKeyDown(KeyS) {-1.0f32} else {0.0};
        let forwardVector = if frame.Input.IsKeyDown(KeyU) {1.0f32} else {0.0};
        let backVector = if frame.Input.IsKeyDown(KeyJ) {-1.0f32} else {0.0};

        let movementVector =
            Float3::new(leftVector + rightVector, upVector + downVector, forwardVector + backVector)
            .normalized();

        let targetVector = Float3::scale_value(movementVector, self._movementSpeed);

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

        self._velocity = Float3::Lerp(self._velocity, targetVector, damping);

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

        entity.world_position.add(self._velocity);
        self.animation_update(entity, self._state, self._direction);
    }
}