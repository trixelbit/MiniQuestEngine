use std::cell::RefCell;
use std::rc::Rc;
use glium::{Display, glutin, Program, ProgramCreationError, Surface, Texture2d, VertexBuffer};
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
    pub VertexBuffer: VertexBuffer<Vertex>,
    pub Indicies: NoIndices,
    pub Program: Program,

    pub Display: Display<WindowSurface>,

    pub Sprite: Sprite
}

impl Renderer2D
{
    pub fn New(display : &Display<WindowSurface>, initialSprite: Sprite) -> Self
    {
        let vertexBuffer = PlaneVertexBuffer(&display);

        let result =
            Program::from_source(display, Self::VertexShader(), Self::FragmentCode(), None);

        match result
        {
            Ok(_) => {}
            Err(x) => {panic!("Render Program Errored: {}", x.to_string())}
        }

        let program = result.unwrap();

        Self
        {
            Sprite: initialSprite,
            VertexBuffer: vertexBuffer,
            Indicies: Indicies(),
            Program: program,
            Display: display.clone()
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

        uniform int time;
        uniform int frame_count;
        uniform int cell_x_count;
        uniform int cell_y_count;
        uniform float speed;

        uniform sampler2D tex;

        void main()
        {
            int currentIndex = int(mod(time * speed, frame_count));

            vec2 offset = vec2(
                 mod(currentIndex, cell_x_count),
                 int(currentIndex / cell_x_count)
            );

            vec2 cellCoord = vec2(
                v_tex_coords.x / cell_x_count,
                v_tex_coords.y / cell_y_count
            );

            vec2 samplePoint = offset + cellCoord;
            color = texture(tex, samplePoint);

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

    fn update(&mut self, entity: Rc<RefCell<&mut Entity>>, frame: &GameFrame)
    {
    }

    fn render(&self, entity: &Entity, frame: &GameFrame)
    {
        let dim = self.Display.get_framebuffer_dimensions();

        let mut target = self.Display.draw();
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
                [entity.scale.x() * self.Sprite.Texture.dimensions().0 as f32 / (1f32 * dim.0 as f32) , 0.0, 0.0, 0.0],
                [0.0, entity.scale.y() * self.Sprite.Texture.dimensions().1 as f32 / (1f32 * dim.1 as f32), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [entity.world_position.x(), entity.world_position.y(), entity.world_position.z(), 1.0f32],
            ],

            tex: glium::uniforms::Sampler(&self.Sprite.Texture, behavior),
            time: frame.TimeSinceGameStart.num_milliseconds() as i32,
            cell_x_count: self.Sprite.CellCounts.0 as i32,
            cell_y_count: self.Sprite.CellCounts.1 as i32,
            frame_count: self.Sprite.FrameCount as i32,
            speed: self.Sprite.AnimationSpeed
        };

        target.draw(&self.VertexBuffer, &self.Indicies, &self.Program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }
}



pub struct Sprite
{
    pub Texture: Texture2d,

    /// Total Number of sprites in sheet
    pub FrameCount: u16,

    /// numbers cells horizontally and vertically in sprite sheet
    pub CellCounts: (u16, u16),

    /// Speed the animation plays at.
    pub AnimationSpeed: f32

}

impl Sprite
{
    pub fn new_simple(spritePath: &str, display: &Display<WindowSurface>) -> Self
    {
        let imageBuffer = ImageBufferFromPath(spritePath);
        let image_dimensions = imageBuffer.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&imageBuffer.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, image).unwrap();

        Self
        {
            Texture: texture,
            FrameCount: 1,
            CellCounts: (1,1),
            AnimationSpeed: 1.0
        }

    }
    pub fn new
    (spritePath: &str, display: &Display<WindowSurface>, spriteCount: u16,
               cellCounts: (u16, u16), animationSpeed: f32) -> Self
    {
        let imageBuffer = ImageBufferFromPath(spritePath);
        let image_dimensions = imageBuffer.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&imageBuffer.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, image).unwrap();

        Self
        {
            Texture: texture,
            FrameCount: spriteCount,
            CellCounts: cellCounts,
            AnimationSpeed: animationSpeed
        }
    }
}