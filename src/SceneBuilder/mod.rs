use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

use crate::GameEntity::Entity;
use crate::Components::*;
use crate::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Math::*;

use std::fs;    
use std::sync::RwLock;

use glium::Display;
use glium::glutin::surface::WindowSurface;


pub struct Scene
{
    _name: String,
    _rawSceneContents : String
}

// Do we load this directly to game state? 
// 
impl Scene
{
    pub fn new(scenePath : &str) -> Self
    {
        // TODO: Add better error messages.
        let contents = fs::read_to_string(scenePath).unwrap();

        Scene
        {
            _rawSceneContents: contents,
            _name: String::from(scenePath)
        }
    }

    pub fn LoadScene(&self, display: &Display<WindowSurface> ) -> Vec<Rc<RefCell<Entity>>>
    {
        println!("Loaded Scene: {}", self._name);

        let lines = self._rawSceneContents.lines().filter( |x| !x.contains("*"));
       
        lines
            .map( |x| Scene::ParseEntity(x, display))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()

    }

    fn ParseEntity(entry: &str, display: &Display<WindowSurface>) -> Option<Rc<RefCell<Entity>>>
    {
        let mut tokens : Vec<String> = Vec::new();
        entry
            .split("|")
            .for_each(|x| tokens.push(String::from(x)));
       
        let objectType = &tokens.first().unwrap();

        match objectType.as_str()
        {
            "Player" => Some(Scene::BuildPlayer(tokens, display)),
            "Tile" => Some(Scene::BuildTile(tokens, display)),
            _ => None
        }
    }

    fn BuildPlayer(data: Vec<String>, display: &Display<WindowSurface>) -> Rc<RefCell<Entity>>
    {
        let position = Float3::FromString(data[1].as_str());

        let player = Rc::new(RefCell::new(Entity::new(position)));
        player.borrow_mut().scale = Float3::scale_value(Float3::one(), 5.0);

        let renderComponent =
            Rc::new(
                RwLock::new(
                    Renderer2D::New(&display,
                        Sprite::new(
                            "../Images/run_down.png",
                            &display,
                            4,
                            (2,2),
                            0.001))
        ));

        let movementComponent =
            Rc::new(
                RwLock::new(
                    PlayerController::PlayerController::new(8.0f32, &display)));
        let mut playerMut = player.borrow_mut();
        playerMut.add_component(movementComponent);
        playerMut.add_component(renderComponent);
        drop(playerMut);

        player
    }

    fn BuildTile(data: Vec<String>, display: &Display<WindowSurface>) -> Rc<RefCell<Entity>>
    {
        let position = 
        Float3::scale_value(
            Float3::FromString(data[1].as_str()), 
            5f32);

        let assetPath = data[2].as_str();

        let tile = Rc::new(RefCell::new(Entity::new(position)));
        tile.borrow_mut().scale = Float3::scale_value(Float3::one(), 5.0);

        let renderComponent =
            Rc::new(
                RwLock::new(
                    Renderer2D::New(&display,
                        Sprite::new(
                            assetPath,
                            &display,
                            1,
                            (1,1),
                            0.001))
        ));

        let mut tileMut = tile.borrow_mut();
        tileMut.add_component(renderComponent);
        drop(tileMut);

        tile
    }
}




