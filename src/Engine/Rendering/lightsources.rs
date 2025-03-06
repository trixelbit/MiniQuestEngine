use crate::Engine::Math::Float3;


pub struct PointLight
{
    pub Color : Float3,
    pub Intensity : f32,
    pub Position : Float3,
    pub Radius : f32
}

pub struct DirectionalLight
{
    pub Color : Float3,
    pub Intensity : f32,
    pub Direction : Float3
}
