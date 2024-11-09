use crate::GameEntity::Entity;
use crate::Components::Component;
use crate::Frame::GameFrame;
use crate::GameAPI::GameAPI;
use crate::Math::Float3;
use crate::Collision::collider::{ColliderData, ECollisionType, ECollisionTag};
use crate::Components::RenderComponents::{Renderer, Renderer2D, Sprite};
use crate::Components::RenderUtilities::{Indicies, PlaneVertexBuffer, Vertex};
use crate::DEBUG_MODE;

use std::sync::{RwLock, Mutex, Arc};
use std::rc::Rc;
use glium::{BackfaceCullingMode, Depth, Display, DrawParameters, Frame, PolygonMode, Program, Surface, VertexBuffer};
use glium::draw_parameters::{ClipControlDepth, ClipControlOrigin, ProvokingVertex};
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};

use super::RenderComponents::{DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER};


/// This component reports current collision data to collision module for most recent information
pub struct Collider
{
    _data: ColliderData,
    // TODO: Add Offset

    // WireFrameRendering
    _display: Display<WindowSurface>,
    _indicies: NoIndices,
    _program: Program,
    _vertexBuffer: VertexBuffer<Vertex>,
    _debugSprite: Arc<Sprite>
}

impl Collider
{
    pub fn Create(
        display: Display<WindowSurface>,
        position: Float3,
        size: Float3,
        collisionType: ECollisionType,
        tag: ECollisionTag)
        -> Rc<RwLock<Self>>
    {
        Rc::new(
            RwLock::new(
                Self
                {
                    _data: ColliderData::Create(
                        position,
                        size,
                        collisionType,
                        tag
                    ),

                    _indicies: Indicies(),
                    _vertexBuffer: PlaneVertexBuffer(&display),
                    _program: Program::from_source(&display,
                                                    DEFAULT_VERTEX_SHADER,
                                                    DEFAULT_FRAGMENT_SHADER,
                                                    None).unwrap(),
                    _debugSprite: Sprite::new_simple("Assets/collider.png", &display),
                    _display: display.clone(),

                }
            )
        )
    }
}

impl Component for Collider
{
    fn start(&mut self, 
        entity: &mut Entity, 
        api: Arc<Mutex<GameAPI>>)
    {
        api.lock().unwrap().Collision.Add(entity.ID(), self._data);
        api.lock().unwrap().Collision.UpdateOrigin(entity.ID(), entity.world_position);
    }

    fn update(&mut self, entity: &mut Entity, frame: &GameFrame, api: Arc<Mutex<GameAPI>>)
    {
        api.lock().unwrap().Collision.UpdateOrigin(entity.ID(), entity.world_position);
    }

    fn OnDestroy(&mut self, entity: &mut Entity, api: Arc<Mutex<GameAPI>>) 
    {
        api.lock().unwrap().Collision.Remove(entity.ID());
    }
}

impl Renderer for Collider
{
    /// Draws bounds of collider if debug mode enabled
    fn render(&self, entity: &Entity, frame: &GameFrame, target: &mut Frame)
    {
        if !DEBUG_MODE
        {
            return;
        }

        let dim = self._display.get_framebuffer_dimensions();

        let behavior = glium::uniforms::SamplerBehavior
        {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            ..Default::default()
        };


        let rawTransform =
            [
                [entity.scale.x() * self._data.Size().x() / (1f32 * dim.0 as f32) , 0.0, 0.0, 0.0],
                [0.0, entity.scale.y() * self._data.Size().y() / (1f32 * dim.1 as f32), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [entity.world_position.x() / dim.0 as f32, entity.world_position.y() / dim.1 as f32, entity.world_position.z(), 1.0f32],
            ];

        let view_mat : [[f32;4];4] = frame.CameraView.into();
        let perspective_mat : [[f32;4];4] = frame.CameraPerspective.into();


        // TODO: Make Debug Shaders Programs
        let uniforms = uniform!
        {
            view: view_mat,
            model: rawTransform,
            perspective: perspective_mat,

            tex: glium::uniforms::Sampler(&self._debugSprite.Texture, behavior),
            time: frame.TimeSinceGameStart.num_milliseconds() as i32,
            cell_x_count: 1.0f32,
            cell_y_count: 1.0f32,
            frame_count: 1.0f32,
            speed: 1.0f32
        };

        let draw = DrawParameters {
            depth: Depth::default(),
            stencil: Default::default(),
            blend: Default::default(),
            color_mask: (true, true, true, true),
            line_width: None,
            point_size: None,
            backface_culling: BackfaceCullingMode::CullingDisabled,
            polygon_mode: PolygonMode::Line,
            clip_planes_bitmask: 0,
            multisampling: true,
            dithering: true,
            viewport: None,
            scissor: None,
            draw_primitives: true,
            samples_passed_query: None,
            time_elapsed_query: None,
            primitives_generated_query: None,
            transform_feedback_primitives_written_query: None,
            condition: None,
            transform_feedback: None,
            smooth: None,
            provoking_vertex: ProvokingVertex::LastVertex,
            primitive_bounding_box: (-1.0 .. 1.0, -1.0 .. 1.0, -1.0 .. 1.0, -1.0 .. 1.0),
            primitive_restart_index: false,
            polygon_offset: Default::default(),
            clip_control_origin: ClipControlOrigin::LowerLeft,
            clip_control_depth: ClipControlDepth::NegativeOneToOne,
        };

        target.draw(
            &self._vertexBuffer,
            &self._indicies,
            &self._program,
            &uniforms,
            &draw).unwrap();
    }
}




