use std::ops::{Add, Index};

pub struct Vec3 {
    content: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { content: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.content[0]
    }

    pub fn y(&self) -> f64 {
        self.content[1]
    }

    pub fn z(&self) -> f64 {
        self.content[2]
    }

    pub fn len(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}
