#![allow(nonstandard_style)]

use core::fmt;
use std::time::SystemTime;
use std::rc::Rc;

use chrono::Local;
use winit::event::{ElementState, MouseScrollDelta, TouchPhase};
use winit::event::KeyEvent;
use winit::event::MouseButton;

use glium::{debug, Display, Surface};
use glium::glutin::surface::WindowSurface;
use winit::event_loop::EventLoop;

use crate::Engine::Editor::CozyEditor;
use crate::Engine::Frame::Input::Input;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::Player::CozyPlayer;
use crate::Engine::SceneBuilder::SceneBuilderFunction;
use crate::Entities::Entities;

use super::Frame::GameFrame;

pub enum EEngineMode
{
    Play,
    Editor
}

impl fmt::Display for EEngineMode
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            EEngineMode::Play => write!(f, "Play"),
            EEngineMode::Editor => write!(f, "Editor")
        }
    }
}

/// The Game Application that is running currently.
pub struct CozyEngine
{
    pub API: GameAPI,

    _editor: CozyEditor,

    _player: CozyPlayer,

    _mode: EEngineMode,


    _display: Display<WindowSurface>,

    _window: winit::window::Window,
}

impl CozyEngine
{
    
    /// Constructs game and performs any tasks before actual application window opens.
    pub fn New(
        sceneBuilderMethod: SceneBuilderFunction,
        display: Display<WindowSurface>,
        window: winit::window::Window

    ) -> Self
    {
        Self
        {
            API: GameAPI::Create(sceneBuilderMethod),
            _editor: CozyEditor::Create(&display),
            _player: CozyPlayer::Create(display.clone()),
            _mode: EEngineMode::Play,

            _display: display,
            _window: window
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
    pub fn Run(&mut self, eventLoop: EventLoop<()>)
    {
        let timeStart = Local::now();

        // Adds all levels that should be available for loading.
        self.API.SceneManager.AddScene("Level1", "Scenes/test.lvl");
        
        // Build starting scene.
        self.API.SceneManager.LoadScene("Level1", &self._display);

        Entities::Start(&mut self.API);

        // Enter frame loop
        let mut input = Input::New();
        let mut dateTimeLastFrame = Local::now();

        // TODO: Break this closure up into static functions
        eventLoop.run( |event, window_target|
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
                        let now = SystemTime::now();
                        let mut renderTime: u128 = 0;
                        let mut target = self._display.draw();

                        target.clear_color_and_depth((0.1, 0.0, 0.2, 1.0), 1.0);

                        self.API.Audio.Update();

                        let timeLastFrame = dateTimeLastFrame.clone();
                        let viewMatrix = self.API.SceneManager.Entities.Camera.ViewMatrix();
                        let perspective= self.API.SceneManager.Entities.Camera.PerspectiveMatrix();

                        // Maybe just avoid allocating new frame and instead just update values. 
                        let frame : Rc<GameFrame> =
                            Rc::new(
                                GameFrame::new(
                                    input.GetStateCopy(),
                                    Local::now() - timeStart,
                                    Local::now() - timeLastFrame,
                                    viewMatrix,
                                    perspective
                                )
                            );

                            // runtime mode update
                            match self._mode
                            {
                                EEngineMode::Play => 
                                    self._player.Update(
                                        &mut self.API,
                                        frame.clone(),
                                        &mut target
                                ),

                                EEngineMode::Editor => 
                                    self._editor.Update(
                                        &mut self.API,
                                        &mut input,
                                        timeStart,
                                        &mut dateTimeLastFrame
                                )
                            }

                        input.ResetPressedAndReleased();
                        input.SetMouseWheelPixelDelta((0.0, 0.0));
                        input.SetMouseWheelLineOffset((0.0, 0.0));

                        dateTimeLastFrame = Local::now();

                        let rnow = SystemTime::now();
                        let _ = target.finish();
                        self._display.finish();
                        renderTime = renderTime + rnow.elapsed().unwrap().as_millis();

                        match now.elapsed()
                        {
                            Ok(elapsed) =>
                                {
                                    println!("Mode: {}", self._mode);
                                    println!("Logic: {}ms fps:{}", elapsed.as_millis(), (1_000_000_000.0 / elapsed.as_nanos() as f32) as u32);
                                    println!("render: {}ms\n\n", renderTime);

                                },
                            _ => {}
                        };
                    },

                    // Because glium doesn't know about windows we need to resize the display
                    // when the window's size has changed.
                    winit::event::WindowEvent::Resized(window_size) =>
                    {
                        self._display.resize(window_size.into());
                    },

                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user Input you could remove this handler.
                winit::event::Event::AboutToWait => 
                {
                    self._window.request_redraw();
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




