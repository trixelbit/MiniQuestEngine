use glium::glutin::surface::WindowSurface;
use glium::Display;
use uuid::Uuid;

use crate::Engine::SceneBuilder::{Scene, SceneBuilder};
use crate::Engine::GameEntity::TEntity;

use std::sync::{Arc, Mutex};
use crate::Engine::GameAPI::GameAPI;
use crate::Entities::{Entities, EEntities};


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
    _sceneBuilder : SceneBuilder
}


impl SceneManager
{
    pub fn Create(sceneBuilderMethod: SceneBuilder) -> Self
    {
        Self
        {
            Entities: Vec::new(),
            _idTable: Vec::new(),
            _deletionTable: Vec::new(),
            _scenes: Vec::new(),
            _sceneBuilder: sceneBuilderMethod
        }
    }

    /// Adds a scene that can be loaded by the game at runtime.
    ///
    /// alias - A name that to associate with the scene. 
    ///     Avoideds needing to know the scene path name.
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
            EEntities::Tile(e) => self.Entities.Tiles.push(e),
            EEntities::AudioPlayer(e) => self.Entities.AudioPlayers.push(e),
            e => {!panic!("Unsupported entity type {}", e);}
        }

        self._deletionTable.push(false);

        // this may be unsafe for use in separate threads
        self._idTable.push(newEntity.borrow_mut().ID())
    }

    /// Marks an entity for deletion next update cycle.
    pub fn DestroyEntity(&mut self, entityID: Uuid)
    {
        let mut index : Option<usize> = None;

        for i in 0..self._idTable.len()
        {
            if self._idTable[i].eq(&entityID)
            {
                index = Some(i);
                break;
            }
        }

        if index.is_some()
        {
            self._deletionTable[index.unwrap()] = true;
        }
    }

    /// Removes all entities that are marked for deletion.
    /// Also calls OnDestroy() on entities before deletion.
    /// Will be called before the start of the next frame.
    pub fn PruneDeadObject(&mut self, api: Arc<Mutex<GameAPI>>)
    {
        let mut deadIndicies = Vec::new();

        for i in 0..self._deletionTable.len()
        {
            if self._deletionTable[i]
            {
                deadIndicies.push(i);
            }
        }

        for i in self._deletionTable.len() - 1..0
        {
            self.Entities[deadIndicies[i]].borrow_mut().OnDestroy(api.clone());
            self.Entities.remove(deadIndicies[i]);
            self._deletionTable.remove(deadIndicies[i]);
            self._idTable.remove(deadIndicies[i]);
        }
    }
}


