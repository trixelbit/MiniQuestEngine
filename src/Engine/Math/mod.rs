use std::ops::{Add, Mul, Sub};
use cgmath::{Point3, Vector3};
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(Copy, Clone)]
pub struct Float3
{
    position: [f32; 3],
}

impl Float3
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
        Float3
        {
            position: [x, y, z]
        }
    }

    pub fn FromString(string : &str) -> Float3
    {
        let a = string.replace(" ", "");
        let mut tokens = a
            .split(",")
            .into_iter();

        let x: f32 = tokens.next().unwrap().parse().unwrap();
        let y: f32 = tokens.next().unwrap().parse().unwrap();
        let z: f32 = tokens.next().unwrap().parse().unwrap();

        Float3::new(x,y,z)
    }

    pub fn left() -> Float3
    {
        crate::Engine::Math::Float3::new(-1.0, 0.0, 0.0)
    }

    pub fn right() -> Float3
    {
        crate::Engine::Math::Float3::new(1.0, 0.0, 0.0)
    }

    pub fn ToCGPoint(&self) -> Point3<f32>
    {
        Point3::new(self.x(), self.y(), self.z())
    }

    pub fn ToCGVector(&self) -> Vector3<f32>
    {
        Vector3::new(self.x(), self.y(), self.z())
    }

    pub fn up() -> Float3
    {
        Float3::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Float3
    {
        Float3::new(0.0, -1.0, 0.0)
    }

    pub fn one() -> Float3
    {
        Float3::new(1.0, 1.0, 1.0)
    }


    pub fn forward() -> Float3
    {
        Float3::new(0.0, 0.0, 1.0)
    }

    pub fn zero() -> Float3
    {
        Float3::new(0.0, 0.0, 0.0)
    }

    pub fn add(&mut self, value: Float3) -> Self
    {
        self.position = [
            self.position[0] + value.position[0],
            self.position[1] + value.position[1],
            self.position[2] + value.position[2]
        ];
        self.clone()
    }

    pub fn ScaleX(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0] * value,
            self.position[1],
            self.position[2]
        ];

        self.clone()
    }
    pub fn ScaleY(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0],
            self.position[1] * value,
            self.position[2]
        ];
        self.clone()
    }

    pub fn ScaleZ(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0],
            self.position[1],
            self.position[2] * value
        ];
        self.clone()
    }

    pub fn OverrideX(&mut self, value: f32) -> Self
    {
        self.position = [
            value,
            self.position[1],
            self.position[2]
        ];
        self.clone()
    }

    pub fn OverrideY(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0],
            value,
            self.position[2]
        ];
        self.clone()
    }

    pub fn OverrideZ(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0],
            self.position[1],
            value
        ];
        self.clone()

    }

    pub fn AddX(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0] + value,
            self.position[1],
            self.position[2]
        ];
        self.clone()
    }

    pub fn AddY(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0],
            self.position[1] + value,
            self.position[2]
        ];
        self.clone()
    }

    pub fn AddZ(&mut self, value: f32) -> Self
    {
        self.position = [
            self.position[0],
            self.position[1],
            self.position[2] + value
        ];
        self.clone()

    }

    pub fn magnitude(&self) -> f32
    {
        return
            (
                (self.x() * self.x()) +
                (self.y() * self.y()) +
                (self.z() * self.z())
            ).sqrt()
    }

    /// Returns the normalized version of the current vector.
    pub fn normalized(&self) -> Float3
    {
        let magnitude = self.magnitude();

        if magnitude <= 0.0
        {
            return Float3::zero();
        }

        Float3
        {
            position: [
                self.x() / magnitude,
                self.y() / magnitude,
                self.z() / magnitude
            ]
        }
    }

    pub fn scale_vector(a: Float3, b: Float3) -> Float3
    {
        Float3
        {
            position:
            [
                a.position[0] * b.position[0],
                a.position[1] * b.position[1],
                a.position[2] * b.position[2]
            ]
        }
    }

    pub fn scale_value(vector: Float3, value: f32) -> Float3
    {
        Float3
        {
            position:
            [
                vector.position[0] * value,
                vector.position[1] * value,
                vector.position[2] * value
            ]
        }
    }

    pub fn add_vectors(a: Float3, b: Float3) -> Float3
    {
        Float3
        {
            position:
            [
                a.position[0] + b.position[0],
                a.position[1] + b.position[1],
                a.position[2] + b.position[2]
            ]
        }
    }


    pub fn Lerp(start: Float3, end: Float3, t: f32) -> Float3
    {
        Float3::new(
            Math::Lerp(start.x(), end.x(), t),
            Math::Lerp(start.y(), end.y(), t),
            Math::Lerp(start.z(), end.z(), t)
        )
    }

    pub fn update(&mut self)
    {

    }
}

impl Display for Float3
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    {
        write!(f, "({}, {}, {})", self.position[0], self.position[1], self.position[2])
    }
}

impl Sub for Float3
{
    type Output = Self;

    fn sub(self, other : Self) -> Self
    {
        Self
        {
            position:
            [
                self.position[0] - other.position[0],
                self.position[1] - other.position[1],
                self.position[2] - other.position[2]
            ]
        }
    }
}

impl Add for Float3
{
    type Output = Self;

    fn add(self, other : Self) -> Self
    {
        Self
        {
            position:
            [
                self.position[0] + other.position[0],
                self.position[1] + other.position[1],
                self.position[2] + other.position[2]
            ]
        }
    }
}
impl Mul for Float3{
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        Self
        {

            position:
            [
                self.position[0] * other.position[0],
                self.position[1] * other.position[1],
                self.position[2] * other.position[2]
            ]
        }
    }
}

pub mod Math
{
    pub fn Lerp(start: f32, end: f32, t: f32) -> f32
    {
        return ((end - start) * t) + start
    }
}

#[derive(Copy, Clone)]
pub struct Ray
{
    pub Origin: Float3,
    pub Direction: Float3
}

impl Ray
{
    pub fn Create(origin: Float3, direction: Float3) -> Self
    {
        Self
        {
            Origin: origin,
            Direction: direction
        }
    }
}
