extern crate gl;

use renderer_gl::*;
use threed::camera::*;
use resources::Resources;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3, vec3,  Deg, Rad, perspective};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct FGVertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    tex: data::f32_f32,
    // #[location = "1"]
    // clr: data::u2_u10_u10_u10_rev_float,
}

pub fn create_renderer(gl: &gl::Gl, res: &Resources) -> Renderer {
    
    let shader_program = shader::Program::from_res(&gl, &res, "shaders/triangle").unwrap();
    // shader_program.setInt(c_str!("texture1"), 0);
    // shader_program.setInt(c_str!("texture2"), 1);

    let CUBE_VERTICES: [FGVertex; 36] = [
        FGVertex{ pos: ( -0.5, -0.5, -0.5).into(), tex: ( 0.0, 0.0,).into()},
        FGVertex{ pos: (  0.5, -0.5, -0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: (  0.5,  0.5, -0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: (  0.5,  0.5, -0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: ( -0.5,  0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: ( -0.5, -0.5, -0.5).into(), tex: ( 0.0, 0.0,).into()},

        FGVertex{ pos: ( -0.5, -0.5,  0.5).into(), tex: ( 0.0, 0.0,).into()},
        FGVertex{ pos: (  0.5, -0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: (  0.5,  0.5,  0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: (  0.5,  0.5,  0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: ( -0.5,  0.5,  0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: ( -0.5, -0.5,  0.5).into(), tex: ( 0.0, 0.0,).into()},

        FGVertex{ pos: ( -0.5,  0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: ( -0.5,  0.5, -0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: ( -0.5, -0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: ( -0.5, -0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: ( -0.5, -0.5,  0.5).into(), tex: ( 0.0, 0.0,).into()},
        FGVertex{ pos: ( -0.5,  0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},

        FGVertex{ pos: (  0.5,  0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: (  0.5,  0.5, -0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: (  0.5, -0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: (  0.5, -0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: (  0.5, -0.5,  0.5).into(), tex: ( 0.0, 0.0,).into()},
        FGVertex{ pos: (  0.5,  0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},

        FGVertex{ pos: ( -0.5, -0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: (  0.5, -0.5, -0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: (  0.5, -0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: (  0.5, -0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: ( -0.5, -0.5,  0.5).into(), tex: ( 0.0, 0.0,).into()},
        FGVertex{ pos: ( -0.5, -0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},

        FGVertex{ pos: ( -0.5,  0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
        FGVertex{ pos: (  0.5,  0.5, -0.5).into(), tex: ( 1.0, 1.0,).into()},
        FGVertex{ pos: (  0.5,  0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: (  0.5,  0.5,  0.5).into(), tex: ( 1.0, 0.0,).into()},
        FGVertex{ pos: ( -0.5,  0.5,  0.5).into(), tex: ( 0.0, 0.0,).into()},
        FGVertex{ pos: ( -0.5,  0.5, -0.5).into(), tex: ( 0.0, 1.0,).into()},
    ];

    let vbo = buffer::ArrayBuffer::new(gl);
    vbo.bind();
    vbo.static_draw_data(&CUBE_VERTICES);
    vbo.unbind();

    // set up vertex array object
    let vao = buffer::VertexArray::new(gl);
    vao.bind();
    vbo.bind();
    FGVertex::vertex_attrib_pointers(gl);
    vbo.unbind();
    vao.unbind();

    return Renderer {
        shader_program: shader_program,
        vbo: vbo,
        vao: vao,
    };
}

pub struct Renderer {
    shader_program: shader::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Renderer {
    pub fn render(
        &self, 
        gl: &gl::Gl, 
        // camera: 
        width : i32, 
        height : i32,
        camera: &Camera,
        cube_positions : &[Vector3<f32>; 10]) 
    {
        
        unsafe {
            // render window contents here
            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // gl::Disable(gl::DEPTH_TEST);

        self.shader_program.set_used();

        unsafe {

            // create transformations
            // NOTE: cgmath requires axis vectors to be normalized!
            let model: Matrix4<f32> = Matrix4::from_axis_angle(
                vec3(0.5, 1.0, 0.0).normalize(), 
                Rad(1.0 as f32)
            );
            let view = camera.GetViewMatrix();
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), width as f32 / height as f32 , 0.1, 100.0);

            // note: currently we set the projection matrix each frame, but since the projection matrix rarely changes it's often best practice to set it outside the main loop only once.
            self.shader_program.set_mat4(c_str!("view"), &view);
            self.shader_program.set_mat4(c_str!("projection"), &projection);

            self.vao.bind();
            for (i, position) in cube_positions.iter().enumerate() {
                // calculate the model matrix for each object and pass it to shader before drawing
                let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                let angle = 20.0 * i as f32;
                model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
                self.shader_program.set_mat4(c_str!("model"), &model);

                gl.DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    }
}
