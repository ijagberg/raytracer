use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct Entity {
    t: f64,
    p: Vec3,
    normal: Vec3
}

impl Entity {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        
    }
}
