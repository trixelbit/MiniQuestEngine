use crate::SceneBuilder::Scene;
use crate::GameEntity::Entity;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;

pub struct SceneManager
{
    pub Entities: Vec<Rc<RefCell<Entity>>>,
    _scenes : Vec<Scene>
}


impl SceneManager
{
    pub fn CurrentScene(&self)
    {

    }

    pub fn Create() -> Self
    {

    }
}


