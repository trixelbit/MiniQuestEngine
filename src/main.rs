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
use std::sync::Mutex;
use chrono::{DateTime, Local};
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use lazy_static::lazy_static;
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

    let position = Vector3::new(0.0, -0.5, 1.0);
    let mut player = Rc::new(RefCell::new(Entity::new(position)));
    player.borrow_mut().scale = Vector3::scale_value(Vector3::one(), 5.0);

    let movementComponent = Rc::new(RefCell::new(
        PlayerController::PlayerController{
            _speed: 0.002f32,
            _entity: Rc::clone(&player),
            _velocity: Vector3::zero(),
        }));

    let renderComponent = Rc::new(RefCell::new(
        Renderer2D::New(&display,
            Sprite::new(
              "Images/run_down-Sheet.png",
              &display,
              4,
              (2,2),
              0.001))
    ));

    player.borrow_mut().add_component(movementComponent.clone());
    player.borrow_mut().add_component(renderComponent.clone());


    let mut entities: Vec<Rc<RefCell<Entity>>> = Vec::new();
    entities.push(player.clone());


    let mut input = Input::New();
    let mut dateTimeLastFrame = Local::now();

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
