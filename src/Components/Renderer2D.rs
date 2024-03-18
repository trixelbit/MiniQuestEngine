use std::cell::RefCell;
use std::rc::Rc;
use glium::{Display, glutin, Program, Surface, Texture2d, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::texture::RawImage2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use crate::Components::{Component};
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use crate::Components::RenderUtilities::{ImageBufferFromPath, Indicies, PlaneVertexBuffer, Vertex};

pub struct Renderer2D
{
    pub Texture: Texture2d,
    pub VertexBuffer: VertexBuffer<Vertex>,
    pub Indicies: NoIndices,
    pub Program: Program,
}

impl Renderer2D
{
    pub fn New(display : &Display<WindowSurface>) -> Self
    {
        let vertexBuffer = PlaneVertexBuffer(&display);
        let imageBuffer = ImageBufferFromPath("TestImg.png");

        let image_dimensions = imageBuffer.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&imageBuffer.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, image).unwrap();

        let program =
            Program::from_source(display, Self::VertexShader(), Self::FragmentCode(), None)
            .unwrap();

        Self
        {
            VertexBuffer: vertexBuffer,
            Texture: texture,
            Indicies: Indicies(),
            Program: program,
        }
    }
}
impl Renderer2D
{
    pub fn FragmentCode() -> &'static str
    {
        r#"
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
        "#
    }

    pub fn VertexShader() -> &'static str
    {
        r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
        "#
    }
}

impl Component for Renderer2D
{
    fn start(&mut self)
    {
    }

    fn update(&mut self, entity: Rc<RefCell<&mut Entity>>, frame: Rc<GameFrame>)
    {
    }

    fn render(&self, entity: &Entity, display: &Display<WindowSurface>)
    {
        let dim = display.get_framebuffer_dimensions();

        print!("x{} y{}", dim.0, dim.1);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let behavior = glium::uniforms::SamplerBehavior
        {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        let uniforms = uniform!
        {
            matrix:
            [
                [entity.scale.x() * self.Texture.dimensions().0 as f32 / (1f32 * dim.0 as f32) , 0.0, 0.0, 0.0],
                [0.0, entity.scale.y() * self.Texture.dimensions().1 as f32 / (1f32 * dim.1 as f32), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [entity.world_position.x(), entity.world_position.y(), 0.0, 1.0f32],
            ],

            tex: glium::uniforms::Sampler(&self.Texture, behavior),
        };

        target.draw(&self.VertexBuffer, &self.Indicies, &self.Program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
