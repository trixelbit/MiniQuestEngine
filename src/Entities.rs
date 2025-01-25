use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::Boxer::Boxer;
use crate::Engine::Components::AudioSource::AudioPlayer;
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::TEntity;
use crate::Engine::Tile::Tile;

/// Collection of all entities that can exist in application
pub struct Entities
{
    pub Boxer: Vec<Boxer>,
    pub Tiles : Vec<Tile>,
    pub AudioPlayers : Vec<AudioPlayer>
}

impl Entities
{
    pub fn Create() -> Self
    {
        Self
        {
            Boxer: Vec::new(),
            Tiles: Vec::new(),
            AudioPlayers: Vec::new(),
        }
    }
}

impl TEntity for Entities
{
    fn HasStartBeenCalled(&self) -> bool {
        todo!()
    }

    fn ID(&self) -> Uuid {
        todo!()
    }

    fn start(&mut self, api: Arc<Mutex<GameAPI>>)
    {
        self.Boxer.start(api);
    }

    fn update(&mut self, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        self.Boxer.update(frame, api);
    }

    fn OnDestroy(&mut self, api: Arc<Mutex<GameAPI>>)
    {
        todo!()
    }
}





// Simplifieis Object Strucutre

// Serse JSON Seri/deseri

// Play / Edit
//   Modify Scene Structure to bi-directional
