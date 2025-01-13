use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Mutex, Arc};
use std::vec::Vec;
use std::fs;    
use glium::Display;
use glium::glutin::surface::WindowSurface;

use crate::Engine::GameAPI::GameAPI;
use crate::Engine::Audio::sample::*;
use crate::Engine::Collision::collider::{ECollisionTag, ECollisionType};
use crate::Engine::GameEntity::Entity;
use crate::Engine::{Components::*};
use crate::Engine::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Engine::Math::*;
use crate::Engine::Components::AudioSource::AudioPlayer;

pub type SceneBuilder
= fn(name: String, rawScene: String, display: &Display<WindowSurface>) -> Vec<Rc<RefCell<Entity>>>;

pub const PROPERTY_SEPARATOR: &str = "|";

/// This maybe exposed to application level since this will contain 
/// game specific constructions methods for deserializing scene data.
pub struct Scene
{
    _name: String,
    _rawSceneContents: String,
    _sceneBuilder: SceneBuilder
}

/// Do we load this directly to game state? 
impl Scene
{
    pub fn Name(&self) -> String
    {
        self._name.clone()
    }

    pub fn new(alias: &str, scenePath : &str,
               sceneBuilder: SceneBuilder
    ) -> Self
    {
        // TODO: Add better error messages.
        let fileReadOption = fs::read_to_string(scenePath);

        if fileReadOption.is_err()
        {
            panic!("\nFailed to read file in path: {}\n", scenePath);
        }

        let contents = fileReadOption.unwrap();

        Scene
        {
            _rawSceneContents: contents,
            _name: String::from(alias),
            _sceneBuilder: sceneBuilder
        }
    }

    /// Constructs a list of entities from a scene. 
    pub fn LoadScene(&self, display: &Display<WindowSurface>) -> Vec<Rc<RefCell<Entity>>>
    {
        (self._sceneBuilder)(
            String::from(&self._name),
            String::from(&self._rawSceneContents),
            &display.clone())
    }
}

/// A scene builder is responsible for the actual construction of a scene object.
/// As is stands, this allows each specific game to implement their own logic for
/// how a scene a deserialized.
pub trait TSceneBuilder
{
    fn LoadScene(name: String, rawScene: String, display: &Display<WindowSurface>) -> Vec<Rc<RefCell<Entity>>>;
}


