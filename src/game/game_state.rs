use cgmath::{Point3, Vector3, vec3};
use threed::camera::{Camera};

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub game_objects : [Vector3<f32>; 10],
    pub light_objects : [Vector3<f32>; 4],
    pub light_colours : [Vector3<f32>; 4],
}