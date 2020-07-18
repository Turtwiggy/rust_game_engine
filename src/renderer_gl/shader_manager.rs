
use util::resources::Resources;
use renderer_gl::shader::*;
use renderer_gl::*;

pub fn create_default(gl : &gl::Gl, res: &Resources ) -> ShaderManager {

    let mut shader_manager = ShaderManager {
        shaders: Vec::new(),
    };

    //Add some default shaders
    shader_manager.add_shader(gl, res, "shaders/lit_multiple_lights_no_tex");
    shader_manager.add_shader(gl, res, "shaders/lit_directional");
    shader_manager.add_shader(gl, res, "shaders/stencil_border");
    shader_manager.add_shader(gl, res, "shaders/skybox");    
    //skybox_shader.set_int(c_str!("skybox"), 0);

    shader_manager
}

pub struct ShaderManager {
        shaders : Vec<(String, shader::Program)>
}

impl ShaderManager {
    fn add_shader(&mut self, gl: &gl::Gl, res: &Resources, name: &str) {

        let shader = shader::Program::from_res(&gl, &res, name).unwrap();

        if !self.shaders.iter().any(|(l, r)| l == &name.to_string()) {
            println!("(ShaderManager) shader {0} not loaded - loading now", name.to_string());
            self.shaders.push((name.to_string(), shader));
        }

        println!("(ShaderManager) Loaded shaders: {0}", self.shaders.len() );
    }

    pub fn get_shader(&self, name: &str) -> Result<&shader::Program, String> {

        for i in 0..self.shaders.len() {

            if self.shaders[i].0 == name.to_string() {
                return Ok(&self.shaders[i].1);
            }
        }

        println!("never found the shader {0}", name);
        Err(format!("Never found the shader {}", name).to_string())
    }
}

//SHADER: lit_directional
//-----------------------
// let light_direction = cgmath::Vector3{x: -0.2, y: -1.0, z: -0.3};

//SHADER: lit_point_noatten
//-------------------------
//lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
// lit_shader.set_vector3(c_str!("light.direction"), &light_direction);

//SHADER: lit_point_atten
//-----------------------
// lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
// lit_shader.set_float(c_str!("light.constant"), 1.0);
// lit_shader.set_float(c_str!("light.linear"), 0.09);
// lit_shader.set_float(c_str!("light.quadratic"), 0.032);

//SHADER: lit_flashlight
//-----------------------
// lit_shader.set_float(c_str!("light.constant"), 1.0);
// lit_shader.set_float(c_str!("light.linear"), 0.09);
// lit_shader.set_float(c_str!("light.quadratic"), 0.032);
// //lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
// let light_position = Vector3 {
//     x: camera.Position.x,
//     y: camera.Position.y,
//     z: camera.Position.z
// };
// lit_shader.set_vector3(c_str!("light.position"), &light_position);
// lit_shader.set_vector3(c_str!("light.direction"), &camera.Front);
// lit_shader.set_float(c_str!("light.cutOff"), 12.5f32.to_radians().cos());
// lit_shader.set_float(c_str!("light.outerCutOff"), 17.5f32.to_radians().cos());
// //light properties
// let light_colour = Vector3 {
//     // x: ((1.0 * 2.0) as f32).sin(),
//     // y: ((1.0 * 0.7) as f32).sin(),
//     // z: ((1.0 * 1.3) as f32).sin(),
//     x: 1.0,
//     y: 1.0,
//     z: 1.0
// };
// let diffuse_colour = light_colour * 0.5;
// let ambient_colour = diffuse_colour * 0.2;
// lit_shader.set_vector3(c_str!("light.ambient"), &ambient_colour);
// lit_shader.set_vector3(c_str!("light.diffuse"), &diffuse_colour);
// lit_shader.set_vec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

//SHADER: lit_mutiple_lights_no_tex
//---------------------------------
/*
Here we set all the uniforms for the 5/6 types of lights we have. We have to set them manually and index
the proper PointLight struct in the array to set each uniform variable. This can be done more code-friendly
by defining light types as classes and set their values in there, or by using a more efficient uniform approach
by using 'Uniform buffer objects', but that is something we'll discuss in the 'Advanced GLSL' tutorial.
*/