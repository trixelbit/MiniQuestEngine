mod EditorAssets;

use chrono::{DateTime, Local};
use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use uuid::Uuid;
use crate::Engine::Components::Camera::Camera;
use crate::Engine::Frame::Input::Input;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::Math::Float3;
use crate::Entities::Entities;


/// Module responsible for the modification of scene files.
pub struct CozyEditor
{
    /// Entities loaded from level file and modified in editor loop
    pub Entities: Entities,


    _mode: EEditorMode,

    /// Used not only to display but center position is where objects are placed
    _camera: Camera,

    /// Objects that are selected
    _selectedIDs: Vec<Uuid>,



    // Insert Mode Data
    _isSelectingAsset : bool,
    _assetCoord: Float3,



}

impl CozyEditor
{
    pub fn Create() -> Self
    {
        Self
        {

        }
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
        display: &Display<WindowSurface>,
        api: &mut GameAPI,
        input: &mut Input,
        timeStart: DateTime<Local>,
        dateTimeLastFrame: &mut DateTime<Local>
    )
    {

    }

    fn TransitionState(&mut self, targetState: EEditorMode)
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
