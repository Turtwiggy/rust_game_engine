extern crate gl;
extern crate sdl2;

pub mod shader;

use gl::types::GLint;
use std::ffi::CString;
use std::time::Instant;

// settings
const SCR_WIDTH: u32 = 720;
const SCR_WIDTH_GL: GLint = SCR_WIDTH as GLint;
const SCR_HEIGHT: u32 = 480;
const SCR_HEIGHT_GL: GLint = SCR_HEIGHT as GLint;

fn main() {
    //Window Init
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 1);

    let window = video_subsystem
        .window("Game", SCR_WIDTH, SCR_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    //OpenGL
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //A Shader
    let vert_shader = shader::Shader::from_vert_source(
        &CString::new(include_str!("data/shaders/triangle.vert")).unwrap(),
    )
    .unwrap();
    let frag_shader = shader::Shader::from_frag_source(
        &CString::new(include_str!("data/shaders/triangle.frag")).unwrap(),
    )
    .unwrap();
    let shader_program = shader::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    //A triangle
    let vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    //VBO
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    //VAO
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    //Viewport
    unsafe {
        gl::Viewport(0, 0, SCR_WIDTH_GL, SCR_HEIGHT_GL); // set viewport
    }

    let mut last_frame: Instant = Instant::now();
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        shader_program.set_used();
        render(vao);

        window.gl_swap_window();
    }
}

fn render(vao: gl::types::GLuint) {
    // render window contents here
    unsafe {
        gl::ClearColor(0.2, 1.0, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        // gl::Enable(gl::BLEND);
        // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // gl::Disable(gl::DEPTH_TEST);

        gl::BindVertexArray(vao);
        gl::DrawArrays(
            gl::TRIANGLES, // mode
            0,             // starting index in the enabled arrays
            3,             // number of indices to be rendered
        );
    }
}