#![allow(nonstandard_style)]
mod Engine;
mod GrapplerSceneBuilder;
mod PlayerController;

#[macro_use]
extern crate glium;

use crate::Engine::Game;
use crate::Engine::SceneBuilder::TSceneBuilder;


fn main()
{
    let mut game = Game::Game::New(
        GrapplerSceneBuilder::GCSBSceneBuilder::LoadScene);
    game.Run();
}


