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
    _scenes : Vec<Scene>
}


impl SceneManager
{
    pub fn Create() -> Self
    {
        Self
        {
            Entities: Vec::new(),
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

        self.Entities.append(
            &mut scene.unwrap().LoadScene(display)
        );
    }

    /// Adds a new Entity to active scene.
    pub fn AddEntity(&mut self, newEntity: Rc<RefCell<Entity>>)
    {
        self.Entities.push(newEntity);
    }

    /// Destroys an Entity with the matching ID
    pub fn DestroyEntity(&mut self, entityID: Uuid)
    {
        let mut index : Option<usize> = None;
        for i in 0..self.Entities.len()
        {
            if entityID == self.Entities[0].borrow_mut().ID()
            {
                index = Some(i);
                break;
            }
        }

        if index.is_some()
        {
            self.Entities.remove(index.unwrap());
        }
    }
}


