#[derive(Copy, Clone)]
pub struct Vector3
{
    position: [f32; 3],
}

impl Vector3
{
    pub fn x(& self) -> f32
    {
        self.position[0]
    }

    pub fn y(& self) -> f32
    {
        self.position[1]
    }

    pub fn z(& self) -> f32
    {
        self.position[2]
    }

    pub fn new(x: f32, y: f32, z:f32) -> Self
    {
        Vector3
        {
            position: [x, y, z]
        }
    }

    pub fn left() -> Vector3
    {
        crate::Math::Vector3::new(-1.0, 0.0, 0.0)
    }

    pub fn right() -> Vector3
    {
        crate::Math::Vector3::new(1.0, 0.0, 0.0)
    }
    pub fn up() -> Vector3
    {

        crate::Math::Vector3::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Vector3
    {
        crate::Math::Vector3::new(0.0, -1.0, 0.0)
    }

    pub fn one() -> Vector3
    {
        crate::Math::Vector3::new(1.0, 1.0, 1.0)
    }

    pub fn add(&mut self, value: Vector3)
    {
        self.position = [
            self.position[0] + value.position[0],
            self.position[1] + value.position[1],
            self.position[2] + value.position[2]
        ];
    }

    pub fn scale_vector(a: Vector3, b: Vector3) -> Vector3
    {
        Vector3
        {
            position:
            [
                a.position[0] * b.position[0],
                a.position[1] * b.position[1],
                a.position[2] * b.position[2]
            ]
        }
    }

    pub fn scale_value(vector: Vector3, value: f32) -> Vector3
    {
        Vector3
        {
            position:
            [
                vector.position[0] * value,
                vector.position[1] * value,
                vector.position[2] * value
            ]
        }
    }

    pub fn add_vectors(a: Vector3, b: Vector3) -> Vector3
    {
        Vector3
        {
            position:
            [
                a.position[0] + b.position[0],
                a.position[1] + b.position[1],
                a.position[2] + b.position[2]
            ]
        }
    }

    pub fn update(&mut self)
    {

    }
}
