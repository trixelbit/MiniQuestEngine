mod EditorAssets;

use glium::Frame;
use uuid::Uuid;
use crate::Engine::Components::Camera::Camera;
use crate::Engine::Frame::GameFrame;
use crate::Entities::Entities;


/// Module responsible for the modification of scene files.
pub struct Editor
{
    _mode: EEditorMode,

    /// Used not only to display but center position is where objects are placed
    _camera: Camera,

    /// Objects that are selected
    _selectedIDs: Vec<Uuid>,

    /// Entities loaded from level file and modified in editor loop
    _entities: Entities
}

impl Editor
{
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

    pub fn Update(&mut self, frame: &GameFrame, target: &mut Frame)
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
