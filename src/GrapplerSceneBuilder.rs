use std::cell::RefCell;
use std::rc::Rc;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use crate::Engine::Audio::sample::{EAudioSpace, ETargetTrack};
use crate::Engine::Collision::collider::{ECollisionTag, ECollisionType};
use crate::Engine::Components::AudioSource::AudioPlayer;
use crate::Engine::Components::Collider;
use crate::Engine::Components::RenderComponents::{Renderer2D, Sprite};
use crate::Engine::GameEntity::Entity;
use crate::Engine::Math::Float3;
use crate::Engine::SceneBuilder::{Scene, TSceneBuilder};
use crate::{LunaController, Grappler};

pub struct GCSBSceneBuilder
{
}

impl TSceneBuilder for GCSBSceneBuilder
{
    fn LoadScene(name: String, rawScene: String, display: &Display<WindowSurface>)
        -> Vec<Rc<RefCell<Entity>>>
    {
        println!("Loaded Scene: {}", name);

        let lines = rawScene.lines().filter( |x| !x.contains("*"));

        lines
            .map( |x| Self::ParseEntity(x, display))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
}

impl GCSBSceneBuilder
{
    pub fn create() -> Self
    {
        Self{}
    }

    fn ParseEntity(entry: &str, display: &Display<WindowSurface>) -> Option<Rc<RefCell<Entity>>>
    {
        let mut tokens : Vec<String> = Vec::new();
        entry
            .split(crate::Engine::SceneBuilder::PROPERTY_SEPARATOR)
            .for_each(|x| tokens.push(String::from(x)));

        let objectType = &tokens.first().unwrap();

        match objectType.as_str()
        {
            "Player" => Some(Self::BuildPlayer(tokens, display)),
            "Tile" => Some(Self::BuildTile(tokens, display)),
            "Audio" => Some(Self::BuildAudioSource(tokens, display)),
            _ => None
        }
    }

    /// constructs a player object
    ///
    /// Entry Structure:
    ///     1 - name
    ///     2 - position
    fn BuildPlayer(data: Vec<String>, display: &Display<WindowSurface>) -> Rc<RefCell<Entity>>
    {
        let name = data[1].as_str();
        let position = Float3::FromString(data[2].as_str());

        let player = Rc::new(RefCell::new(Entity::Create(name, position)));

        let renderComponent =
            Renderer2D::New(&display,
                            Sprite::new(
                                "Assets/run_down.png",
                                &display,
                                4,
                                (2,2),
                                0.001),
            );

        let movementComponent = Grappler::GrapplerController::new(16.0f32, &display);
        //LunaController::LunaController::new(16.0f32, &display);

        let mut playerMut = player.borrow_mut();
        playerMut.add_component(movementComponent);
        playerMut.add_component(renderComponent);
        playerMut.add_component(
            Collider::Collider::Create(
                display.clone(),
                position,
                Float3::new(32.0, 32.0, 32.0),
                ECollisionType::Solid,
                ECollisionTag::None
            ));

        drop(playerMut);

        player
    }

    /// Builds a static tile object
    ///
    /// Entry Structure:
    ///     1 - name
    ///     2 - position
    ///     3 - asset path
    ///     4 - is a collider
    ///     5 - tag
    fn BuildTile(data: Vec<String>, display: &Display<WindowSurface>) -> Rc<RefCell<Entity>>
    {
        // 1 - name
        let name = data[1].as_str();

        // 2 - position
        let position = Float3::FromString(data[2].as_str());

        // 3 - asset path
        let assetPath = data[3].as_str();

        // 4 - is a collider - TODO
        let mut collider = false;

        if data.len() >= 5
        {
            collider = data[4].as_str().parse().unwrap();
        }


        let tile = Rc::new(RefCell::new(Entity::Create(name, position)));

        let renderComponent =
            Renderer2D::New(&display,
                            Sprite::new(
                                assetPath,
                                &display,
                                1,
                                (1,1),
                                0.001),
            );

        let mut tileMut = tile.borrow_mut();
        tileMut.add_component(renderComponent);

        if collider
        {
            tileMut.add_component(
                Collider::Collider::Create(
                    display.clone(),
                    position,
                    Float3::new(32.0, 32.0, 32.0),
                    ECollisionType::Solid,
                    ECollisionTag::None
                ));
        }

        drop(tileMut);

        tile
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
    fn BuildAudioSource(data: Vec<String>, display: &Display<WindowSurface>)
                        -> Rc<RefCell<Entity>>
    {
        let name = data[1].as_str();
        let position = Float3::FromString(data[2].as_str());

        let assetPath = String::from(data[3].as_str());

        let volume : f32 = data[4].as_str().trim().parse().unwrap();

        // TODO: implement remaining properties
        let audioSource = Rc::new(RefCell::new(Entity::Create(name, position)));

        let mut mutEnt = audioSource.borrow_mut();

        let audioSourceComp =
            AudioPlayer::Create(
                assetPath,
                volume,
                true,
                EAudioSpace::Is2D,
                ETargetTrack::Music
            );

        mutEnt.add_component(audioSourceComp);
        drop(mutEnt);

        audioSource
    }
}
