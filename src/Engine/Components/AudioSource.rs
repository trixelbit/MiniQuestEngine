use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::sync::Mutex;
use glium::Frame;
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

impl Debug for AudioPlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Audio Player")
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

    unsafe fn Start(&mut self, api: *mut GameAPI)
    {
        (*api).Audio.PlayAudio(&mut self._sample);
    }

    unsafe fn Update(&mut self, frame: &GameFrame, api: *mut GameAPI)
    {

    }

    unsafe fn OnDestroy(&mut self, api: *mut GameAPI)
    {
    }

    fn Render(&mut self, frame: &GameFrame, target: &mut Frame)
    {
    }
}
