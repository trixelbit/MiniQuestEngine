use glium::glutin::surface::WindowSurface;
use glium::Display;

use crate::SceneBuilder::Scene;
use crate::GameEntity::Entity;

use std::borrow::Borrow;
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

    pub fn AddScene(&mut self, alias: &str, path: &str)
    {
        self._scenes.push(Scene::new(alias, path));
    }

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

    pub fn AddEntity(&mut self, newEntity: Rc<RefCell<Entity>>)
    {
        self.Entities.push(newEntity);
    }

    pub fn DestroyEntity(&mut self, entityName: &str)
    {
        let mut index : Option<usize> = None;
        for i in 0..self.Entities.len()
        {
            if entityName ==  &self.Entities[0].borrow_mut().Name
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


