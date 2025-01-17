use std::sync::{Mutex, Arc};
use std::rc::Rc;
use std::sync::RwLock;
use uuid::Uuid;
use crate::Engine::Frame::GameFrame;
use crate::Engine::Math::Float3;
use crate::Engine::Components::Component;
use crate::Engine::GameAPI::GameAPI;

pub struct Entity
{
    pub world_position: Float3,
    pub scale: Float3,
    pub Name: String,
    _hasStartBeenCalled: bool,
    _id : Uuid,
    _components : Vec<Rc<RwLock<dyn Component>>>,
    _componentNames : Vec<String>
}
impl Entity
{
    pub fn HasStartBeenCalled(&self) -> bool
    {
        self._hasStartBeenCalled
    }

    pub fn ID(&self) -> Uuid
    {
        self._id.clone()
    }

    pub fn Create(name: &str, position: Float3) -> Self
    {
        Entity
        {
            Name: String::from(name),
            world_position: position,
            scale: Float3::one(),
            _components: Vec::new(),
            _componentNames: Vec::new(),
            _id: Uuid::new_v4(),
            _hasStartBeenCalled: false
        }
    }

    pub fn add_component(&mut self, component: Rc<RwLock<dyn Component>>)
    {
        self._components.push(component.clone());
        self._componentNames.push(component.read().unwrap().ComponentTypeName().clone());
    }

    pub fn get_component<TComponent: Component + 'static>(&self, caller: Option<&dyn Component>)
                                                          -> Option<Rc<RwLock<TComponent>>>
    {
        for i in 0..self._components.len()
        {
            if caller.is_some() && self._componentNames[i] == caller?.ComponentTypeName()
            {
                continue;
            }

            let component = self._components[i].clone();

            let target = std::any::type_name::<TComponent>().to_string();
            let readComp = component.read().unwrap();
            let name = readComp.ComponentTypeName().clone();

            if name == target
            {
                return component.try_read()
                    .ok()?
                    .downcast_ref::<TComponent>()
                    .map(|_| Rc::clone(&component))
                    .map(|rc| unsafe { Rc::from_raw(Rc::into_raw(rc) as *const RwLock<TComponent>) })
            }
        }

        return None;
    }

    pub fn start(
        &mut self, 
        api: Arc<Mutex<GameAPI>>)
    {
        if self._hasStartBeenCalled
        {
            panic!("Trying to call Start() after it has been already called in lifetime.");
        }

        let components = &self._components.clone();

        for component in components
        {
            let mut writeGuard = component.write().unwrap();
            writeGuard.start(self, api.clone());
        }

        self._hasStartBeenCalled = true;
    }

    pub fn update(
        &mut self, 
        frame: &GameFrame, 
        api: Arc<Mutex<GameAPI>>)
    {
        let components = &self._components.clone();

        for component in components
        {
            component.write().unwrap().update(self, &frame, api.clone());
        }
    }

    pub fn OnDestroy(
        &mut self, 
        api: Arc<Mutex<GameAPI>>)
    {
        let components = &self._components.clone();

        for component in components
        {
            component.write().unwrap().OnDestroy(self, api.clone());
        }
    }
}





