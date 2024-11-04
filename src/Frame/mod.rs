use cgmath::{num_traits::ToPrimitive, Matrix4, SquareMatrix};
use chrono::TimeDelta;
use crate::Frame::Input::*;

pub mod Input;

/// Frame information that should be passed to game entities.
pub struct GameFrame
{
    pub Input : InputState,
    pub TimeSinceGameStart: TimeDelta,
    pub DeltaTime: TimeDelta,
    pub DeltaTime_Seconds: f32,
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
            DeltaTime_Seconds: deltaTime.num_milliseconds().to_f32().unwrap() / 100.0,
            CameraView: matrix,
            CameraPerspective: scale
        }
    }
}
