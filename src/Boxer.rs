use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use uuid::Uuid;
use winit::keyboard::KeyCode::*;
use crate::Engine::Collision::collider::{ECollisionTag, ECollisionType};
use crate::Engine::Components::Collider::Collider;

use crate::Engine::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::TEntity;
use crate::Engine::GameEntity::EntityHeader;
use crate::Engine::Math::Float3;

#[derive(Copy, Clone, PartialEq, Eq)]
enum EPlayerState
{
    idle,
    run,
    trot,
    jump,
    fall,
    punch1,
    punch2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum EDirection
{
    Left = 0,
    Right = 1
}


const RUN_SPEED: f32 = 0.01;
const LIGHT_SPEED: f32 = 0.015;
const MEDIUM_SPEED: f32 = 0.015;

// Sprite Asset References
const IDLE_LEFT: &str  = "Assets/boxer_idle_left.png";
const IDLE_RIGHT: &str = "Assets/boxer_idle_right.png";
    
const RUN_LEFT: &str   = "Assets/boxer_run_left.png";
const RUN_RIGHT: &str  = "Assets/boxer_run_right.png";

const JUMP_LEFT: &str   = "Assets/boxer_jump_left.png";
const JUMP_RIGHT: &str  = "Assets/boxer_jump_right.png";

const FALL_LEFT: &str   = "Assets/boxer_fall_left.png";
const FALL_RIGHT: &str  = "Assets/boxer_fall_right.png";

const TROT_LEFT: &str   = "Assets/boxer_trot_left.png";
const TROT_RIGHT: &str  = "Assets/boxer_trot_right.png";

const PUNCH1_LEFT: &str   = "Assets/boxer_1_left.png";
const PUNCH1_RIGHT: &str  = "Assets/boxer_1_right.png";

const PUNCH2_LEFT: &str   = "Assets/boxer_2_right.png";
const PUNCH2_RIGHT: &str  = "Assets/boxer_2_right.png";


const WATER_BALL_SPRITE: &str = "Assets/waterball.png";
const WATER_SHOOT_SFX: &str = "Assets/Shoot.ogg";

const GRAVITY : f32 = 0.5;
const JUMP_STRENGTH: f32 = 35.0;


pub struct Boxer
{
    pub Header: EntityHeader,
    _renderer2d: Renderer2D,
    _collider: Collider,

    pub _movementSpeed: f32,
    pub _velocity: Float3,
    pub _lastInputVector: Float3,

    _spriteTable: [Arc<Sprite>; 14],

    _state : EPlayerState,
    _direction :  EDirection,
    _display: Display<WindowSurface>,

    _waterSprite: Arc<Sprite>,
    _isAttacking: bool,
}

impl Boxer
{
    pub fn Create(movementSpeed: f32, display: &Display<WindowSurface>, position: Float3) -> Self
    {
        let default = Sprite::new(IDLE_LEFT, display, 8, (3,3), RUN_SPEED);
        Self
        {
            Header: EntityHeader::Create("Player", position),
            _movementSpeed: movementSpeed,
            _velocity: Float3::zero(),
            _lastInputVector: Float3::zero(),

            _spriteTable:
                [
                    Sprite::new(IDLE_LEFT, display, 8, (3,3), RUN_SPEED),
                    Sprite::new(IDLE_RIGHT, display, 8, (3,3), RUN_SPEED),
                    
                    Sprite::new(RUN_LEFT, display, 8, (3,3), RUN_SPEED),
                    Sprite::new(RUN_RIGHT, display, 8, (3,3), RUN_SPEED),
                    
                    Sprite::new(TROT_LEFT, display, 5, (3,2), RUN_SPEED),
                    Sprite::new(TROT_RIGHT, display, 5, (3,2), RUN_SPEED),
               
                    Sprite::new(JUMP_LEFT, display, 2, (2,1), RUN_SPEED),
                    Sprite::new(JUMP_RIGHT, display, 2, (2,1), RUN_SPEED),
                    
                    Sprite::new(FALL_LEFT, display, 2, (2,1), RUN_SPEED),
                    Sprite::new(FALL_RIGHT, display, 2, (2,1), RUN_SPEED),
               
                    Sprite::new(PUNCH1_LEFT, display, 3, (2,2), LIGHT_SPEED),
                    Sprite::new(PUNCH1_RIGHT, display, 3, (2,2), LIGHT_SPEED),
                    
                    Sprite::new(PUNCH2_LEFT, display, 4, (2,2), MEDIUM_SPEED),
                    Sprite::new(PUNCH2_RIGHT, display, 4, (2,2), MEDIUM_SPEED),
                ],

            _renderer2d: Renderer2D::New(display, default, true),
            _collider: Collider::Create(
                display.clone(),
                position,
                Float3::new(32.0,32.0,1.0),
                ECollisionType::Solid,
                ECollisionTag::None

            ),
            _state: EPlayerState::idle,
            _direction: EDirection::Left,
            _display: display.clone(),
            _waterSprite: Sprite::new_simple(WATER_BALL_SPRITE, display),
            _isAttacking: false,
        }
    }

