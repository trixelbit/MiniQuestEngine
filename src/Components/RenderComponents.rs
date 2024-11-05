use std::rc::Rc;
use std::sync::{Arc, RwLock, Mutex};
use glium::{Display, Frame, Program, Surface, Texture2d, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::texture::RawImage2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use crate::Components::Component;
use crate::Frame::GameFrame;
use crate::GameEntity::Entity;
use crate::Components::RenderUtilities::{ImageBufferFromPath, Indicies, PlaneVertexBuffer, Vertex};
use crate::GameAPI::GameAPI;

pub trait Renderer
{
    fn render(&self, entity: &Entity, frame: &GameFrame, target: &mut Frame);
}

/// Draws a 2D sprite to screen.
pub struct Renderer2D
{
    pub VertexBuffer: VertexBuffer<Vertex>,
    pub Indicies: NoIndices,
    pub Program: Program,
    pub Display: Display<WindowSurface>,
    pub Sprite: Arc<Sprite>,
}

impl Renderer for Renderer2D
{
    fn render(&self, entity: &Entity, frame: &GameFrame, target: &mut Frame)
    {
        let dim = self.Display.get_framebuffer_dimensions();

        let behavior = glium::uniforms::SamplerBehavior
        {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            ..Default::default()
        };


        let rawTransform =
        [
            [entity.scale.x() * self.Sprite.Texture.dimensions().0 as f32 / (1f32 * dim.0 as f32 * self.Sprite.CellCounts.0 as f32) , 0.0, 0.0, 0.0],
            [0.0, entity.scale.y() * self.Sprite.Texture.dimensions().1 as f32 / (1f32 * dim.1 as f32 * self.Sprite.CellCounts.1 as f32), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [entity.world_position.x() / dim.0 as f32, entity.world_position.y() / dim.1 as f32, entity.world_position.z(), 1.0f32],
        ];

        let view_mat : [[f32;4];4] = frame.CameraView.into();
        let perspective_mat : [[f32;4];4] = frame.CameraPerspective.into();


        let uniforms = uniform!
        {
            view: view_mat,
            model: rawTransform,
            perspective: perspective_mat,

            tex: glium::uniforms::Sampler(&self.Sprite.Texture, behavior),
            time: frame.TimeSinceGameStart.num_milliseconds() as i32,
            cell_x_count: self.Sprite.CellCounts.0 as f32,
            cell_y_count: self.Sprite.CellCounts.1 as f32,
            frame_count: self.Sprite.FrameCount as f32,
            speed: self.Sprite.AnimationSpeed
        };

        target.draw(&self.VertexBuffer, &self.Indicies, &self.Program, &uniforms,
                    &Default::default()).unwrap();
    }
}

impl Renderer2D
{
    pub fn New(display : &Display<WindowSurface>, initialSprite: Arc<Sprite>) -> Rc<RwLock<Self>>
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

        Rc::new(
            RwLock::new(
                Self
                {
                    Sprite: initialSprite,
                    VertexBuffer: vertexBuffer,
                    Indicies: Indicies(),
                    Program: program,
                    Display: display.clone()
                }
            )
        )
    }

    pub fn set_new_sprite(&mut self, newSprite: Arc<Sprite>)
    {
        self.Sprite = newSprite;
    }

    pub fn FragmentCode() -> &'static str
    {
        r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform int time;
        uniform float frame_count;
        uniform float cell_x_count;
        uniform float cell_y_count;
        uniform float speed;

        uniform sampler2D tex;

        void main()
        {
            int currentIndex = int(mod(time * speed, frame_count));

            vec2 cellSize
                = vec2(
                    1.0 / cell_x_count,
                    1.0 / cell_y_count
                );

            vec2 offset = vec2(
                 mod(float(currentIndex), cell_x_count) / cell_x_count,
                  1 - (0.5 * floor(2 * float(currentIndex) * cellSize.x * cellSize.y))
            );


            vec2 cellCoord = vec2(
                v_tex_coords.x * cellSize.x,
                -(1 - v_tex_coords.y) * cellSize.y
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

        in vec3 position;
        in vec3 normal;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            v_tex_coords = tex_coords;
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            //gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
        "#
    }
}

impl Component for Renderer2D
{
    fn start(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>)
    {
    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
    }
}

pub struct Sprite
{
    /// Based texture contains sprite sheet
    pub Texture: Texture2d,

    /// Total Number of sprites in sheet
    pub FrameCount: u16,

    /// numbers cells horizontally and vertically in sprite sheet
    pub CellCounts: (u16, u16),

    /// Speed the animation plays at
    pub AnimationSpeed: f32

}

impl Sprite
{
    /// Creates a simple, un animated sprite
    pub fn new_simple(spritePath: &str, display: &Display<WindowSurface>) -> Arc<Self>
    {
        let imageBuffer = ImageBufferFromPath(spritePath);
        let image_dimensions = imageBuffer.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&imageBuffer.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, image).unwrap();

        Arc::new(
            Sprite
            {
                Texture: texture,
                FrameCount: 1,
                CellCounts: (1,1),
                AnimationSpeed: 1.0
            }
        )
    }

    /// Creates a new Sprite
    ///
    /// spritePath - Path to Sprite Image (png).
    /// display - Display reference.
    /// frameCount - total number of frames.
    /// cellCounts - number of cell rows and columns.
    /// animationSpeed - speed animation should play at.
    pub fn new
    (spritePath: &str, display: &Display<WindowSurface>, frameCount: u16,
     cellCounts: (u16, u16), animationSpeed: f32) -> Arc<Sprite>
    {
        let imageBuffer = ImageBufferFromPath(spritePath);
        let image_dimensions = imageBuffer.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&imageBuffer.into_raw(), image_dimensions);
        let texture = Texture2d::new(display, image).unwrap();

        Arc::new(
            Sprite
            {
                Texture: texture,
                FrameCount: frameCount,
                CellCounts: cellCounts,
                AnimationSpeed: animationSpeed
            })
    }
}
