use std::ops::{Add, Div, Mul, Sub, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy)]
pub enum Facing {
    Left,
    Right,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn inew(x: i32, y: i32) -> Self {
        Self::new(x as f32, y as f32)
    }
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}
impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };        
    }
}
impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x + rhs,
            y: self.y + rhs,
        };
    }
}
impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
        }
    }
}
impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self as f32 * rhs.x,
            y: self as f32 * rhs.y,
        }
    }
}
impl MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, rhs: i32) {
        *self = Self {
            x: self.x + rhs as f32,
            y: self.y + rhs as f32,
        };
    }
}
impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x + rhs,
            y: self.y + rhs,
        };
    }
}
impl Div<i32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
        }
    }
}
impl DivAssign<i32> for Vec2 {
    fn div_assign(&mut self, rhs: i32) {
        *self = Self {
            x: self.x + rhs as f32,
            y: self.y + rhs as f32,
        };
    }
}
