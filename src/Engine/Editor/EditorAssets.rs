use crate::Boxer::Boxer;
use crate::Engine::Components::AudioSource::AudioPlayer;
use crate::Engine::Components::RenderComponents::Sprite;
use crate::Engine::Math::Float3;
use crate::Engine::Tile::Tile;
use crate::Entities::{EEntity, EEntityType};
use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;

use std::sync::Arc;


pub struct Asset
{
    _type: EEntityType,
    _texture: Arc<Sprite>,
    _display: Display<WindowSurface>
}

impl Asset
{
    pub fn Create(assetType: EEntityType, sprite: Arc<Sprite>, display: Display<WindowSurface>) -> Self
    {
        Self
        {
            _type: assetType,
            _texture: sprite,
            _display: display
        }
    }

    pub fn GenerateEntity(&self, cursorPosition: Float3) -> EEntity
    {
        match self._type
        {
            EEntityType::Boxer => 
            {
                EEntity::Boxer(
                    Boxer::Create(32.0, &self._display, cursorPosition)
                )
            },

            // how do we get tile dimensions?
            // how are we going to deal with field options?
            EEntityType::Tiles =>
            {
                EEntity::Tiles(
                    Tile::Create(
                        "Tile",
                        cursorPosition,
                        self._texture.clone(),
                        false,
                        &self._display.clone(),
                        None

                    )
                )
            },

            // how are we going to deal with field options?
            EEntityType::AudioPlayer => 
            {
                EEntity::AudioPlayer
                (
                    AudioPlayer::Create(
                            "Audio",
                            cursorPosition, 
                            String::from(""), 
                            0.5, 
                            false, 
                            crate::Engine::Audio::sample::EAudioSpace::Is3D, 
                            crate::Engine::Audio::sample::ETargetTrack::Music)
                )
            }
        }
    }

}

