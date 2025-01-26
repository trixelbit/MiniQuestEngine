use glium::Display;
use glium::glutin::surface::WindowSurface;
use uuid::Uuid;

use crate::Engine::SceneBuilder::{Scene, SceneBuilderFunction};
use crate::Entities::{EEntities, Entities};

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
        self._scenes.push(Scene::new(alias, path, self._sceneBuilder));
    }

    /// Loads a scene
    pub fn LoadScene(&mut self, alias: &str, display: &Display<WindowSurface>)
    {
        let mut sceneIter = self._scenes.iter();

        let scene = sceneIter
            .find(|x| x.Name() == alias);

        self.Entities.clear();

        let list = &mut scene.unwrap().LoadScene(display);

        for entity in list 
        {
            self.AddEntity(entity.clone());
        }
    }

    /// Adds a new Entity to active scene.
    pub fn AddEntity(&mut self, newEntity: EEntities)
    {
        match newEntity
        {
            EEntities::Boxer(e) => self.Entities.Boxer.push(e), 
            EEntities::Tiles(e) => self.Entities.Tiles.push(e),
            EEntities::AudioPlayer(e) => self.Entities.AudioSources.push(e),
            e => !panic!("Unsupported entity type {:?}", e)
        }
    }

    /// Marks an entity for deletion next update cycle.
    pub fn DestroyEntity(&mut self, entityID: Uuid)
    {
        self.Entities.MarkEntityDead(entityID);
    }
}


