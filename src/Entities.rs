use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use glium::Frame;
use uuid::Uuid;
use crate::Boxer::Boxer;
use crate::Engine::Components::AudioSource::AudioPlayer;
use crate::Engine::Components::Camera::Camera;
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::TEntity;
use crate::Engine::Math::Float3;
use crate::Engine::Tile::Tile;


#[derive(Debug)]
pub enum EEntityType
{
    Boxer,
    Tiles,
    AudioPlayer
}

#[derive(Debug)]
pub enum EEntity
{
    Boxer(Boxer),
    Tiles(Tile),
    AudioPlayer(AudioPlayer)
}

/// Collection of all entities that can exist in application
pub struct Entities
{
    pub Camera : Camera,
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
            Camera: Camera::New(30.0, Float3::new(0.0, 1.0, 0.0)),
            Boxer: Vec::new(),
            Tiles: Vec::new(),
            AudioSources: Vec::new(),
            _deadEntities: Vec::new()
        }
    }
    pub fn Start(api: &mut GameAPI)
    {
        unsafe
        {
            let a: *mut GameAPI  = api;
            let mut ent = &mut api.SceneManager.Entities;


            for x in ent.Boxer.iter_mut()
            {
                x.Start(a);
            }

            for x in ent.Tiles.iter_mut()
            {
                x.Start(a);
            }

            for x in ent.AudioSources.iter_mut()
            {
                x.Start(a);
            }
        }
    }

    pub fn Update(frame: &GameFrame, api: &mut GameAPI, target: &mut Frame)
    {

        unsafe
        {
            let a: *mut GameAPI  = api;
            let mut ent = &mut api.SceneManager.Entities;

            ent.Camera.Update(frame, a);
            ent.Camera.Render(frame, target);


            for x in ent.Tiles.iter_mut()
            {
                x.Update(frame, a);
                x.Render(frame, target);
            }

            for x in ent.AudioSources.iter_mut()
            {
                x.Update(frame, a);
                x.Render(frame, target);
            }

            for x in ent.Boxer.iter_mut()
            {
                x.Update(frame, a);
                x.Render(frame, target);
            }
        }
    }

    pub fn MarkEntityDead(&mut self, id: &Uuid)
    {
        if self._deadEntities.contains(id)
        {
            return;
        }

        self._deadEntities.push(id.clone());
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

    /// Adds a new Entity to active scene.
    pub fn AddEntity(&mut self, newEntity: EEntity)
    {
        match newEntity
        {
            EEntity::Boxer(e) => self.Boxer.push(e),
            EEntity::Tiles(e) => self.Tiles.push(e),
            EEntity::AudioPlayer(e) => self.AudioSources.push(e),
            e => !panic!("Unsupported entity type {:?}", e)
        }
    }
}

impl Debug for Entities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        todo!()
    }
}




// Simplifieis Object Strucutre

// Serse JSON Seri/deseri

// Play / Edit
//   Modify Scene Structure to bi-directional
