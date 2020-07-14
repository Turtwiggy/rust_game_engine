extern crate gl;

use renderer_gl::*;
use threed::camera::*;
use resources::Resources;
use game::{GameState};

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, vec3,  Deg, Rad, perspective};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct FGVertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    nml: data::f32_f32_f32,
    // #[location = "1"]
    // tex: data::f32_f32,
    // #[location = "1"]
    // clr: data::u2_u10_u10_u10_rev_float,
}

fn get_cube_vertices() -> [FGVertex; 36] {
    let verts: [FGVertex; 36] = [
        FGVertex{pos: (-0.5, -0.5, -0.5).into(), nml: ( 0.0,  0.0, -1.0).into()},
        FGVertex{pos: ( 0.5, -0.5, -0.5).into(), nml: ( 0.0,  0.0, -1.0).into()},
        FGVertex{pos: ( 0.5,  0.5, -0.5).into(), nml: ( 0.0,  0.0, -1.0).into()},
        FGVertex{pos: ( 0.5,  0.5, -0.5).into(), nml: ( 0.0,  0.0, -1.0).into()},
        FGVertex{pos: (-0.5,  0.5, -0.5).into(), nml: ( 0.0,  0.0, -1.0).into()},
        FGVertex{pos: (-0.5, -0.5, -0.5).into(), nml: ( 0.0,  0.0, -1.0).into()},

        FGVertex{pos: (-0.5, -0.5,  0.5).into(), nml: ( 0.0,  0.0,  1.0).into()},
        FGVertex{pos: ( 0.5, -0.5,  0.5).into(), nml: ( 0.0,  0.0,  1.0).into()},
        FGVertex{pos: ( 0.5,  0.5,  0.5).into(), nml: ( 0.0,  0.0,  1.0).into()},
        FGVertex{pos: ( 0.5,  0.5,  0.5).into(), nml: ( 0.0,  0.0,  1.0).into()},
        FGVertex{pos: (-0.5,  0.5,  0.5).into(), nml: ( 0.0,  0.0,  1.0).into()},
        FGVertex{pos: (-0.5, -0.5,  0.5).into(), nml: ( 0.0,  0.0,  1.0).into()},

        FGVertex{pos: (-0.5,  0.5,  0.5).into(), nml: (-1.0,  0.0,  0.0).into()},
        FGVertex{pos: (-0.5,  0.5, -0.5).into(), nml: (-1.0,  0.0,  0.0).into()},
        FGVertex{pos: (-0.5, -0.5, -0.5).into(), nml: (-1.0,  0.0,  0.0).into()},
        FGVertex{pos: (-0.5, -0.5, -0.5).into(), nml: (-1.0,  0.0,  0.0).into()},
        FGVertex{pos: (-0.5, -0.5,  0.5).into(), nml: (-1.0,  0.0,  0.0).into()},
        FGVertex{pos: (-0.5,  0.5,  0.5).into(), nml: (-1.0,  0.0,  0.0).into()},

        FGVertex{pos: ( 0.5,  0.5,  0.5).into(), nml: ( 1.0,  0.0,  0.0).into()},
        FGVertex{pos: ( 0.5,  0.5, -0.5).into(), nml: ( 1.0,  0.0,  0.0).into()},
        FGVertex{pos: ( 0.5, -0.5, -0.5).into(), nml: ( 1.0,  0.0,  0.0).into()},
        FGVertex{pos: ( 0.5, -0.5, -0.5).into(), nml: ( 1.0,  0.0,  0.0).into()},
        FGVertex{pos: ( 0.5, -0.5,  0.5).into(), nml: ( 1.0,  0.0,  0.0).into()},
        FGVertex{pos: ( 0.5,  0.5,  0.5).into(), nml: ( 1.0,  0.0,  0.0).into()},

        FGVertex{pos: (-0.5, -0.5, -0.5).into(), nml: ( 0.0, -1.0,  0.0).into()},
        FGVertex{pos: ( 0.5, -0.5, -0.5).into(), nml: ( 0.0, -1.0,  0.0).into()},
        FGVertex{pos: ( 0.5, -0.5,  0.5).into(), nml: ( 0.0, -1.0,  0.0).into()},
        FGVertex{pos: ( 0.5, -0.5,  0.5).into(), nml: ( 0.0, -1.0,  0.0).into()},
        FGVertex{pos: (-0.5, -0.5,  0.5).into(), nml: ( 0.0, -1.0,  0.0).into()},
        FGVertex{pos: (-0.5, -0.5, -0.5).into(), nml: ( 0.0, -1.0,  0.0).into()},

        FGVertex{pos: (-0.5,  0.5, -0.5).into(), nml: ( 0.0,  1.0,  0.0).into()},
        FGVertex{pos: ( 0.5,  0.5, -0.5).into(), nml: ( 0.0,  1.0,  0.0).into()},
        FGVertex{pos: ( 0.5,  0.5,  0.5).into(), nml: ( 0.0,  1.0,  0.0).into()},
        FGVertex{pos: ( 0.5,  0.5,  0.5).into(), nml: ( 0.0,  1.0,  0.0).into()},
        FGVertex{pos: (-0.5,  0.5,  0.5).into(), nml: ( 0.0,  1.0,  0.0).into()},
        FGVertex{pos: (-0.5,  0.5, -0.5).into(), nml: ( 0.0,  1.0,  0.0).into()},
    ];
    return verts;
}

