#![allow(nonstandard_style)]
mod Math;
mod GameEntity;
mod Frame;
mod Components;


use Math::*;

#[macro_use]
extern crate glium;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use chrono::{Local};
use glium::{Surface};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase};
use winit::keyboard::NamedKey::Camera;
use crate::Frame::GameFrame;
use crate::Frame::Input::Input;
use crate::GameEntity::Entity;
use crate::Components::*;
use crate::Components::RenderComponents::{Renderer, Renderer2D, Sprite};


fn main()
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



    /// scene build

    let position = Float3::new(0.0, -0.5, 0.0);
    let mut player = Rc::new(RefCell::new(Entity::new(position)));
    player.borrow_mut().scale = Float3::scale_value(Float3::one(), 5.0);


    let renderComponent =
        Rc::new(
            RwLock::new(
                Renderer2D::New(&display,
                    Sprite::new(
                        "Images/run_down.png",
                        &display,
                        4,
                        (2,2),
                        0.001))
    ));

    let movementComponent =
        Rc::new(
            RwLock::new(
                PlayerController::PlayerController::new(8.0f32, &display)));
    let mut playerMut = player.borrow_mut();
    playerMut.add_component(movementComponent);
    playerMut.add_component(renderComponent);
    drop(playerMut);


    let camera = Rc::new(
        RwLock::new(
            Components::Camera::Camera::New(30.0),
        )
    );

    let cameraController =
        Rc::new(
            RwLock::new(
                Components::Camera::CameraMouseController::New()));

    let mut cameraEnt =
        Rc::new(
            RefCell::new(
                Entity::new(
                    Float3::new(0.0, 0.0, 5.0)
                )
            )
        );
    cameraEnt.borrow_mut().add_component(camera.clone());
    cameraEnt.borrow_mut().add_component(cameraController);


    /// object registry
    let mut entities: Vec<Rc<RefCell<Entity>>> = Vec::new();
    entities.push(player.clone());
    entities.push(cameraEnt.clone());


    /// Enter frame loop

    let mut input = Input::New();
    let mut dateTimeLastFrame = Local::now();

    for entityMutRef in &entities
    {
        let mut entity = entityMutRef.borrow_mut();
        entity.start();
    }

    event_loop.run(move |event, window_target|
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
                        let mut target = display.draw();

                        target.clear_color(0.5, 0.0, 0.5, 1.0);

                        for entityMutex in &entities
                        {
                            let frame =
                                Rc::new(
                                    GameFrame::new(
                                            input.GetStateCopy(),
                                                   Local::now() - timeStart,
                                                   Local::now() - dateTimeLastFrame,
                                                   camera.read().unwrap().ViewMatrix(),
                                            camera.read().unwrap().PerspectiveMatrix()
                                    )
                                );

                            let mut entity = entityMutex.borrow_mut();
                            entity.update(&frame);

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
                        }

                        input.ResetPressedAndReleased();
                        input.SetMouseWheelPixelDelta((0.0, 0.0));
                        input.SetMouseWheelLineOffset((0.0, 0.0));
                        dateTimeLastFrame = Local::now();

                        target.finish();
                        display.finish();
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
