use crate::Engine::Components::RenderComponents::Sprite;
use crate::Engine::Math::Float3;
use crate::Engine::Tile::Tile;
use crate::Entities::{EEntity, EEntityType};

pub struct Asset
{
    _type: EEntityType,
    _texture: Sprite,
}

impl Asset
{
    /*
    pub fn Create(&self, cursorPosition: Float3) -> EEntity
    {

        match self._type
        {
            EEntityType::Boxer => {}

            // how are we going to deal with field options?
            EEntityType::Tiles =>
                EEntity::Tiles(
                    Tile::Create(
                        "Tile",
                        cursorPosition,

                    ))

            // how are we going to deal with field options?
            EEntityType::AudioPlayer => {}
        }
    }
    */
}

