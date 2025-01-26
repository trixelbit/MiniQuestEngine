use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use glium::Frame;
use uuid::Uuid;
use crate::Boxer::Boxer;
use crate::Engine::Components::AudioSource::AudioPlayer;
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::TEntity;
use crate::Engine::Tile::Tile;


#[derive(Debug)]
pub enum EEntities
{
    Boxer(Boxer),
    Tiles(Tile),
    AudioPlayer(AudioPlayer)
}

/// Collection of all entities that can exist in application
pub struct Entities
{
    pub Boxer: Vec<Boxer>,
    pub Tiles : Vec<Tile>,
    pub AudioSources: Vec<AudioPlayer>,

    _deadEntities: Vec<Uuid>,
}

impl Entities
{
    pub fn Create() -> Self
    {
        Self
        {
            Boxer: Vec::new(),
            Tiles: Vec::new(),
            AudioSources: Vec::new(),
            _deadEntities: Vec::new()
        }
    }

    pub fn MarkEntityDead(&mut self, id: Uuid)
    {
        if self._deadEntities.contains(&id)
        {
            return;
        }

        self._deadEntities.push(id);
    }

    pub fn PruneDeadEntities(&mut self)
    {
        if(self._deadEntities.len() == 0)
        {
            return;
        }

        let mut deadIndicies: Vec<usize> = Vec::new();

        for i in 0..self.Boxer.len()
        {
            if self._deadEntities.contains(&self.Boxer[i].Header.ID())
            {
                deadIndicies.push(i);
            }
        }

        deadIndicies.iter().for_each( |x|
            {
                self.Boxer.remove(*x);
            });

        deadIndicies.clear();


        for i in 0..self.Tiles.len()
        {
            if self._deadEntities.contains(&self.Tiles[i].Header.ID())
            {
                deadIndicies.push(i);
            }
        }

        deadIndicies.iter().for_each( |x|
            {
                self.Boxer.remove(*x);
            });
        deadIndicies.clear();


        for i in 0..self.AudioSources.len()
        {
            if self._deadEntities.contains(&self.AudioSources[i].Header.ID())
            {
                deadIndicies.push(i);
            }
        }

        deadIndicies.iter().for_each( |x|
            {
                self.Tiles.remove(*x);
            });
        deadIndicies.clear();
    }
}

impl Debug for Entities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        todo!()
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

    fn Start(&mut self, api: Arc<Mutex<GameAPI>>)
    {
        for mut x in self.Boxer
        {
            x.Start(api.clone());
        }

        for mut x in self.Tiles
        {
            x.Start(api.clone());
        }

        for mut x in self.AudioSources
        {
            x.Start(api.clone());
        }
    }

    fn Update(&mut self, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        for mut x in self.Boxer
        {
            x.Update(frame, api.clone());
        }

        for mut x in self.Tiles
        {
            x.Update(frame, api.clone());
        }

        for mut x in self.AudioSources
        {
            x.Update(frame, api.clone());
        }
    }

    fn OnDestroy(&mut self, api: Arc<Mutex<GameAPI>>)
    {
    }

    fn Render(&self, frame: &GameFrame, target: &mut Frame)
    {
        for x in self.Boxer
        {
            x.Render(frame, target);
        }

        for x in self.Tiles
        {
            x.Render(frame, target);
        }

        for x in self.AudioSources
        {
            x.Render(frame, target);
        }
    }
}





// Simplifieis Object Strucutre

// Serse JSON Seri/deseri

// Play / Edit
//   Modify Scene Structure to bi-directional
