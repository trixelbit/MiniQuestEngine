use std::sync::{Arc, Mutex};

use glium::Display;
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

    fn start(&mut self, api: Arc<Mutex<GameAPI>>)
    {
    }

    fn update(&mut self, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
    }

    fn OnDestroy(&mut self, api: Arc<Mutex<GameAPI>>)
    {
    }
}