use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn point_at(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
