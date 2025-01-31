use std::fmt::{Debug, Formatter};
use glium::Frame;
use uuid::Uuid;
use crate::Engine::Frame::GameFrame;
use crate::Engine::Math::Float3;
use crate::Engine::GameAPI::GameAPI;

pub struct EntityHeader
{ 
    pub WorldPosition: Float3,
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
            WorldPosition: position,
            scale: Float3::one(),
    
            _id: Uuid::new_v4(),
            _hasStartBeenCalled: false
        }
    }

    pub fn HasStartBeenCalled(&self) -> bool
    {
        self._hasStartBeenCalled
    }

    /// Returns copy of current ID
    pub fn ID(&self) -> Uuid
    {
        self._id.clone()
    }
}

pub trait TEntity: Debug + Clone + Copy
{
    fn HasStartBeenCalled(&self) -> bool;

    fn ID(&self) -> Uuid;

    unsafe fn Start(
        &mut self, 
        api: *mut GameAPI);

    unsafe fn Update(
        &mut self,
        frame: &GameFrame,
        api: *mut GameAPI);

    unsafe fn OnDestroy(
        &mut self,
        api: *mut GameAPI);

    fn Render(&mut self, frame: &GameFrame, target: &mut Frame);

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Entity")
    }
}





