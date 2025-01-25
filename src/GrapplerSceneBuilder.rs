use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use crate::Engine::Audio::sample::{EAudioSpace, ETargetTrack};
use crate::Engine::Collision::collider::{ECollisionTag, ECollisionType};
use crate::Engine::Components::AudioSource::AudioPlayer;
use crate::Engine::Components::{AudioSource, Collider};
use crate::Engine::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Engine::GameEntity::TEntity;
use crate::Engine::Math::Float3;
use crate::Engine::SceneBuilder::{Scene, TSceneBuilder};
use crate::{LunaController, Boxer};
use crate::Engine::Tile::Tile;
use crate::Entities::Entities;

pub struct GCSBSceneBuilder
{
}

impl TSceneBuilder for GCSBSceneBuilder
{
    fn LoadScene(name: String, rawScene: String, display: &Display<WindowSurface>)
        -> Entities
    {
        println!("Loaded Scene: {}", name);

        let lines = rawScene.lines().filter( |x| !x.contains("*"));

        let mut entities = Entities::Create();

        lines.for_each( |x| Self::ParseEntity(x, &mut entities, display));

        entities
    }
}

impl GCSBSceneBuilder
{
    pub fn create() -> Self
    {
        Self{}
    }

    fn ParseEntity(entry: &str, entities: &mut Entities, display: &Display<WindowSurface>)
    {
        let mut tokens : Vec<String> = Vec::new();
        entry
            .split(crate::Engine::SceneBuilder::PROPERTY_SEPARATOR)
            .for_each(|x| tokens.push(String::from(x)));

        let objectType = &tokens.first().unwrap();

        match objectType.as_str()
        {
            "Player" => Self::BuildPlayer(tokens, entities, display),
            "Tile" => Self::BuildTile(tokens, entities, display),
            "Audio" => Self::BuildAudioSource(tokens, entities, display),
            _ => {}
        };
    }

    /// constructs a player object
    ///
    /// Entry Structure:
    ///     1 - name
    ///     2 - position
    fn BuildPlayer(data: Vec<String>, entities: &mut Entities, display: &Display<WindowSurface>)
    {
        let name = data[1].as_str();
        let position = Float3::FromString(data[2].as_str());

        let mut isLit = false;
        
        if data.len() >= 4
        {
            isLit = data[3].as_str().parse().unwrap();
        }

        let player = Boxer::Boxer::Create(
            32.0,
            display,
            position);

        entities.Boxer.push(player);
    }

    /// Builds a static tile object
    ///
    /// Entry Structure:
    ///     1 - name
    ///     2 - position
    ///     3 - asset path
    ///     4 - is a collider
    ///     5 - tag
    fn BuildTile(data: Vec<String>, entities: &mut Entities,  display: &Display<WindowSurface>)
    {
        // 1 - name
        let name = data[1].as_str();

        // 2 - position
        let position = Float3::FromString(data[2].as_str());

        // 3 - asset path
        let assetPath = data[3].as_str();

        // 4 - is a collider - TODO
        let mut hasCollider = false;

        if data.len() >= 5
        {
            let o: Result<bool, _> = data[4].as_str().parse();

            if(o.is_ok())
            {
                hasCollider = data[4].as_str().parse().unwrap();

            }
        }

        // 5 - is lit
        let mut isLit = false;
        if data.len() >= 6
        {
            isLit = data[5].as_str().parse().unwrap();
        }

        let mut collider = None;
        if hasCollider
        {
            collider = Some(
                Collider::Collider::Create(
                    display.clone(),
                    position,
                    Float3::new(32.0, 32.0, 1.0),
                    ECollisionType::Solid,
                    ECollisionTag::None
                )
            );
        }
        else
        {
            collider = None;
        }

        let tile =
            Tile::Create(
                name,
                position,
                Sprite::new(
                    assetPath,
                    &display,
                    1,
                    (1,1),
                    0.001),
                isLit,
                display,
                collider
            );

        entities.Tiles.push(tile);
    }

    /// Constructs an audio source object.
    ///
    /// Entry Structure:
    ///     1 - name
    ///     2 - position
    ///     3 - path
    ///     4 - volume
    ///     5 - space(2D/ 3D)
    ///     6 - track(music/ sfx)
    ///
    fn BuildAudioSource(data: Vec<String>, entities: &mut Entities, display: &Display<WindowSurface>)
    {
        let name = data[1].as_str();
        let position = Float3::FromString(data[2].as_str());

        let assetPath = String::from(data[3].as_str());

        let volume : f32 = data[4].as_str().trim().parse().unwrap();

        // TODO: implement remaining properties
        let audioSource =
            AudioPlayer::Create(
                name,
                position,
                assetPath,
                volume,
                true,
                EAudioSpace::Is2D,
                ETargetTrack::Music
            );

        entities.AudioPlayers.push(audioSource);
    }
}