    fn animation_update(&mut self, state: EPlayerState, direction: EDirection)
    {
        let renderer = &mut self._renderer2d;

        if self._isAttacking 
        {
            if renderer.IsComplete()
            {
                self._isAttacking = false;
            }
            else
            {
                return;
            }
        }

        if state == EPlayerState::punch1 || state == EPlayerState::punch2
        {

            let index = Self::IndexFromState(state, direction);
            renderer.SetSprite1Loop(self._spriteTable[index].clone());
            
            self._isAttacking = true;

            return;
        }

        // render all other simple states
        let index = (state as usize  * 2) + direction as usize;
       
        renderer.set_new_sprite(self._spriteTable[index].clone());
    }

    fn IndexFromState(state: EPlayerState, direction: EDirection) -> usize
    {
        (state as usize  * 2) + direction as usize
    }

    fn CheckState(&mut self)
    {

    }

    fn StateUpdate(&mut self)
    {

    }
}

impl Debug for Boxer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Boxer {}", self.Header.Name)
    }
}

impl TEntity for Boxer
{
    fn HasStartBeenCalled(&self) -> bool
    {
        self.Header.HasStartBeenCalled()
    }

    fn ID(&self) -> Uuid
    {
        self.Header.ID().clone()
    }

    unsafe fn Start(&mut self, api: *mut GameAPI)
    {
        self._collider.Start(&self.Header, api);
        self._renderer2d.Start(&self.Header, api);
    }

