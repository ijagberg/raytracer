use std::ops::{Add, Index, Mul, Sub};

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    content: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { content: [x, y, z] }
    }

    pub fn unit(vec: &Self) -> Self {
        let scalar = 1.0 / vec.len();
        Vec3::new(scalar * vec.x(), scalar * vec.y(), scalar * vec.z())
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

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            -(self.x() * other.z() - self.z() * other.x()),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn len(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn into_unit(self) -> Self {
        let scalar = 1.0 / self.len();
        self * scalar
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

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + -1 * rhs
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        *self * rhs       
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(rhs * self.x(), rhs * self.y(), rhs * self.z())
    }
}

impl Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i64) -> Self::Output {
        rhs as f64 * self
    }
}

impl Mul<Vec3> for i64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
