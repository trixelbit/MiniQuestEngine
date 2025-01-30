use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use cgmath::{Matrix4, ortho, perspective};
use glium::Frame;
use uuid::Uuid;
use winit::event::MouseButton;

use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::{EntityHeader, TEntity};
use crate::Engine::Math::Float3;

pub enum EProjectionType
{
    Perspective,
    Orthographic
}

pub struct Camera
{
    pub Header: EntityHeader,
    pub FocalDirection: Float3,
    pub UpDirection : Float3,
    pub FieldOfView : f32,
    pub Projection : EProjectionType,

    _editorController : EditorCameraController

}

impl Camera
{
    pub fn New(fov: f32, position: Float3) -> Self
    {
        Self
        {
            Header: EntityHeader::Create("Camera", position),
            FocalDirection: Float3::new(0.0, 0.0, 1.0),
            UpDirection: Float3::up(),
            FieldOfView: fov,
            Projection: EProjectionType::Perspective,
            _editorController : EditorCameraController::New()
        }
    }

    pub fn PerspectiveMatrix(&self) -> Matrix4<f32>
    {
        match self.Projection
        {
            EProjectionType::Orthographic =>
                {
                    let bound_width = 0.2;
                    let bound_height = 0.2;

                    ortho(-bound_width, bound_width, -bound_height, bound_height, -100.0, 100.0)

                }
            EProjectionType::Perspective =>
                {
                    perspective(cgmath::Deg(self.FieldOfView), 1.0, 0.1, 100.0)
                }
        }
    }

    pub fn ViewMatrix(&self) -> Matrix4<f32>
    {
        let x = self.Header.WorldPosition.x();
        let y = self.Header.WorldPosition.y();
        let z = self.Header.WorldPosition.z();

        return Matrix4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, -y, -z, 1.0]
        ]);
    }

    pub fn ScaleMatrix(&self) -> Matrix4<f32>
    {
        let scale = self.Header.WorldPosition.z() / 100.0;
        return Matrix4::from_scale(scale);
        return Matrix4::from([
            [scale, 1.0, 1.0, 1.0],
            [1.0, scale, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0]
        ]);
    }


}

impl Debug for Camera {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Camera")
    }
}

impl TEntity for Camera
{
    fn HasStartBeenCalled(&self) -> bool
    {
        self.Header.HasStartBeenCalled()
    }

    fn ID(&self) -> Uuid
    {
        self.Header.ID()
    }

    unsafe fn Start(&mut self, api: *mut GameAPI)
    {

    }

    unsafe fn Update(&mut self, frame: &GameFrame, api: *mut GameAPI)
    {
        self._editorController.Update(&mut self.Header, frame, api );
    }

    unsafe fn OnDestroy(&mut self, api: *mut GameAPI)
    {
    }

    fn Render(&mut self, frame: &GameFrame, target: &mut Frame)
    {

    }
}


/// Camera controller module that is used in edit mode.
pub struct EditorCameraController
{
    _initialWorldPosition : Float3,
    _initialMousePosition: Float3,
    _delta: Float3
}

impl EditorCameraController
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


    unsafe fn Update(&mut self, entity: &mut EntityHeader, frame: &GameFrame, api: *mut GameAPI)
    {
        //TODO If in editor mode, allow controls

        let mousePosition = frame.Input.MousePosition();
        let vectorPosition = Float3::new(-mousePosition.0 as f32 / 300.0, mousePosition.1 as f32 / 300.0, 0.0);

        if frame.Input.IsMousePressed(MouseButton::Middle)
        {
            // cache initial position
            self._initialMousePosition = vectorPosition;
            self._initialWorldPosition = entity.WorldPosition;
        }

        if frame.Input.IsMouseButtonDown(MouseButton::Middle)
        {
            // calculate delta position
            self._delta = vectorPosition - self._initialMousePosition;
            entity.WorldPosition = self._initialWorldPosition + self._delta;
        }

        let scroll = frame.Input.MouseWheelLineDelta();
        let zAdd = -scroll.1 / 10.0;
        entity.WorldPosition = entity.WorldPosition + Float3::new(0.0, 0.0, zAdd as f32);
    }
}


