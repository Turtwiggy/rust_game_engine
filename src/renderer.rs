extern crate gl;

pub mod buffer;
pub mod shader;
pub mod data;

use render_gl_derive::*;
use resources::Resources;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub fn create_renderer(gl: &gl::Gl, res: &Resources) -> Renderer {
    
    let shader_program = shader::Program::from_res(&gl, &res, "shaders/triangle").unwrap();

    let vertices: Vec<Vertex> = vec![
        Vertex {
            pos: (0.5, -0.5, 0.0).into(),
            clr: (1.0, 0.0, 0.0, 1.0).into(),
        }, // bottom right
        Vertex {
            pos: (-0.5, -0.5, 0.0).into(),
            clr: (0.0, 1.0, 0.0, 1.0).into(),
        }, // bottom left
        Vertex {
            pos: (0.0, 0.5, 0.0).into(),
            clr: (0.0, 0.0, 1.0, 1.0).into(),
        }, // top
    ];

    let vbo = buffer::ArrayBuffer::new(gl);
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    // set up vertex array object
    let vao = buffer::VertexArray::new(gl);
    vao.bind();
    vbo.bind();
    Vertex::vertex_attrib_pointers(gl);
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
    vao: buffer::VertexArray
}

impl Renderer {
    pub fn render(&self, gl: &gl::Gl) {
        unsafe {
            // render window contents here
            gl.ClearColor(0.2, 0.3, 0.3, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // gl::Disable(gl::DEPTH_TEST);

        self.shader_program.set_used();
        self.vao.bind();
        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }
    }

    pub fn set_viewport(&self, gl: &gl::Gl, width: i32, height: i32) {
        use gl::types::GLint;
        println!("setting viewport: {0}, {1}", width, height);
        unsafe {
            gl.Viewport(0, 0, width as GLint, height as GLint);
        }
    }
}
