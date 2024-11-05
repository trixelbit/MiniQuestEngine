use std::rc::Rc;
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::Mutex;

use crate::Components::Component;
use crate::GameAPI::GameAPI;
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use crate::Audio::{AudioSample, ETargetTrack,EAudioSpace};

pub struct AudioPlayer
{
    _sample: AudioSample
}

impl AudioPlayer
{
    pub fn Create(
        path: String, 
        volume: f32, 
        loops: bool, 
        space: EAudioSpace, 
        soundType: ETargetTrack) -> Rc<RwLock<Self>>
    {
        Rc::new(
            RwLock::new(
                Self
                {
                    _sample: AudioSample::Create(path, volume, loops, space, soundType)
                }
            )
        )
    }
}

impl Component for AudioPlayer
{
    fn start(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>)
    {
        api.lock().unwrap().Audio.PlayAudio(&mut self._sample);
    }

    fn update(&mut self, entity: &mut Entity,  frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {

    }
}
