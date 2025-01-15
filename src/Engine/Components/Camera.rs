use std::sync::{Mutex, Arc};
use cgmath::{Matrix4, perspective, Vector3, ortho};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;
use winit::keyboard::KeyCode::SuperLeft;
use crate::Engine::Components::Component;
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameEntity::Entity;
use crate::Engine::Math::Float3;
use crate::Engine::GameAPI::GameAPI;

pub enum EProjectionType 
{
    Perspective,
    Orthographic
}

pub struct Camera
{
    pub EyePosition: Float3,
    pub FocalDirection: Float3,
    pub UpDirection : Float3,
    pub FieldOfView : f32,
    pub Projection : EProjectionType

}

impl Camera
{
    pub fn New(fov: f32) -> Self
    {
        Self
        {
            EyePosition: Float3::zero(),
            FocalDirection: Float3::new(0.0, 0.0, 1.0),
            UpDirection: Float3::up(),
            FieldOfView: fov,
            Projection: EProjectionType::Orthographic
        }
    }

    pub fn PerspectiveMatrix(&self) -> Matrix4<f32>
    {
        let bound_width = 0.3;
        let bound_height = 0.3;

        ortho(-bound_width, bound_width, -bound_height, bound_height, -10.0, 10.0)
        //perspective(cgmath::Deg(self.FieldOfView), 1.0, 0.1, 100.0)
    }

    pub fn ViewMatrix(&self) -> Matrix4<f32>
    {
        return Matrix4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.EyePosition.x(), -self.EyePosition.y(), -self.EyePosition.z(), 1.0]
        ]);
        return Matrix4::look_at_lh(
            self.EyePosition.ToCGPoint(),
            (self.EyePosition + self.FocalDirection).ToCGPoint(),
            self.UpDirection.ToCGVector()
        );

        let a =
        cgmath::Matrix4::look_at_lh(
            self.EyePosition.ToCGPoint(),
            self.FocalDirection.clone().add(self.EyePosition).ToCGPoint(),
            self.UpDirection.ToCGVector()
        );

        let b=
        perspective(cgmath::Deg(self.FieldOfView), 1.0, 0.1, 100.0);

        return a * b
    }

    pub fn ScaleMatrix(&self) -> Matrix4<f32>
    {
        let scale = self.EyePosition.z() / 100.0;
        return Matrix4::from_scale(scale);
        return Matrix4::from([
            [scale, 1.0, 1.0, 1.0],
            [1.0, scale, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0]
        ]);
    }


}

impl Component for Camera
{
    fn start(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>)
    {

    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        self.EyePosition = entity.world_position;
    }
}


pub struct CameraMouseController
{
    _initialWorldPosition : Float3,
    _initialMousePosition: Float3,
    _delta: Float3
}

impl CameraMouseController
{
    pub fn New() -> Self
    {
        Self
        {
            _initialMousePosition: Float3::zero(),
            _initialWorldPosition: Float3::zero(),
            _delta: Float3::zero()
        }
    }
}

impl Component for CameraMouseController
{
    fn start(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>)
    {

    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        let mousePosition = frame.Input.MousePosition();
        let vectorPosition = Float3::new(-mousePosition.0 as f32 / 300.0, mousePosition.1 as f32 / 300.0, 0.0);

        if frame.Input.IsMousePressed(MouseButton::Middle)
        {
            // cache initial position
            self._initialMousePosition = vectorPosition;
            self._initialWorldPosition = entity.world_position;
        }

        if frame.Input.IsMouseButtonDown(MouseButton::Middle)
        {
            // calculate delta position
            self._delta = vectorPosition - self._initialMousePosition;
            entity.world_position = self._initialWorldPosition + self._delta;
        }

        let scroll = frame.Input.MouseWheelLineDelta();
        let zAdd = -scroll.1 / 10.0;
        entity.world_position = entity.world_position + Float3::new(0.0, 0.0, zAdd as f32);
        
        //println!("Scroll Value: x:{} y:{}", scroll.0, scroll.1);
        //println!("Current EntityPosition: x{} y{} z{}", entity.world_position.x(), entity.world_position.y(), entity.world_position.z())
    }
}


