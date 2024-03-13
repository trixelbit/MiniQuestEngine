#![allow(nonstandard_style)]
mod Math;
mod GameEntity;
mod Frame;

use Math::*;

#[macro_use]
extern crate glium;

use std::any::Any;
use std::cell::RefCell;
use std::ffi::c_uchar;
use std::rc::Rc;
use std::sync::Mutex;
use glium::{Surface, Texture2d};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use image::RgbaImage;
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use crate::Frame::GameFrame;
use crate::Frame::input::Input;
use crate::GameEntity::{Component, Entity, PlayerController};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #6")
        .build(&event_loop);

    let image = image::load(std::io::Cursor::new(&include_bytes!("../TestImg.png")),
                            image::ImageFormat::Png).unwrap().to_rgba16();//.to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::Texture2d::new(&display, image).unwrap();

    let position = Vector3::new(0.0, 1.1, 1.0);
    let mut player = Rc::new(RefCell::new(Entity::new(position, texture)));

    let movementComponent = Rc::new(RefCell::new(
        PlayerController{
            _speed: 0.002f32,
            _entity: Rc::clone(&player)
        }));
    //movementComponent.borrow_mut().attach_entity(player.clone());
    player.borrow_mut().add_component(movementComponent.clone());

    let mut entities: Vec<Rc<RefCell<Entity>>> = Vec::new();
    entities.push(player.clone());

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);
    // We've changed our shape to a rectangle so the image isn't distorted.
    let shape = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },

        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
    ];
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
            if(color.a < .01)
            {
                discard;
            }

        }
    "#;
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let behavior = glium::uniforms::SamplerBehavior
    {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    let mut input = Input::New();

    event_loop.run(move |event, window_target|
    {
        match event
        {
            winit::event::Event::WindowEvent { event, .. } => match event
            {
                winit::event::WindowEvent::KeyboardInput {event, ..} => match &event.physical_key
                {
                    key =>
                    {
                        &input.reset_maps();
                        &input.set_key_state(key.clone(), true);
                    }
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
                            let mut entity = entityMutex.borrow_mut();
                            entity.update(
                                Rc::new(
                                    GameFrame::new(input.GetStateCopy())
                                ));

                            let mut target = display.draw();
                            target.clear_color(0.0, 0.0, 1.0, 1.0);

                            let uniforms = uniform!
                            {
                                matrix:
                                [
                                    [entity.scale.x() * entity.texture.dimensions().0 as f32 / 100f32 , 0.0, 0.0, 0.0],
                                    [0.0, entity.scale.y() * entity.texture.dimensions().1 as f32 / 100f32, 0.0, 0.0],
                                    [0.0, 0.0, 1.0, 0.0],
                                    [entity.world_position.x(), entity.world_position.y(), 0.0, 1.0f32],
                                ],
                                tex: glium::uniforms::Sampler(&entity.texture, behavior),
                            };

                            target.draw(&vertex_buffer, &indices, &program, &uniforms,
                                        &Default::default()).unwrap();
                            target.finish().unwrap();
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
            // For applications that only change due to user input you could remove this handler.
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
        .unwrap();
}
