use std::sync::Arc;
use crate::lights::light::Light;
use crate::vec3::Color;
use crate::hittables::hittable::{HitRecord, Hittable};

pub struct LightList{
    pub(crate) lights: Vec<Arc<Light>>,
    pub(crate) ambience: Color
}
impl Clone for LightList{
    fn clone(&self) -> Self {
        return LightList{lights: self.lights.to_vec(), ambience: self.ambience.clone()};
    }
}
impl LightList{
    pub fn add(&mut self, light: Arc<Light>){
        self.lights.push(light);
    }
    pub fn get_color(&self, rec: &HitRecord, world: Arc<Hittable>, col: &mut Color){
        let mut light_change = self.ambience;
        for light in &self.lights{
            if !light.clone().is_shadow(rec, world.clone()) {
                light_change = light_change + light.clone().color;
            }
        }
        *col = *col * light_change;
    }
}