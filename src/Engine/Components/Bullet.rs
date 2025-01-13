use crate::Engine::GameEntity::Entity;
use crate::Engine::Components::Component;
use crate::Engine::Frame::GameFrame;
use crate::Engine::Math::Float3;
use crate::Engine::GameAPI::GameAPI;


use cgmath::num_traits::ToPrimitive;
use rxrust::prelude::*;
use std::sync::{RwLock, Mutex, Arc};
use std::rc::Rc;


pub struct Bullet
{
    pub Direction: Float3,
    pub Speed: f32,
    pub LifeTime_Seconds: f32
    // Potentially added enum of behaviors like destroy on contact.
}

impl Bullet
{
    pub fn Create(direction: Float3, speed: f32, lifeTime_Seconds: f32) -> Rc<RwLock<Self>>
    {
        Rc::new(
            RwLock::new(
                Self
                {
                    Direction: direction,
                    Speed: speed,
                    LifeTime_Seconds: 1.0// lifeTime_Seconds
                }
            )
        )
    }
}

impl Component for Bullet
{
    fn start(&mut self, 
        entity: &mut Entity, 
        api: Arc<Mutex<GameAPI>>)
    {
        // sync runner
        // let mut scheduler = FuturesLocalSchedulerPool::new();
    
        // asynch threaded scheduling
        let scheduler = FuturesThreadPoolScheduler::new();

        let apiCopy: Arc<Mutex<GameAPI>> = api.clone();
        let id = entity.ID().clone();
        
        rxrust::observable::timer(
            false, 
            // TODO: Decompose this  f32 seconds value to u64 seconds values and u34 nano seconds
            Duration::new(self.LifeTime_Seconds.to_u64().unwrap(), 0),  
            scheduler.unwrap())
            .subscribe(
                move |_| 
                {
                    apiCopy.lock().unwrap().SceneManager.DestroyEntity(id);
                }
            );
    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        entity.world_position.add(
            Float3::scale_value(
                self.Direction, 
                self.Speed * frame.DeltaTime_Seconds
            ));
    }

    fn OnDestroy(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>)
    {
        println!("OnDestroy called for entity.");
    }
}





