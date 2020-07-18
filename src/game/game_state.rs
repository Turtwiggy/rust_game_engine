use cgmath::{Vector3, vec3};

pub fn create_default_gamestate() -> GameState
{
    let mut light_positions = Vec::new();
    light_positions.push(vec3( 0.0,  0.0,  0.0));
    light_positions.push(vec3( 1.0,  1.0,  1.0));
    light_positions.push(vec3( 5.0,  5.0,  5.0));
    light_positions.push(vec3(-2.0, -2.0, -2.0));
    light_positions.push(vec3(-6.0, -6.0, -6.0));
    let mut light_colours = Vec::new();
    light_colours.push(vec3( 1.0,  1.0,  1.0));
    light_colours.push(vec3( 1.0,  0.0,  0.0));
    light_colours.push(vec3( 0.0,  1.0,  0.0));
    light_colours.push(vec3( 0.0,  0.0,  1.0));
    light_colours.push(vec3( 0.5,  0.5,  0.5));

    let mut cube_positions = Vec::new();
    cube_positions.push(vec3(0.0, 0.0, 0.0));
    cube_positions.push(vec3(2.0, 5.0, -15.0));
    cube_positions.push(vec3(-1.5, -2.2, -2.5));
    cube_positions.push(vec3(-3.8, -2.0, -12.3));
    cube_positions.push(vec3(2.4, -0.4, -3.5));
    cube_positions.push(vec3(-1.7, 3.0, -7.5));
    cube_positions.push(vec3(1.3, -2.0, -2.5));
    cube_positions.push(vec3(1.5, 2.0, -2.5));
    cube_positions.push(vec3(1.5, 0.2, -1.5));
    cube_positions.push(vec3(-1.3, 1.0, -1.5));

    let mut sponza_position = Vec::new();
    sponza_position.push(vec3(0.0, 0.0, 0.0));

    let mut plane_position = Vec::new();
    plane_position.push(vec3(0.0, 5.0, 0.0));

    let mut grass_position = Vec::new();
    grass_position.push(vec3(0.0, 0.0, 0.0));
    grass_position.push(vec3(1.0, 0.0, 1.0));
    grass_position.push(vec3(2.0, 0.0, 2.0));
    grass_position.push(vec3(3.0, 0.0, 3.0));
    grass_position.push(vec3(4.0, 0.0, 4.0));

    let state_current: GameState = GameState {
        game_objects: cube_positions,
        light_objects: light_positions,
        light_colours: light_colours,
        sponza_position: sponza_position,
        plane_position: plane_position,
        grass_position: grass_position
    };

    state_current
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub game_objects : Vec<Vector3<f32>>,
    pub light_objects : Vec<Vector3<f32>>,
    pub light_colours : Vec<Vector3<f32>>,
    
    pub sponza_position : Vec<Vector3<f32>>,
    pub plane_position : Vec<Vector3<f32>>,
    pub grass_position : Vec<Vector3<f32>>
}