use std::fs;

use glium::Display;
use glium::glutin::surface::WindowSurface;

use crate::Entities::Entities;

/// Function pointer type so a game specific builder with knowledge os all
/// types can build its entities.
pub type SceneBuilderFunction
= fn(name: String, rawScene: String, display: &Display<WindowSurface>) -> Entities;

pub const PROPERTY_SEPARATOR: &str = "|";

/// This maybe exposed to application level since this will contain 
/// game specific constructions methods for deserializing scene data.
pub struct Scene
{
    // The alias name of the scene
    _name: String,

    // Name of scene file path.
    _fileName: String,
    _rawSceneContents: String,
    _sceneBuilder: SceneBuilderFunction
}

/// Do we load this directly to game state? 
impl Scene
{
    pub fn Name(&self) -> String
    {
        self._name.clone()
    }

    pub fn Create(alias: &str, scenePath : &str,
                  sceneBuilder: SceneBuilderFunction
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
            _fileName: String::from(scenePath),
            _sceneBuilder: sceneBuilder
        }
    }

    /// Constructs a list of entities from a scene. 
    pub fn LoadScene(&self, display: &Display<WindowSurface>) -> Entities
    {
        (self._sceneBuilder)
            (
                String::from(&self._name),
                String::from(&self._rawSceneContents),
                &display.clone()
            )
    }

    pub fn SaveScene(&self, entities: &Entities)
    {

    }
}

/// A scene builder is responsible for the actual construction of a scene object.
/// As is stands, this allows each specific game to implement their own logic for
/// how a scene a deserialized.
pub trait TSceneBuilder
{
    fn LoadScene(name: String, rawScene: String, display: &Display<WindowSurface>) -> Entities;
}




