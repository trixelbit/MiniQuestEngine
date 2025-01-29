use glium::Display;
use glium::glutin::surface::WindowSurface;
use uuid::Uuid;

use crate::Engine::SceneBuilder::{Scene, SceneBuilderFunction};
use crate::Entities::{EEntity, Entities};

/// Contains the active scene state.
/// And manages the loading and saving of scenes
/// Also contains all scenes in the Scene folder and keeps them ready for loading.
pub struct SceneManager
{
    /// Entities that actively exist in the game currently.
    pub Entities: Entities,

    /// The IDs of all entities currently in game.
    /// Created to avoid borrow_mut reference from entities to get their IDs
    /// when marking them for deletion.
    _idTable: Vec<Uuid>,

    /// All entitys marked for deletion.
    _deletionSet: Vec<Uuid>,

    /// All scenes available for loading into active scene.
    _scenes : Vec<Scene>,

    /// Game Specific builder application
    _sceneBuilder : SceneBuilderFunction
}


impl SceneManager
{
    pub fn Create(sceneBuilderMethod: SceneBuilderFunction) -> Self
    {
        Self
        {
            Entities: Entities::Create(),
            _idTable: Vec::new(),
            _deletionSet: Vec::new(),
            _scenes: Vec::new(),
            _sceneBuilder: sceneBuilderMethod
        }
    }

    /// Adds a scene that can be loaded by the game at runtime.
    ///
    /// alias - A name that to associate with the scene. 
    ///     Avoids needing to know the scene path name.
    ///
    /// path - Path to scene file.
    pub fn AddScene(&mut self, alias: &str, path: &str)
    {
        self._scenes.push(Scene::Create(alias, path, self._sceneBuilder));
    }

    /// Loads a scene
    pub fn LoadScene(&mut self, alias: &str, display: &Display<WindowSurface>)
    {
        let mut sceneIter = self._scenes.iter();

        let scene = sceneIter
            .find(|x| x.Name() == alias);


        // TODO: this likely leaks, change signature to mutate single instance
        self.Entities = scene.unwrap().LoadScene(display);
    }
}


