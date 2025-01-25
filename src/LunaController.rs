use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Mutex;

use cgmath::num_traits::ToPrimitive;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use winit::keyboard::KeyCode::*;
use crate::Engine::Audio::sample::*;
use crate::Engine::Components::Component;
use crate::Engine::Components::Bullet::Bullet;
use crate::Engine::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameEntity::TEntity;
use crate::Engine::Math::{Float3, Ray};
use crate::Engine::GameAPI::GameAPI;

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


const RUN_SPEED: f32 = 0.01;

// Sprite Asset References
pub const IDLE_DOWN: &str  = "Assets/idle_down.png";
const IDLE_UP: &str    = "Assets/idle_up.png";
const IDLE_LEFT: &str  = "Assets/idle_left.png";
const IDLE_RIGHT: &str = "Assets/idle_right.png";
    
const RUN_DOWN: &str   = "Assets/run_down.png";
const RUN_UP: &str     = "Assets/run_up.png";
const RUN_LEFT: &str   = "Assets/run_left.png";
const RUN_RIGHT: &str  = "Assets/run_right.png";

const WATER_BALL_SPRITE: &str = "Assets/waterball.png";
const WATER_SHOOT_SFX: &str = "Assets/Shoot.ogg";

pub struct LunaController
{
    pub _movementSpeed: f32,
    pub _velocity: Float3,
    pub _lastMovementVector: Float3,

    _spriteTable: [Arc<Sprite>; 8],

    _state : EPlayerState,
    _direction :  EDirection,
    _display: Display<WindowSurface>,

    _waterSprite: Arc<Sprite>
}

impl LunaController
{
    pub fn new(movementSpeed: f32, display: &Display<WindowSurface>) -> Rc<RwLock<Self>>
    {
        Rc::new(
            RwLock::new(
                Self
                {
                    _movementSpeed: movementSpeed,
                    _velocity: Float3::zero(),
                    _lastMovementVector: Float3::zero(),

                    _spriteTable:
                        [
                            Sprite::new_simple(IDLE_DOWN, display),
                            Sprite::new_simple(IDLE_UP, display),
                            Sprite::new_simple(IDLE_LEFT, display),
                            Sprite::new_simple(IDLE_RIGHT, display),
                            Sprite::new(RUN_DOWN, display, 4,(2,2), RUN_SPEED),
                            Sprite::new(RUN_UP, display, 4, (2,2), RUN_SPEED),
                            Sprite::new(RUN_LEFT, display, 4, (2,2), RUN_SPEED),
                            Sprite::new(RUN_RIGHT, display, 4, (2,2), RUN_SPEED)
                        ],

                    _state: EPlayerState::idle,
                    _direction: EDirection::Up,
                    _display: display.clone(),
                    _waterSprite: Sprite::new_simple(WATER_BALL_SPRITE, display),
                }
            )
        )
    }

    fn animation_update(&self, entity: &mut TEntity, state: EPlayerState, direction: EDirection)
    {
        let componentOption = entity.get_component::<Renderer2D>(Some(self));

        if componentOption.is_none()
        {
            return;
        }

        let renderer = componentOption.unwrap();

        let index = (state as usize  * 4) + direction as usize;

        renderer.write().unwrap().set_new_sprite(self._spriteTable[index].clone());
    }

    /// Creates a water ball and fires it in the direction the player is facing.
    pub fn CreateWaterBall(&mut self, entity: &TEntity, api: Arc<Mutex<GameAPI>>, direction: Float3)
    {
        let waterEntity =
            Rc::new(
                RefCell::new(
                    TEntity::Create("Water", entity.world_position)
                ));

        // update constructors to return Rc RwLock wrapped Components
        waterEntity
            .borrow_mut()
            .add_component(
                Bullet::Create(
                    direction,
                    10.0,//128.0,
                    0.5
                ));
        
        waterEntity
            .borrow_mut()
            .add_component(
                        Renderer2D::New(
                            &self._display, 
                            self._waterSprite.clone(), 
                            false
                ));


        api.lock().unwrap().Audio.PlayAudio(
            &AudioSample::Create(
                String::from(WATER_SHOOT_SFX), 
                1.0,
                false,
                EAudioSpace::Is2D,
                ETargetTrack::Effect
            )
        );

        api.lock().unwrap().SceneManager.AddEntity(waterEntity);
    }
}

impl Component for LunaController
{
    fn start(&mut self, entity: &mut TEntity, api: Arc<Mutex<GameAPI>>)
    {
    }

    fn update(&mut self, entity: &mut TEntity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
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

        let damping;

        if movementVector.magnitude() > 0.001
        {
            damping = 1.0;
            self._state = EPlayerState::run;
            self._lastMovementVector = movementVector;
        }
        else
        {
            damping = 0.1;
            self._state = EPlayerState::idle;
        }

        let targetVector = Float3::scale_value(movementVector, self._movementSpeed);


        // move this logic into frame
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

        if self._velocity.y() < 0.0
        {
            self._direction = EDirection::Up;
        }
        else if self._velocity.y() > 0.0
        {
            self._direction = EDirection::Down;
        }


        let positionDelta = Float3::scale_value(self._velocity, frame.DeltaTime_Seconds);
        let futurePosition = entity.world_position + positionDelta;

        // Check for collision.
        if !api.lock().unwrap().Collision.IsThereSolidCollisionAt(entity.ID(), futurePosition)
        { 
            entity.world_position.add(Float3::scale_value(self._velocity, frame.DeltaTime_Seconds));
        }
        else
        {
            // Check and see if we can apply sliding.
            let x_comp = Float3::new(positionDelta.x(), 0.0, 0.0);
            if !api.lock().unwrap().Collision.IsThereSolidCollisionAt(
                entity.ID(),
                entity.world_position + x_comp)
            {
                entity.world_position.add(x_comp);
            }

            let y_comp = Float3::new(0.0, positionDelta.y(), 0.0);
            if !api.lock().unwrap().Collision.IsThereSolidCollisionAt(
                entity.ID(),
                entity.world_position + y_comp)
            { 
                entity.world_position.add(y_comp);
            }
        }


        self.animation_update(entity, self._state, self._direction);

        // Shoot water ball
        if frame.Input.IsKeyPressed(Space)
        {
            self.CreateWaterBall(entity, api, self._lastMovementVector.normalized());
        }
    }
}
