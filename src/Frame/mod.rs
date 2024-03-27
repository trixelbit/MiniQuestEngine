use cgmath::{Matrix4, SquareMatrix};
use chrono::{TimeDelta};
use crate::Frame::Input::*;

pub mod Input;

pub struct GameFrame
{
    pub Input : InputState,
    pub TimeSinceGameStart: TimeDelta,
    pub DeltaTime: TimeDelta,
    pub CameraPosition: Matrix4<f32>,
    pub CameraScale: Matrix4<f32>
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
            CameraPosition: matrix,
            CameraScale: scale
        }
    }
}
