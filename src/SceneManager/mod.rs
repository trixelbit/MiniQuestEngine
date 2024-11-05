use glium::glutin::surface::WindowSurface;
use glium::Display;
use uuid::Uuid;

use crate::SceneBuilder::Scene;
use crate::GameEntity::Entity;

use std::cell::RefCell;
use std::rc::Rc;


pub struct SceneManager
{
    pub Entities: Vec<Rc<RefCell<Entity>>>,
    _idTable: Vec<Uuid>,
    // All entitys marked for deletion (true)
    _deletionTable: Vec<bool>,

    _scenes : Vec<Scene>
}


impl SceneManager
{
    pub fn Create() -> Self
    {
        Self
        {
            Entities: Vec::new(),
            _idTable: Vec::new(),
            _deletionTable: Vec::new(),
            _scenes: Vec::new()
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
        self._scenes.push(Scene::new(alias, path));
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
    pub fn AddEntity(&mut self, newEntity: Rc<RefCell<Entity>>)
    {
        self.Entities.push(newEntity.clone());
        self._deletionTable.push(false);
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

    pub fn PruneDeadObject(&mut self)
    {
        let mut deadIndicies = Vec::new();

        for i in 0..self._deletionTable.len()
        {
            if self._deletionTable[i]
            {
                deadIndicies.push(i);
            }
        }

        deadIndicies
            .into_iter()
            .for_each( |x| 
                {
                    self.Entities.remove(x);
                    self._deletionTable.remove(x);
                    self._idTable.remove(x);
                });
    }

}


