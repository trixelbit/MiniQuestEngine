use glium::VertexBuffer;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use image::{ImageBuffer, Rgba};

#[derive(Copy, Clone)]
pub struct Vertex
{
    position: [f32; 2],
    tex_coords: [f32; 2],
}

pub fn PlaneVertexBuffer(display: &Display<WindowSurface>) -> VertexBuffer<Vertex>
{
    implement_vertex!(Vertex, position, tex_coords);

    let shape = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },

        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
    ];


    VertexBuffer::new(display, &shape).unwrap()
}

pub fn Indicies() -> NoIndices
{
    NoIndices(glium::index::PrimitiveType::TrianglesList)
}

pub fn ImageBufferFromPath(path: &str) -> ImageBuffer<Rgba<u16>, Vec<u16>>
{
    // Read bytes from the file at the given path
    let bytes = match std::fs::read(path)
    {
        Ok(content) => content,
        Err(err) => {
            panic!("Error reading file: {}", err);
        }
    };

    // Load the image from the bytes
    let image = image::load_from_memory(&bytes)
        .unwrap()
        .to_rgba16();

    image
}