pub fn create_renderer(gl: &gl::Gl, res: &Resources) -> Renderer {
    
    //Shaders available to the renderer
    let lit_shader = shader::Program::from_res(&gl, &res, "shaders/lit_flashlight").unwrap();
    let light_shader = shader::Program::from_res(&gl, &res, "shaders/light").unwrap();
    // shader_program.setInt(c_str!("texture1"), 0);
    
    //Model information available to the renderer
    let cube_vertices = get_cube_vertices();    

    let vbo = buffer::ArrayBuffer::new(gl);
    vbo.bind();
    vbo.static_draw_data(&cube_vertices);
    vbo.unbind();

    // set up vertex array object
    let vao = buffer::VertexArray::new(gl);
    vao.bind();
    vbo.bind();
    FGVertex::vertex_attrib_pointers(gl);
    vbo.unbind();
    vao.unbind();

    // Configure OpenGL
    // ----------------
    unsafe {
        gl.Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // gl::Disable(gl::DEPTH_TEST);
    }

    return Renderer {
        lit_shader : lit_shader,
        flat_shader : light_shader,
        vbo: vbo,
        vao: vao,
    };
}

pub struct Renderer {
    lit_shader: shader::Program,
    flat_shader: shader::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Renderer {
    pub fn render(
        &self, 
        gl: &gl::Gl, 
        window_size : (u32, u32),
        camera: &Camera,
        game_state: &GameState ) 
    {
        unsafe {
            // render window contents here
            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
        self.lit_shader.set_used();

        let view_pos = cgmath::Vector3{x: camera.Position.x, y: camera.Position.y, z: camera.Position.z};
        self.lit_shader.set_vector3(c_str!("viewPos"), &view_pos);

        //SHADER: lit_directional
        //-----------------------
        // let light_direction = cgmath::Vector3{x: -0.2, y: -1.0, z: -0.3};

        //SHADER: lit_point_noatten
        //-------------------------
        //self.lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
        // self.lit_shader.set_vector3(c_str!("light.direction"), &light_direction);

        //SHADER: lit_point_atten
        //-----------------------
        // self.lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
        // self.lit_shader.set_float(c_str!("light.constant"), 1.0);
        // self.lit_shader.set_float(c_str!("light.linear"), 0.09);
        // self.lit_shader.set_float(c_str!("light.quadratic"), 0.032);

        //SHADER: lit_flashlight
        //-----------------------
        self.lit_shader.set_float(c_str!("light.constant"), 1.0);
        self.lit_shader.set_float(c_str!("light.linear"), 0.09);
        self.lit_shader.set_float(c_str!("light.quadratic"), 0.032);

        //self.lit_shader.set_vector3(c_str!("light.position"), &game_state.light_objects[0]);
        let light_position = Vector3 {
            x: camera.Position.x,
            y: camera.Position.y,
            z: camera.Position.z
        };
        self.lit_shader.set_vector3(c_str!("light.position"), &light_position);
        self.lit_shader.set_vector3(c_str!("light.direction"), &camera.Front);
        self.lit_shader.set_float(c_str!("light.cutOff"), 12.5f32.to_radians().cos());
        self.lit_shader.set_float(c_str!("light.outerCutOff"), 17.5f32.to_radians().cos());

        //light properties
        let light_colour = Vector3 {
            // x: ((1.0 * 2.0) as f32).sin(),
            // y: ((1.0 * 0.7) as f32).sin(),
            // z: ((1.0 * 1.3) as f32).sin(),
            x: 1.0,
            y: 1.0,
            z: 1.0
        };
        let diffuse_colour = light_colour * 0.5;
        let ambient_colour = diffuse_colour * 0.2;
        self.lit_shader.set_vector3(c_str!("light.ambient"), &ambient_colour);
        self.lit_shader.set_vector3(c_str!("light.diffuse"), &diffuse_colour);
        self.lit_shader.set_vec3(c_str!("light.specular"), 1.0, 1.0, 1.0);

        //material properties
        self.lit_shader.set_vec3(c_str!("material.ambient"), 1.0, 0.5, 0.31);
        self.lit_shader.set_vec3(c_str!("material.diffuse"), 1.0, 0.5, 0.31);
        self.lit_shader.set_vec3(c_str!("material.specular"), 0.5, 0.5, 0.5);
        self.lit_shader.set_float(c_str!("material.shininess"), 32.0);

        //View Projection 
        let view = camera.GetViewMatrix();
        let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), window_size.0 as f32 / window_size.1 as f32 , 0.1, 100.0);
        self.lit_shader.set_mat4(c_str!("view"), &view);
        self.lit_shader.set_mat4(c_str!("projection"), &projection);

        // create transformations
        // NOTE: cgmath requires axis vectors to be normalized!
        // let model: Matrix4<f32> = Matrix4::from_axis_angle(
        //     vec3(0.5, 1.0, 0.0).normalize(), 
        //     Rad(1.0 as f32)
        // );
        let mut model = Matrix4::<f32>::identity();
        self.lit_shader.set_mat4(c_str!("model"), &model);
        self.vao.bind();

        //render cubes
        for (i, position) in game_state.game_objects.iter().enumerate() {
            // calculate the model matrix for each object and pass it to shader before drawing
            let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
            // let angle = 20.0 * i as f32;
            // model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
            self.lit_shader.set_mat4(c_str!("model"), &model);

            unsafe {
                gl.DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        //Also draw lamp object
        self.flat_shader.set_used();
        self.flat_shader.set_mat4(c_str!("projection"), &projection);
        self.flat_shader.set_mat4(c_str!("view"), &view);

        model = Matrix4::from_translation(game_state.light_objects[0]);
        model = model * Matrix4::from_scale(0.2);  // a smaller cube
        self.flat_shader.set_mat4(c_str!("model"), &model);
        unsafe {
            gl.DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}
