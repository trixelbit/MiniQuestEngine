use chrono::{TimeDelta};
use crate::Frame::Input::*;

pub mod Input;

pub struct GameFrame
{
    pub Input : InputState,
    pub TimeSinceGameStart: TimeDelta,

    pub DeltaTime: TimeDelta
}

impl GameFrame
{
    pub fn new(input: InputState, timeSinceGameStart: TimeDelta, deltaTime: TimeDelta) -> Self
    {
        Self
        {
            Input: input,
            TimeSinceGameStart: timeSinceGameStart,
            DeltaTime: deltaTime,
        }
    }
}
