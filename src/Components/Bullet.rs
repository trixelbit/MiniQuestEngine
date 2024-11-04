use crate::GameEntity::Entity;
use crate::Components::Component;
use crate::Frame::GameFrame;
use crate::Math::Float3;
use crate::GameAPI::GameAPI;


use cgmath::num_traits::ToPrimitive;
use rxrust::prelude::*;
use rxrust::prelude::timer::TimerObservable;
use std::sync::RwLock;
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
                    LifeTime_Seconds: lifeTime_Seconds
                }
            )
        )
    }
}

impl Component for Bullet
{
    fn start(&mut self, 
        entity: &mut Entity, 
        api: &mut GameAPI)
    {
        // sync runner
        // let mut scheduler = FuturesLocalSchedulerPool::new();
    
        // asynch threaded scheduling
        let scheduler = FuturesThreadPoolScheduler::new();
        
        rxrust::observable::timer(
            false, 
            // TODO: Decompose this  f32 seconds value to u64 seconds values and u34 nano seconds
            Duration::new(self.LifeTime_Seconds.to_u64().unwrap(), 0),  
            scheduler.unwrap())
            .subscribe
            (
                // figure out how to pass mutable death function to thread
                |_| println!("Died") 
            );

    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: &mut GameAPI)
    {
        entity.world_position.add(
            Float3::scale_value(
                self.Direction, 
                self.Speed * frame.DeltaTime_Seconds
            ));
    }
}





