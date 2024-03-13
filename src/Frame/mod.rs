use crate::Frame::input::{Input, InputState};

pub mod input;

pub struct GameFrame
{
    pub Input : InputState
}
impl GameFrame
{
    pub fn new(input: InputState) -> Self
    {
        Self
        {
            Input: input
        }
    }
}
