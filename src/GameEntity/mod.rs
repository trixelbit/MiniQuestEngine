use std::rc::Rc;
use std::sync::{RwLock};
use winit::keyboard::KeyCode::*;
use crate::Frame::GameFrame;
use crate::Math::Float3;
use crate::Components::{Component};
use crate::GameAPI::GameAPI;

pub struct Entity
{
    pub world_position: Float3,
    pub scale: Float3,
    pub Name: String,
    _components : Vec<Rc<RwLock<dyn Component>>>,
    _componentNames : Vec<String>
}
impl Entity
{
    pub fn new(name: &str, position: Float3) -> Self
    {
        Entity
        {
            Name: String::from(name),
            world_position: position,
            scale: Float3::one(),
            _components: Vec::new(),
            _componentNames: Vec::new()
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
        api: &mut GameAPI)
    {
        let components = &self._components.clone();

        for component in components
        {
            let mut writeGuard = component.write().unwrap();
            writeGuard.start(self, api);
        }
    }

    pub fn update(
        &mut self, 
        frame: &GameFrame, 
        api: &mut GameAPI)
    {
        let components = &self._components.clone();

        for component in components
        {
            component.write().unwrap().update(self, &frame, api);
        }
    }
}
