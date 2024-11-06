#![allow(nonstandard_style)]

use std::sync::Arc;
use std::sync::Mutex;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;
use chrono::{DateTime, Local};
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use winit::event::{ElementState, MouseScrollDelta, TouchPhase};

use crate::Frame::GameFrame;
use crate::Frame::Input::Input;
use crate::GameEntity::Entity;
use crate::Components::{self, *};
use crate::Components::RenderComponents::{Renderer, Renderer2D};
use crate::Math::*;
use crate::GameAPI::GameAPI;

/// The Game Application that is running currently.
pub struct Game    
{
    // TODO: Make this Arc Mutex so it can be shared across threads for async operations.
    pub API: Arc<Mutex<GameAPI>>,
}

impl Game
{
    /// Constructs game and performs any tasks before actual application window opens.
    pub fn New() -> Self
    {
        Self
        {
            API: 
                Arc::new(
                    Mutex::new(
                        GameAPI::Create()
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

        let camera = Rc::new(
            RwLock::new(
                Components::Camera::Camera::New(30.0),
            )
        );

        let cameraController =
            Rc::new(
                RwLock::new(
                    Components::Camera::CameraMouseController::New()));

        let cameraEnt =
            Rc::new(
                RefCell::new(
                    Entity::new(
                        "",
                        Float3::new(0.0, 0.0, 1.0)
                    )
                )
            );
        cameraEnt.borrow_mut().add_component(camera.clone());
        cameraEnt.borrow_mut().add_component(cameraController);

        self.API.lock().unwrap().SceneManager.AddEntity(cameraEnt.clone());


        // Enter frame loop
        let mut input = Input::New();
        let mut dateTimeLastFrame = Local::now();

        // TODO: This is bad because it does not respond to the creation and deletion of entities
        // on runtime.
        let entityList =  &self.API.lock().unwrap().SceneManager.Entities.clone();

        for entityMutRef in entityList
        {
            let mut entity = entityMutRef.borrow_mut();
            entity.start(self.API.clone());
        }

        // TODO: Break this closure up into static functions
        event_loop.run( |event, window_target|
        {
            match event
            {
                winit::event::Event::WindowEvent { event, .. } => match event
                {
                    winit::event::WindowEvent::KeyboardInput {event, ..} =>
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
                    },

                    winit::event::WindowEvent::MouseInput {state, device_id, button, ..} =>
                    {
                        match state
                        {
                            ElementState::Pressed =>
                                {
                                    &input.Mouse_Pressed(button);
                                }
                            ElementState::Released =>
                                {
                                    &input.Mouse_Release(button);
                                }
                        }
                    },

                    winit::event::WindowEvent::MouseWheel {phase, delta, ..} =>
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

                    winit::event::WindowEvent::CursorMoved {position,..} =>
                    {
                        &input.SetMousePosition((position.x, position.y));
                    }

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
                                &mut dateTimeLastFrame,
                                camera.clone()
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

    pub fn Update(
        display: &Display<WindowSurface>,
        api: Arc<Mutex<GameAPI>>,
        input: &mut Input,
        timeStart: DateTime<Local>,
        dateTimeLastFrame: &mut DateTime<Local>,
        camera: Rc<RwLock<Camera::Camera>>
    )
    {
        let mut target = display.draw();

        target.clear_color(0.1, 0.0, 0.2, 1.0);

        api.lock().unwrap().Audio.Update();

        let timeLastFrame = dateTimeLastFrame.clone();

        let list = &api.lock().unwrap().SceneManager.Entities.clone();
        for entityMutex in list
        {
            let frame =
                Rc::new(
                    GameFrame::new(
                        input.GetStateCopy(),
                        Local::now() - timeStart,
                        Local::now() - timeLastFrame,
                        camera.read().unwrap().ViewMatrix(),
                        camera.read().unwrap().PerspectiveMatrix()
                    )
                );

            let mut entity = entityMutex.borrow_mut();

            if !entity.HasStartBeenCalled()
            {
                entity.start(api.clone());
            }

            entity.update(&frame, api.clone());


            // Render
            let renderOption =
                entity.get_component::<Renderer2D>(None);

            match renderOption
            {
                None => {}
                Some(_) =>
                    {
                        renderOption.unwrap().write().unwrap().render(&entity, &frame, &mut target);
                    }
            }

            // Destroy dead objects
            api.lock().unwrap().SceneManager.PruneDeadObject(api.clone());
        }


        input.ResetPressedAndReleased();
        input.SetMouseWheelPixelDelta((0.0, 0.0));
        input.SetMouseWheelLineOffset((0.0, 0.0));

        *dateTimeLastFrame = Local::now();

        target.finish();
        display.finish();
    }

}




