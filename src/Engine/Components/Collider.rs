use crate::Engine::Editor::TNewLevelClone;
use crate::Engine::GameEntity::{TEntity, EntityHeader};
use crate::Engine::Frame::GameFrame;
use crate::Engine::GameAPI::GameAPI;
use crate::Engine::Math::Float3;
use crate::Engine::Collision::collider::{ColliderData, ECollisionType, ECollisionTag};
use crate::Engine::Components::RenderComponents::Sprite;
use crate::Engine::Components::RenderUtilities::{Indicies, PlaneVertexBuffer, Vertex};
use crate::Engine::DEBUG_MODE;

use std::sync::{Mutex, Arc};
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

    /// The Amount to offset collider from entity.
    _offset: Float3,

    // WireFrameRendering
    _display: Display<WindowSurface>,
    _indicies: NoIndices,
    _program: Program,
    _vertexBuffer: VertexBuffer<Vertex>,
    _debugSprite: Arc<Sprite>
}

impl TNewLevelClone for Option<Collider>
{
    #[inline]
    fn LevelClone(&self) -> Self
    {
        match self
        {
            Some(x) => Some(x.LevelClone()),
            None => None
         }
    }
}

impl TNewLevelClone for Collider
{
    fn LevelClone(&self) -> Self
    {
        Collider::Create(
            self._display.clone(),
            self._data.Origin(),
            self._data.Size(),
            self._data.Type(),
            self._data.Tag()
        )
    }
}

impl Collider
{
    pub fn Create(
        display: Display<WindowSurface>,
        worldPosition: Float3,
        size: Float3,
        collisionType: ECollisionType,
        tag: ECollisionTag)
        -> Self
    {
        Self
        {
            _data: ColliderData::Create(
                worldPosition,
                size,
                collisionType,
                tag
            ),
            _offset: Float3::zero(),

            _indicies: Indicies(),
            _vertexBuffer: PlaneVertexBuffer(&display),
            _program: Program::from_source(&display,
                                            DEFAULT_VERTEX_SHADER,
                                            DEFAULT_FRAGMENT_SHADER,
                                            None).unwrap(),
            _debugSprite: Sprite::new_simple("Assets/collider.png", &display),
            _display: display.clone(),

        }
    }

    pub unsafe fn Start(
        &mut self,
        entity: &EntityHeader,
        api: *mut GameAPI)
    {

        (*api).Collision.Add(entity.ID(), self._data);
        (*api).Collision.UpdateOrigin(entity.ID(), entity.WorldPosition);
    }

    pub unsafe fn Update(&mut self, entity: &EntityHeader, frame: &GameFrame, api: *mut GameAPI)
    {
        (*api).Collision.UpdateOrigin(entity.ID(), entity.WorldPosition);
    }

    pub unsafe fn OnDestroy(&mut self, entity: &EntityHeader, api: *mut GameAPI)
    {
        (*api).Collision.Remove(entity.ID());
    }

    /// Draws bounds of collider if debug mode enabled
    pub fn render(&mut self, entity: &EntityHeader, frame: &GameFrame, target: &mut Frame)
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
                [entity.WorldPosition.x() / dim.0 as f32, entity.WorldPosition.y() / dim.1 as f32, entity.WorldPosition.z(), 1.0f32],
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





