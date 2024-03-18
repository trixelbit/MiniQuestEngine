use crate::Frame::Input::*;

pub mod Input;

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
