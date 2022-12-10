use super::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize,
}

impl Vector2 {
    pub fn zero() -> Self {
        Vector2::new(0, 0)
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> isize {
        if self.x.abs() > self.y.abs() {
            self.x.abs()
        } else {
            self.y.abs()
        }
    }
}

impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
