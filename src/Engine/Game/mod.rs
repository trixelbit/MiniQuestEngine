#![allow(nonstandard_style)]

use std::sync::Arc;
use std::sync::Mutex;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use winit::event::KeyEvent;
use winit::event::MouseButton;
use winit::event::{ElementState, MouseScrollDelta, TouchPhase};

use crate::Engine::Frame::GameFrame;
use crate::Engine::Frame::Input::Input;
use crate::Engine::GameEntity::TEntity;
use crate::Engine::Components::{self, *};
use crate::Engine::Math::*;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::SceneBuilder::SceneBuilderFunction;

/// The Game Application that is running currently.
pub struct Game    
{
    // TODO: Make this Arc Mutex so it can be shared across threads for async operations.
    pub API: Arc<Mutex<GameAPI>>,
}

impl Game
{
    /// Constructs game and performs any tasks before actual application window opens.
    pub fn New(sceneBuilderMethod: SceneBuilderFunction) -> Self
    {
        Self
        {
            API: 
                Arc::new(
                    Mutex::new(
                        GameAPI::Create(sceneBuilderMethod)
            ))
        }
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
        self.API.lock().unwrap().SceneManager.AddScene("Level1", "Scenes/test.lvl");
        
        // Build starting scene.
        self.API.lock().unwrap().SceneManager.LoadScene("Level1", &display);


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
                            Self::Update(
                                &display,
                                self.API.clone(),
                                &mut input,
                                timeStart,
                                &mut dateTimeLastFrame
                            );
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



    /// General Engine Update cycle.
    pub fn Update(
        display: &Display<WindowSurface>,
        api: Arc<Mutex<GameAPI>>,
        input: &mut Input,
        timeStart: DateTime<Local>,
        dateTimeLastFrame: &mut DateTime<Local>
    )
    {
        println!("Update");
        let now = SystemTime::now();
        let mut renderTime: u128 = 0;

        let mut target = display.draw();

        target.clear_color_and_depth((0.1, 0.0, 0.2, 1.0), 1.0);

        api.lock().unwrap().Audio.Update();

        let timeLastFrame = dateTimeLastFrame.clone();

        let viewMatrix = api.clone().lock().unwrap().SceneManager.Entities.Camera.ViewMatrix();
        let perspective= api.clone().lock().unwrap().SceneManager.Entities.Camera.PerspectiveMatrix();

        let frame =
            Rc::new(
                GameFrame::new(
                    input.GetStateCopy(),
                    Local::now() - timeStart,
                    Local::now() - timeLastFrame,
                    viewMatrix,
                    perspective
                )
            );

        let mut binding = api.clone();
        let mut a = binding.lock().unwrap();
        a.SceneManager.Entities.Update(&frame, api.clone(), &mut target);
        a.SceneManager.Entities.PruneDeadEntities();


        input.ResetPressedAndReleased();
        input.SetMouseWheelPixelDelta((0.0, 0.0));
        input.SetMouseWheelLineOffset((0.0, 0.0));

        *dateTimeLastFrame = Local::now();

        let rnow = SystemTime::now();
        let _ = target.finish();
        display.finish();
        renderTime = renderTime + rnow.elapsed().unwrap().as_millis();

        match now.elapsed()
        {
                Ok(elapsed) => 
                    {
                        println!("ms:{} fps:{}", elapsed.as_millis(), (1_000_000_000.0 / elapsed.as_nanos() as f32) as u32);
                        println!("render:{}", renderTime);

                    },
                _ => {}
        };
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


