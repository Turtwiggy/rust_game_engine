extern crate gl;

pub mod buffer;
pub mod shader;

// #[derive(VertexAttribPointers, Copy, Clone, Debug)]
// #[repr(C, packed)]
// struct Vertex {
//     #[location = "0"]
//     pos: data::f32_f32_f32,
//     #[location = "1"]
//     clr: data::u2_u10_u10_u10_rev_float,
// }

use gl::types::GLint;
use std::ffi::CString;

pub fn create_renderer(gl: &gl::Gl) -> Renderer {
    //A Shader
    use std::ffi::CString;
    let vert_shader = shader::Shader::from_vert_source(
        &gl,
        &CString::new(include_str!("data/shaders/triangle.vert")).unwrap(),
    )
    .unwrap();

    let frag_shader = shader::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("data/shaders/triangle.frag")).unwrap(),
    )
    .unwrap();

    let shader_program = shader::Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();

    //A square
    let vertices: Vec<f32> = vec![
        // positions
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];

    let indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];

    //VBO
    let mut vbo: u32 = 0;
    unsafe {
        gl.GenBuffers(1, &mut vbo);
    }
    unsafe {
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    //EBO
    let mut ebo: u32 = 0;
    unsafe {
        gl.GenBuffers(1, &mut ebo);
    }
    unsafe {
        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl.BufferData(
            gl::ELEMENT_ARRAY_BUFFER, // target
            (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
            indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                              // usage
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    //VAO
    let mut vao: u32 = 0;
    unsafe {
        gl.GenVertexArrays(1, &mut vao);
    }
    unsafe {
        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl.VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        gl.BindVertexArray(0);
    }

    return Renderer {
        shader_program: shader_program,
        vbo: vbo,
        ebo: ebo,
        vao: vao,
    };
}

pub struct Renderer {
    shader_program: shader::Program,
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    vao: gl::types::GLuint,
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
        unsafe {
            gl.BindVertexArray(self.vao);
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
