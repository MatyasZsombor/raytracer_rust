use std::fmt::{Display, Formatter};
use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec3
{
    e : [f32; 3],
}

impl Vec3
{
    pub fn new_zero() -> Self
    {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }

    pub fn new(e0 : f32, e1 : f32, e2 : f32) -> Self
    {
        Vec3 {e: [e0, e1, e2]}
    }

    pub fn x(self) -> f32
    {
        self.e[0]
    }

    pub fn y(self) -> f32
    {
        self.e[1]
    }

    pub fn z(self) -> f32
    {
        self.e[2]
    }

    pub fn length(self) -> f32
    {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32
    {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn normalize(self) -> Vec3
    {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f32 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[2] * v.e[3] - self.e[3] * v.e[2],
            self.e[3] * v.e[0] - self.e[0] * v.e[3],
            self.e[0] * v.e[1] - self.e[1] * v.e[0],
        )
    }
}

impl Display for Vec3
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Index<usize> for Vec3
{
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2])
    }
}

impl Mul<f32> for Vec3
{
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Div<f32> for Vec3
{
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}