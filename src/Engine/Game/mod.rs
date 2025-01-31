#![allow(nonstandard_style)]

use chrono::Local;
use winit::event::{ElementState, MouseScrollDelta, TouchPhase};
use winit::event::KeyEvent;
use winit::event::MouseButton;

use crate::Engine::Components::*;
use crate::Engine::Editor::CozyEditor;
use crate::Engine::Frame::Input::Input;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::GameEntity::TEntity;
use crate::Engine::Player::CozyPlayer;
use crate::Engine::SceneBuilder::SceneBuilderFunction;
use crate::Entities::Entities;

pub enum EEngineMode
{
    Play,
    Editor
}

/// The Game Application that is running currently.
pub struct CozyEngine
{
    pub API: GameAPI,

    _editor: CozyEditor,

    _player: CozyPlayer,

    _mode: EEngineMode
}

impl CozyEngine
{
    /// Constructs game and performs any tasks before actual application window opens.
    pub fn New(sceneBuilderMethod: SceneBuilderFunction) -> Self
    {
        Self
        {
            API: GameAPI::Create(sceneBuilderMethod),
            _editor: CozyEditor::Create(),
            _player: CozyPlayer::Create(),
            _mode: EEngineMode::Play
        }
    }

    pub fn EnterPlayMode(&mut self)
    {
        // Reset GameAPI

        self.API.SceneManager.Entities.CopyFrom(&mut self._editor.Entities);

        self._mode = EEngineMode::Play;

        self._player.Start();
    }

    pub fn EnterEditorMode(&mut self)
    {
        // Reset GameAPI

        self.API.SceneManager.Entities.CopyFrom(&mut self._editor.Entities);

        self._mode = EEngineMode::Editor;

        self._player.Start();
    }


    /// Begins the game loop.
    pub fn Run(&mut self)
    {
        let timeStart = Local::now();

        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("event loop building");

        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new()
                .with_title("Mini Quest Engine Test")
                .with_inner_size(800, 600)
                .build(&event_loop);

        // Adds all levels that should be available for loading.
        self.API.SceneManager.AddScene("Level1", "Scenes/test.lvl");
        
        // Build starting scene.
        self.API.SceneManager.LoadScene("Level1", &display);

        unsafe
        {
            Entities::Start(&mut self.API);
        }

        // Enter frame loop
        let mut input = Input::New();
        let mut dateTimeLastFrame = Local::now();

        // TODO: Break this closure up into static functions
        event_loop.run( |event, window_target|
        {
            match event
            {
                winit::event::Event::WindowEvent { event, .. } => match event
                {
                    winit::event::WindowEvent::KeyboardInput {event, ..} 
                        => Self::KeyBoardInput(&mut input, event),

                    winit::event::WindowEvent::MouseInput {state, button, ..} 
                        => Self::MouseInput(&mut input, state, button),

                    winit::event::WindowEvent::MouseWheel {phase, delta, ..} 
                        => Self::MouseWheel(&mut input, phase, delta),

                    winit::event::WindowEvent::CursorMoved {position,..} =>
                    {
                        let _ = &input.SetMousePosition((position.x, position.y));
                    },

                    winit::event::WindowEvent::CloseRequested =>
                    {
                        window_target.exit();
                    },


                    // We now need to render everything in response to a RedrawRequested event due to the animation
                    winit::event::WindowEvent::RedrawRequested =>
                    {
                        match self._mode
                        {
                            EEngineMode::Play => self._player.Update(
                                &display,
                                &mut self.API,
                                &mut input,
                                timeStart,
                                &mut dateTimeLastFrame
                            ),

                            EEngineMode::Editor => self._editor.Update(
                                &display,
                                &mut self.API,
                                &mut input,
                                timeStart,
                                &mut dateTimeLastFrame
                            )
                        }
                    },

                    // Because glium doesn't know about windows we need to resize the display
                    // when the window's size has changed.
                    winit::event::WindowEvent::Resized(window_size) =>
                        {
                            display.resize(window_size.into());
                        },

                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user Input you could remove this handler.
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                },
                _ => (),
            }
        })
            .unwrap();
    }


    pub fn KeyBoardInput(input: &mut Input, event: KeyEvent)
    {
        match event.state
        {
            ElementState::Pressed =>
            {
                &input.Key_Pressed(event.physical_key)
            },

            ElementState::Released =>
            {
                &input.Key_Released(event.physical_key)
            }
        };
    }

    pub fn MouseInput(input: &mut Input, state: ElementState, button: MouseButton)
    {
        match state
        {
            ElementState::Pressed =>
                {
                    let _ = &input.Mouse_Pressed(button);
                }
            ElementState::Released =>
                {
                    let _ = &input.Mouse_Release(button);
                }
        }
    }

    pub fn MouseWheel(input: &mut Input, phase: TouchPhase, delta: MouseScrollDelta )
    {
        match delta
        {
            MouseScrollDelta::LineDelta(x, y) =>
            {
                input.SetMouseWheelLineOffset((x,y));
            },

            MouseScrollDelta::PixelDelta(value) =>
            {
                input.SetMouseWheelPixelDelta( (value.x, value.y));
            }
        };

        match phase
        {
            TouchPhase::Started => {}
            TouchPhase::Moved => {}
            TouchPhase::Ended =>
                {
                    input.SetMouseWheelPixelDelta((0.0, 0.0));
                    input.SetMouseWheelLineOffset((0.0, 0.0));
                },
            TouchPhase::Cancelled =>
                {
                    input.SetMouseWheelPixelDelta((0.0, 0.0));
                    input.SetMouseWheelLineOffset((0.0, 0.0));
                },
        };
    }
}


