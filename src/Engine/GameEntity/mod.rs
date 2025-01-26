use std::fmt::{Debug, Formatter};
use std::sync::{Mutex, Arc};
use glium::Frame;
use uuid::Uuid;
use crate::Engine::Frame::GameFrame;
use crate::Engine::Math::Float3;
use crate::Engine::GameAPI::GameAPI;

pub struct EntityHeader
{ 
    pub world_position: Float3,
    pub scale: Float3,
    pub Name: String,
    _hasStartBeenCalled: bool,
    _id : Uuid,
}

impl EntityHeader
{ 
    pub fn Create(name: &str, position: Float3) -> Self
    {
        EntityHeader
        {
            Name: String::from(name),
            world_position: position,
            scale: Float3::one(),
    
            _id: Uuid::new_v4(),
            _hasStartBeenCalled: false
        }
    }

    pub fn HasStartBeenCalled(&self) -> bool
    {
        self._hasStartBeenCalled
    }

    pub fn ID(&self) -> Uuid
    {
        self._id.clone()
    }
}

pub trait TEntity : Debug
{
    fn HasStartBeenCalled(&self) -> bool;

    fn ID(&self) -> Uuid;

    fn Start(
        &mut self, 
        api: Arc<Mutex<GameAPI>>);

    fn Update(
        &mut self, 
        frame: &GameFrame, 
        api: Arc<Mutex<GameAPI>>);

    fn OnDestroy(
        &mut self, 
        api: Arc<Mutex<GameAPI>>);

    fn Render(&self, frame: &GameFrame, target: &mut Frame);

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Entity")
    }
}





