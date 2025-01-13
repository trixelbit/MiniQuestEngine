use crate::Engine::Components::RenderComponents::Renderer2D;
use crate::Engine::Math::Float3;

pub struct Renderer
{
    _light: Vec<LightingData>,
    _renderers: Vec<Renderer2D>
}

impl Renderer
{
    pub fn Renderer(&mut self)
    {

    }

    fn SortZ(&mut self)
    {

    }

    fn Draw()
    {

    }
}

pub struct LightingData
{
    pub Position : Float3,
    pub Color : Float3,
    pub Intensity : f32
}




