/*
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use cgmath::num_traits::ToPrimitive;
use glium::Frame;
use rxrust::prelude::*;
use uuid::Uuid;

use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::{EntityHeader, TEntity};
use crate::Engine::Math::Float3;

pub struct Bullet
{
    pub Header: EntityHeader,
    pub Direction: Float3,
    pub Speed: f32,
    pub LifeTime_Seconds: f32
    // Potentially added enum of behaviors like destroy on contact.
}

impl Bullet
{
    pub fn Create(position: Float3, direction: Float3, speed: f32, lifeTime_Seconds: f32) -> Self
    {
        Self
        {
            Header: EntityHeader::Create("Bullet", position),
            Direction: direction,
            Speed: speed,
            LifeTime_Seconds: 1.0// lifeTime_Seconds
        }
    }
}

impl Debug for Bullet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Bullet")
    }
}

impl TEntity for Bullet
{
    fn HasStartBeenCalled(&self) -> bool
    {
        self.Header.HasStartBeenCalled()
    }

    fn ID(&self) -> Uuid
    {
        self.Header.ID()
    }

    fn Start(&mut self, api: Arc<Mutex<GameAPI>>)
    {
        // sync runner
        // let mut scheduler = FuturesLocalSchedulerPool::new();

        // asynch threaded scheduling
        let scheduler = FuturesThreadPoolScheduler::new();

        let apiCopy: Arc<Mutex<GameAPI>> = api.clone();
        let id = self.ID().clone();

        rxrust::observable::timer(
            false,
            // TODO: Decompose this  f32 seconds value to u64 seconds values and u34 nano seconds
            Duration::new(self.LifeTime_Seconds.to_u64().unwrap(), 0),
            scheduler.unwrap())
            .subscribe(
                move |_|
                    {
                        apiCopy.lock().unwrap().SceneManager.Entities.MarkEntityDead(&id);
                    }
            );
    }

    fn Update(&mut self, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        self.Header.WorldPosition.add(
            Float3::scale_value(
                self.Direction,
                self.Speed * frame.DeltaTime_Seconds
            ));
    }

    fn OnDestroy(&mut self, api: Arc<Mutex<GameAPI>>)
    {
        println!("OnDestroy called for entity.");
    }

    fn Render(&mut self, frame: &GameFrame, target: &mut Frame)
    {

    }
}





*/