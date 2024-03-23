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
use std::sync::{Arc, RwLock};
use chrono::{Local};
use glium::{Surface};
use winit::event::ElementState;
use crate::Frame::GameFrame;
use crate::Frame::Input::Input;
use crate::GameEntity::Entity;
use crate::Components::*;
use crate::Components::RenderComponents::{Renderer2D, Sprite};


fn main()
{
    let timeStart = Local::now();

    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Mini Quest Engine Test")
        .build(&event_loop);



    /// scene build

    let position = Vector3::new(0.0, -0.5, 1.0);
    let mut player = Rc::new(RefCell::new(Entity::new(position)));
    player.borrow_mut().scale = Vector3::scale_value(Vector3::one(), 5.0);


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
                PlayerController::PlayerController::new(4.0f32, &display)));

    let mut playerMut = player.borrow_mut();
    playerMut.add_component(movementComponent);
    playerMut.add_component(renderComponent);
    drop(playerMut);


    let mut entities: Vec<Rc<RefCell<Entity>>> = Vec::new();
    entities.push(player.clone());


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
                            &input.Pressed(event.physical_key)
                        },

                        ElementState::Released =>
                        {
                            &input.Released(event.physical_key)
                        }
                    };
                },

                winit::event::WindowEvent::CloseRequested =>
                    {
                        window_target.exit();
                    },


                // We now need to render everything in response to a RedrawRequested event due to the animation
                winit::event::WindowEvent::RedrawRequested =>
                    {
                        for entityMutex in &entities
                        {
                            let frame =
                                Rc::new(
                                    GameFrame::new(input.GetStateCopy(),
                                                   Local::now() - timeStart,
                                                   Local::now() - dateTimeLastFrame
                                    )
                                );

                            let mut entity = entityMutex.borrow_mut();
                            entity.update(&frame);

                            entity.render(&frame);
                        }

                        dateTimeLastFrame = Local::now();
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
