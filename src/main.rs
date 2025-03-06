#![allow(nonstandard_style)]
mod Engine;
mod GrapplerSceneBuilder;
mod LunaController;
pub mod Boxer;
mod Entities;

#[macro_use]
extern crate glium;

use winit::event_loop::EventLoop;

use crate::Engine::Game;
use crate::Engine::SceneBuilder::TSceneBuilder;


fn main()
{
    let event_loop : EventLoop<()>= 
        winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");

    let (window, display) =
        glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Mini Quest Engine Test")
            .with_inner_size(800, 600)
            .build(&event_loop);

    let mut game = 
        Game::CozyEngine::New(
            GrapplerSceneBuilder::GCSBSceneBuilder::LoadScene, 
            display, 
            window
    );

    game.Run(event_loop);
}


