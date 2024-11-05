#![allow(nonstandard_style)]
mod Math;
mod GameEntity;
mod Frame;
mod Components;
mod SceneManager;
mod SceneBuilder;
mod Game;
mod GameAPI;
mod MetaInfo;
mod GameState;
mod Audio;
mod Collision;

#[macro_use]
extern crate glium;


fn main()
{
    let mut game = Game::Game::New();
    game.Run();
}


