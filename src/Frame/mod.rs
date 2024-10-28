use cgmath::{Matrix4, SquareMatrix};
use chrono::{TimeDelta};
use crate::Frame::Input::*;

pub mod Input;

/// Frame information that should be passed to game entities.
pub struct GameFrame
{
    pub Input : InputState,
    pub TimeSinceGameStart: TimeDelta,
    pub DeltaTime: TimeDelta,
    pub CameraView: Matrix4<f32>,
    pub CameraPerspective: Matrix4<f32>,
}


impl GameFrame
{
    pub fn new(input: InputState, timeSinceGameStart: TimeDelta, deltaTime: TimeDelta, matrix: Matrix4<f32>, scale: Matrix4<f32>) -> Self
    {
        Self
        {
            Input: input,
            TimeSinceGameStart: timeSinceGameStart,
            DeltaTime: deltaTime,
            CameraView: matrix,
            CameraPerspective: scale
        }
    }
}
