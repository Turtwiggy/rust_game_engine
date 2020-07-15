use renderer_gl::data::f32_f32_f32;
use cgmath::{Point3, Vector3, vec3};
use threed::camera::{Camera};

#[derive(Debug, Clone)]
pub struct GameState {
    pub game_objects : Vec<Vector3<f32>>,
    pub light_objects : Vec<Vector3<f32>>,
    pub light_colours : Vec<Vector3<f32>>,
    pub sponza_position : Vec<Vector3<f32>>
}