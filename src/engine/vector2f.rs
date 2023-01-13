use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2f {
    x: f32,
    y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f { x, y }
    }
    pub fn inew(x: i32, y: i32) -> Vec2f {
        Vec2f { x: x as f32, y: y as f32 }
    }
    pub fn zero() -> Vec2f{
        Vec2f::inew(0, 0)
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
}

impl ops::Add<Vec2f> for Vec2f{
    type Output = Self;

    fn add(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vec2f> for Vec2f{
    fn add_assign(&mut self, rhs: Vec2f) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vec2f> for Vec2f{
    type Output = Self;

    fn sub(self, rhs: Vec2f) -> Self::Output {
        Vec2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Vec2f> for Vec2f{
    fn sub_assign(&mut self, rhs: Vec2f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

