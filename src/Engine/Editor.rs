mod EditorAssets;

use chrono::{DateTime, Local};
use glium::Display;
use glium::glutin::surface::WindowSurface;
use std::sync::Arc;
use uuid::Uuid;
use crate::Engine::Components::Camera::Camera;
use crate::Engine::Frame::Input::Input;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::Math::Float3;
use crate::Entities::{EEntityType, Entities};

use EditorAssets::Asset;

use std::collections::hash_map::HashMap;
use std::fs;

use super::Components::RenderComponents::Sprite;


/// Defines a clone method.
/// However, it does not gaurente a perfec clone of all data in objects,
/// rather, it clones them to an appropriate state for when level begins.
///
/// Example: _hasStartBeenCalled in EntityHeader should always be false 
/// to ensure that the start method is called on copied entities on play.
pub trait TNewLevelClone
{
    fn LevelClone(&self) -> Self;
} 


/// Module responsible for the modification of scene files.
pub struct CozyEditor
{
    /// Entities loaded from level file and modified in editor loop
    pub Entities: Entities,

    _display: Display<WindowSurface>,


    _mode: EEditorMode,

    /// Used not only to display but center position is where objects are placed
    _camera: Camera,

    /// Objects that are selected
    _selectedIDs: Vec<Uuid>,

    // All sprites assets
    _spriteAssets: Vec<Asset>,

    // Insert Mode Data
    _isSelectingAsset : bool,
}

impl CozyEditor
{
    pub fn Create(display: &Display<WindowSurface>) -> Self
    {
        let mut newEditor = Self
        {
            Entities: Entities::Create(),
            _display: display.clone(),
            _mode: EEditorMode::Command,
            _camera: Camera::New(32.0, Float3::zero()),
            _selectedIDs: Vec::new(),
            _spriteAssets: Vec::new(),
            _isSelectingAsset: false,
        };

        // load and populate all assets in directory
        let imageNames: Vec<String> = fs::read_dir("Assets")
            .unwrap()
            .map(|x| x.unwrap().path())
            .filter(|x| x.is_file())
            .filter(|x| x.extension().unwrap() == "png")
            .map(|x| String::from(x.to_str().unwrap()))
            .collect();
        
        let audioNames: Vec<String> = fs::read_dir("Assets")
            .unwrap()
            .map(|x| x.unwrap().path())
            .filter(|x| x.is_file())
            .filter(|x| x.extension().unwrap() == "ogg")
            .map(|x| String::from(x.to_str().unwrap()))
            .collect();
    

        for name in imageNames.iter()
        {
            let asset = Asset::Create(
                EEntityType::Tiles, 
                Sprite::new_simple(name.as_str(), display),
                display.clone()
            );

            newEditor._spriteAssets.push(asset);
        }

        return newEditor;
    }

    // TODO: Create Method
    pub fn LoadLevel(&mut self)
    {

    }

    pub fn SaveLevel(& self)
    {

    }


    pub fn Start(&mut self)
    {

    }

    pub fn Update(
        &mut self,
        display: &Display<WindowSurface>,
        api: &mut GameAPI,
        input: &mut Input,
        timeStart: DateTime<Local>,
        dateTimeLastFrame: &mut DateTime<Local>
    )
    {
        match self._mode
        {
            EEditorMode::Command =>
                {

                }
            EEditorMode::Insert =>
                {
                    // Tiles => select any png
                    // Entities = Boxer, AudioSources, Camera

                    // Display assets pallet
                    //
                }
            EEditorMode::Selection =>
                {

                }
            EEditorMode::Transform =>
                {

                }
        }
    }

    fn TransitionState(&mut self, targetState: EEditorMode)
    {
        // listen for global commands

        match self._mode
        {
            EEditorMode::Command =>
                {

                }
            EEditorMode::Insert =>
                {
                    // Tiles => select any png
                    // Entities = Boxer, AudioSources, Camera

                    // Display assets pallet
                    //
                }
            EEditorMode::Selection =>
                {

                }
            EEditorMode::Transform =>
                {

                }
        }
    }

    fn CommandUpdate(&mut self)
    {

    }

    fn InsertUpdate(&mut self)
    {

    }
}

enum EEditorMode
{
    /// Allows moving the camera/cursor
    Command,

    /// Opens asset palette and placement of game objects
    Insert,

    /// Creates a bounding box, and everything in it can be selected
    Selection,

    /// Manipulates the position of an entity.
    Transform
}
