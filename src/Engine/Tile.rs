use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use uuid::Uuid;

use crate::Engine::Components::Collider::Collider;
use crate::Engine::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::{EntityHeader, TEntity};
use crate::Engine::Math::Float3;

pub struct Tile
{
    pub Header: EntityHeader,
    _renderer: Renderer2D,
    _collider: Option<Collider>
}

impl Tile
{
    pub fn Create(
        name: &str,
        position: Float3,
        sprite: Arc<Sprite>,
        isLit: bool,
        display: &Display<WindowSurface>,
        collider: Option<Collider>
    ) -> Self
    {
        Self
        {
            Header: EntityHeader::Create(name, position),
            _renderer: Renderer2D::New(display, sprite, isLit),
            _collider: collider
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Tile")
    }
}

impl TEntity for Tile
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
        self._renderer.Start(&self.Header, api);
        match &mut self._collider
        {
            Some(T) =>
                {
                    T.Start(&self.Header, api);
                },
            _ => {}
        }
    }

    unsafe fn Update(&mut self, frame: &GameFrame, api: *mut GameAPI)
    {
        match &mut self._collider
        {
            Some(T) =>
                {
                    T.Update(&self.Header, frame, api);
                },
            _ => {}
        }
    }

    unsafe fn OnDestroy(&mut self, api: *mut GameAPI)
    {
        match &mut self._collider
        {
            Some(T) =>
                {
                    T.OnDestroy(&self.Header, api);
                },
            _ => {}
        }
    }

    fn Render(&mut self, frame: &GameFrame, target: &mut Frame)
    {
        self._renderer.Render(&self.Header, frame, target);
    }
}