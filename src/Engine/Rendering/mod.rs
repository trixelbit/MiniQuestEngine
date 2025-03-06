pub mod lightsources;

use core::panic;
use std::usize;

use glium::Display;
use glium::glutin::surface::WindowSurface;

use crate::Engine::Components::RenderComponents::Renderer2D;

use lightsources::*;

use super::Math::Float3;


pub struct RenderTicket
{
    _index: usize,
    _renderPipeline: *mut RenderPipeline
} 

impl RenderTicket
{
    pub fn New(index: usize, pipeline: *mut RenderPipeline) -> Self
    {
        Self
        {
            _index: index,
            _renderPipeline: pipeline 
        }
    }

    pub fn Index(&self) -> usize
    {
        self._index
    }

    // consider making this have a pointer reference to the RenderPipeline so getting render2d
    // componet can be handled from the ticket.

    pub unsafe fn Get(&self) -> *mut Renderer2D
    {
        (*self._renderPipeline).Get(self)
    }
}


/// Centralizes all Rendering components and lightsources so light information can be used in
/// shader uniforms.
pub struct RenderPipeline
{
    _display: Display<WindowSurface>,
    _renderers: Vec<Renderer2D>,
    _isReserved: Vec<bool>,

    _pointLights: Vec<PointLight>,
    _directionaLights: Vec<DirectionalLight>
}

impl RenderPipeline
{
    pub fn CreateTicket(&mut self, index: usize) -> RenderTicket
    {
        let a: *mut RenderPipeline = self;
        RenderTicket::New(index, a)
    }

    pub fn Reserve(&mut self) -> RenderTicket
    {
        for i in 0..self._isReserved.len()
        {
            if self._isReserved[i]
            {
                let a: *mut RenderPipeline = self;
                return RenderTicket::New(i, a);
            }
        }

        // TODO: implement empty constructors for renderers so we can bulk allocate on startup
        //
        //self._renderers.push(Renderer2D::New(self._display));
        //self._isReserved.push(true);

        panic!("No Available Renderers")
    }

    pub fn Return(&mut self, ticket: RenderTicket)
    {
        self._isReserved[ticket.Index()] = false;
    }

    pub unsafe fn Get(&mut self, ticket: &RenderTicket) -> *mut Renderer2D
    {
        &mut self._renderers[ticket.Index()]
    }

    pub fn NearestPointLight(&self, entityPosition: Float3) -> PointLight
    {
        panic!("not implemented");
    }
}

// TODO: maybe implement KD or QuadTree tree to query nearest point light sources near sprite 







