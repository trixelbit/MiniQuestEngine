use std::rc::Rc;
use std::sync::{Arc, RwLock, Mutex};
use glium::{Display, Frame, Program, Surface, Texture2d, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::texture::RawImage2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use crate::Engine::Components::Component;
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameEntity::Entity;
use crate::Engine::Components::RenderUtilities::{ImageBufferFromPath, Indicies, PlaneVertexBuffer, Vertex};
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::Math::Float3;
use crate::Engine::Shader::{DEFAULT_FRAGMENT, DEFAULT_VERTEX};
use std::time::{Instant, Duration};



pub struct LightSource
{
    pub Color : Float3,
    pub Intensity: f32
}

impl Component for LightSource
{
    fn start(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>)
    {
    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
    }
}



/// Draws a 2D sprite to screen.
pub struct Renderer2D
{

    pub VertexBuffer: VertexBuffer<Vertex>,
    pub Indices: NoIndices,
    pub Program: Program,
    pub Display: Display<WindowSurface>,
    pub Sprite: Arc<Sprite>,

    _vertexShader: Option<String>,
    _fragmentShader: Option<String>,
    _playTime: Instant,
   
    _currentIndex: i32,
    _loops: bool,
    _completed: bool,

    _isLit: bool,
}

impl Renderer2D
{
    /// Creates a new 2D Rendering component
    /// 
    /// Display - Display Reference
    /// Sprite - Sprite that should be rendered
    pub fn New(
        display : &Display<WindowSurface>, 
        initialSprite: Arc<Sprite>,
        isLit: bool
        ) -> Rc<RwLock<Self>>
    {
        let vertexBuffer = PlaneVertexBuffer(&display);

        Rc::new(
            RwLock::new(
                Self
                {
                    Display: display.clone(),
                    Sprite: initialSprite,
                    VertexBuffer: vertexBuffer,
                    Indices: Indicies(),
                    Program: Program::from_source(display, DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER, None).unwrap(),
                    _fragmentShader: None,
                    _vertexShader: None,

                    _currentIndex: 0,
                    _loops: true,
                    _completed: false,
                    _playTime: Instant::now(),
                    _isLit: isLit,
                }
            )
        )
    }

    pub fn SetSprite1Loop(&mut self, newSprite: Arc<Sprite>)
    {
        self._playTime = Instant::now();
        self._currentIndex = 0;
        self.Sprite = newSprite;
        self._loops = false;
        self._completed = false;
    }

    pub fn set_new_sprite(&mut self, newSprite: Arc<Sprite>)
    {
        self._playTime = Instant::now();
        self._currentIndex = 0;
        self.Sprite = newSprite;
        self._loops = true;
        self._completed = false;
    }

    // properties

    /// Current index of sprite
    pub fn CurrentIndex(&self) -> i32
    {
        self._currentIndex
    }


    /// Returns true if this is a non looped animation and it has finished playing.
    pub fn IsComplete(&self) -> bool
    {
        self._completed
    }

    pub fn ChangeLightState(&mut self, isLit: bool)
    {
        self._isLit = isLit;
    }
}

pub trait Renderer
{
    fn render(&mut self, entity: &Entity, frame: &GameFrame, target: &mut Frame);
}

impl Renderer for Renderer2D
{
    fn render(&mut self, entity: &Entity, frame: &GameFrame, target: &mut Frame)
    {
        
        // calculate sprite index
        let elapsedTime = self._playTime.elapsed().as_millis() as f32;

        if(self._loops || !self._completed)
        {
            self._currentIndex 
                = 
                (
                    (elapsedTime * self.Sprite.AnimationSpeed as f32) 
                    % 
                    self.Sprite.FrameCount as f32
                ) 
                as i32;

            if(!self._loops && 
                (self._currentIndex == (self.Sprite.FrameCount - 1) as i32))
            {
                self._completed = true;
            }
        }



        let dim = self.Display.get_framebuffer_dimensions();

        let behavior = glium::uniforms::SamplerBehavior
        {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            ..Default::default()
        };

        let x = entity.world_position.x();
        let y = entity.world_position.y();
        let z = entity.world_position.z();


        let display_width = dim.0 as f32;
        let display_height = dim.1 as f32;

        let image_dimension_x = self.Sprite.Texture.dimensions().0 as f32;
        let image_dimension_y = self.Sprite.Texture.dimensions().1 as f32;
        let cell_count_x = self.Sprite.CellCounts.0 as f32;
        let cell_count_y = self.Sprite.CellCounts.1 as f32;

        let scale = 
        if(entity.world_position.z() < 0.0)
        {
            Float3::scale_value(entity.scale, -entity.world_position.z())
        }
        else
        {
            entity.scale
        };

        let rawTransform =
        [
            [
                scale.x() * image_dimension_x / (1.0 * display_width * cell_count_x), 
                0.0, 
                0.0, 
                0.0],
            [
                0.0, 
                scale.y() * image_dimension_y / (1.0 * display_height * cell_count_y), 
                0.0, 
                0.0],
            [
                0.0, 
                0.0, 
                1.0, 
                0.0],
            [
                x / display_width, 
                y / display_height, 
                z, 
                1.0],
        ];

        let view_mat : [[f32;4];4] = frame.CameraView.into();
        let perspective_mat : [[f32;4];4] = frame.CameraPerspective.into();


        let uniforms = uniform!
        {
            view: view_mat,
            model: rawTransform,
            perspective: perspective_mat,

            tex: glium::uniforms::Sampler(&self.Sprite.Texture, behavior),
            is_lit: self._isLit,
            current_index: self._currentIndex,
            pixel_dimension_x: image_dimension_x,
            pixel_dimension_y: image_dimension_y,
            time: frame.TimeSinceGameStart.num_milliseconds() as i32,
            cell_x_count: self.Sprite.CellCounts.0 as f32,
            cell_y_count: self.Sprite.CellCounts.1 as f32,
            frame_count: self.Sprite.FrameCount as f32,
            speed: self.Sprite.AnimationSpeed
        };

        let params = glium::DrawParameters{
            depth: glium::Depth
                {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },

                .. Default::default()
        };

        target.draw(&self.VertexBuffer, &self.Indices, &self.Program, &uniforms,
                    &params//Default::default()

        ).unwrap();
    }
}


impl Component for Renderer2D
{
    fn start(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>)
    {
        let vertexBuffer = PlaneVertexBuffer(&self.Display);


        let loadedFragmentShader: String = 
        match &self._fragmentShader
        {
            Some(x) => api.lock().unwrap().Shader.GetShader(x.as_str()),
            None => api.lock().unwrap().Shader.GetShader(DEFAULT_FRAGMENT)
        };

        let loadedVertexShader: String = 
        match &self._vertexShader
        {
            Some(x) => api.lock().unwrap().Shader.GetShader(x.as_str()),
            None => api.lock().unwrap().Shader.GetShader(DEFAULT_VERTEX)
        };
        

        let result =
            Program::from_source(&self.Display, loadedVertexShader.as_str(), loadedFragmentShader.as_str(), None);

        match result
        {
            Ok(_) => {}
            Err(x) => {panic!("Render Program Error: {}", x.to_string())}
        }

        let program = result.unwrap();

        self.VertexBuffer = vertexBuffer;
        self.Indices = Indicies();
        self.Program = program;
        self._fragmentShader = Some(loadedFragmentShader);
        self._vertexShader = Some(loadedVertexShader);

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

    /// Creates a new Sprite.
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



pub const DEFAULT_FRAGMENT_SHADER: &str = 
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
    "#;

pub const DEFAULT_VERTEX_SHADER: &str = 
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
    "#;