    unsafe fn Update(&mut self, frame: &GameFrame, api: *mut GameAPI)
    {
        ///*
        let cam_pos = (*api).SceneManager.Entities.Camera.Header.WorldPosition.clone();
        let mut player_pos = self.Header.WorldPosition.clone();

        println!("Player Pos: {}", player_pos);
        println!("Cam Pos: {}", cam_pos);

         (*api).SceneManager.Entities.Camera.Header.WorldPosition =
             Float3::Lerp(
                 cam_pos,
                 player_pos
                     .OverrideY(self.Header.WorldPosition.y() + 0.0)
                     .AddZ(256.0 * 4.0),
                 0.5f32 * frame.DeltaTime_Seconds
             );
         //*/

        self._collider.Update(&self.Header, frame, api);

        let id = &self.Header.ID();
        let entity= &mut self.Header;

        let oldDirection = self._direction;
        let oldState = self._state;


        let leftVector : f32 = if frame.Input.IsKeyDown(KeyA) {-1.0} else {0.0};
        let rightVector: f32 = if frame.Input.IsKeyDown(KeyD) {1.0} else {0.0};
        let upVector : f32 = if frame.Input.IsKeyDown(KeyW) {1.0} else {0.0};
        let downVector : f32 = if frame.Input.IsKeyDown(KeyS) {-1.0} else {0.0};

        let inputVector =
            Float3::new(leftVector + rightVector, 0.0, 0.0)
            .normalized();

        let damping;

        if inputVector.magnitude() > 0.001 && !self._isAttacking
        {
            damping = 1.0;
            self._lastInputVector = inputVector;
        }
        else
        {
            damping = 0.05;
        }

        let isGrounded =
            (*api).Collision.IsThereSolidCollisionAt(
                id,
                entity.WorldPosition
                    + Float3::new(0.0, -0.1, 0.0)
            );

        // horizontal movement
        let targetVector = Float3::scale_value(inputVector, self._movementSpeed)
            .OverrideY(self._velocity.y());


        self._velocity =
                Float3::Lerp(self._velocity, targetVector, damping);

        if self._velocity.x() < 0.0
        {
            self._direction = EDirection::Left;
        }
        else if self._velocity.x() > 0.0
        {
            self._direction = EDirection::Right;
        }

        let gravity = Float3::new(0.0, -GRAVITY, 0.0);


        // Ground behavior
        if isGrounded
        {
            self._velocity = self._velocity.OverrideY(0.0);
        }
        else
        {
            self._velocity = self._velocity + gravity;
        }

        // Jump logic
        if frame.Input.IsKeyPressed(KeyW) && isGrounded
        {
            self._velocity = self._velocity.OverrideY(JUMP_STRENGTH);
        }

        let positionDelta = Float3::scale_value(self._velocity, frame.DeltaTime_Seconds);
        let futurePosition = entity.WorldPosition + positionDelta;


        // Check for collision.
        if !(*api).Collision.IsThereSolidCollisionAt(
            &entity.ID(), futurePosition)
        { 
            entity.WorldPosition.add(Float3::scale_value(self._velocity, frame.DeltaTime_Seconds));
        }
        else
        {
            let tolerance = 0.1;
            let step = 0.1;

            // Check and see if we can apply sliding.
            let mut x_comp = Float3::new(positionDelta.x(), 0.0, 0.0);
            if !(*api).Collision.IsThereSolidCollisionAt(
                &entity.ID(),
                entity.WorldPosition + x_comp)
            {
                entity.WorldPosition.add(x_comp);
            }
            else
            {
                let mut i = positionDelta.x();
                let sign = i / f32::abs(i);

                while(
                    f32::abs(i) > tolerance )
                {
                    i = i - (sign * step);
                    x_comp.OverrideX(i);

                    println!("X OVERRIDE{}", i);

                    if !(*api).Collision.IsThereSolidCollisionAt(
                        &entity.ID(),
                        entity.WorldPosition + Float3::new(i, 0.0, 0.0))
                    {
                        entity.WorldPosition.add(x_comp);
                        break;
                    }

                }

            }

            let mut y_comp = Float3::new(0.0, positionDelta.y(), 0.0);
            if !(*api).Collision.IsThereSolidCollisionAt(
                &entity.ID(),
                entity.WorldPosition + y_comp)
            { 
                entity.WorldPosition.add(y_comp);
            }
            else
            {
                let mut i = positionDelta.y();
                let sign = i / f32::abs(i);

                while(
                    f32::abs(i) > tolerance)
                {
                    i = i - (sign * step);
                    y_comp.OverrideY(i);
                    println!("Y OVERRIDE{}", i);

                    if !(*api).Collision.IsThereSolidCollisionAt(
                        &entity.ID(),
                        entity.WorldPosition + Float3::new(0.0, i, 0.0))
                    {
                        entity.WorldPosition.add(y_comp);
                        break;
                    }
                }
            }
        }


        if(isGrounded)
        {
            if(self._velocity.x().abs() > 15.0)
            {
                self._state = EPlayerState::run;
            }
            else if(self._velocity.x().abs() > 3.0)
            {
                self._state = EPlayerState::trot;
            }
            else
            {
                self._state = EPlayerState::idle;
            }
        }
        else
        {
            if(self._velocity.y() > 0.0)
            {
                self._state = EPlayerState::jump;
            }
            else
            {
                self._state = EPlayerState::fall;
            }
        }

        if frame.Input.IsKeyPressed(KeyU) && !self._isAttacking
        {
            self._state = EPlayerState::punch1;
        }

        if frame.Input.IsKeyPressed(KeyI) && !self._isAttacking
        {
            self._state = EPlayerState::punch2;
        }

        
        if
            self._isAttacking ||
            (oldState != self._state || oldDirection != self._direction)
        {
            self.animation_update(self._state, self._direction);
        }

        println!("{}", self._velocity);
        println!("BOXER CLOSE");

    }

    unsafe fn OnDestroy(&mut self, api: *mut GameAPI)
    {
        self._collider.OnDestroy(&self.Header, api);
    }

    fn Render(&mut self, frame: &GameFrame, target: &mut Frame)
    {
        self._renderer2d.Render(&self.Header, frame, target);
    }
}

