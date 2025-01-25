use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

use crate::Engine::GameAPI::GameAPI;
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameEntity::{EntityHeader, TEntity};
use crate::Engine::Audio::sample::*;
use crate::Engine::Math::Float3;

pub struct AudioPlayer
{
    pub Header: EntityHeader,
    _sample: AudioSample
}

impl AudioPlayer
{
    pub fn Create(
        name: &str,
        position: Float3,
        path: String, 
        volume: f32, 
        loops: bool, 
        space: EAudioSpace, 
        soundType: ETargetTrack) -> Self
    {
        Self
        {
            _sample: AudioSample::Create(path, volume, loops, space, soundType),
            Header: EntityHeader::Create(name, position)
        }
    }
}

impl TEntity for AudioPlayer
{
    fn HasStartBeenCalled(&self) -> bool
    {
        self.Header.HasStartBeenCalled()
    }

    fn ID(&self) -> Uuid
    {
        self.Header.ID()
    }

    fn start(&mut self, api: Arc<Mutex<GameAPI>>)
    {
        api.lock().unwrap().Audio.PlayAudio(&mut self._sample);
    }

    fn update(&mut self, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {

    }

    fn OnDestroy(&mut self, api: Arc<Mutex<GameAPI>>)
    {
    }
}
