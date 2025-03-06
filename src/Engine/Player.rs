use std::rc::Rc;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;

use crate::Engine::Frame::GameFrame;
use crate::Engine::Frame::Input::Input;
use crate::Engine::GameAPI::GameAPI;
use crate::Entities::Entities;

use glium::Frame;

pub struct CozyPlayer
{
    _display: Display<WindowSurface>
}

impl CozyPlayer
{
    pub fn Create(display: Display<WindowSurface>) -> Self
    {
        Self
        {
            _display: display
        }
    }

    pub fn Start(&mut self)
    {

    }

    /// General Engine Update cycle.
    pub fn Update(
        &mut self,
        api: &mut GameAPI,
        frame: Rc<GameFrame>,
        target: &mut Frame
    )
    {
        Entities::Update(&frame, api, target);
    }
}
