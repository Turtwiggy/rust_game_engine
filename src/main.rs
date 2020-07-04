extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

pub mod shader;
pub mod game_window;

use gl::types::GLint;
use std::ffi::CString;
use std::time::Instant;

// settings
const SCR_WIDTH: u32 = 720;
const SCR_HEIGHT: u32 = 480;

fn main() {
    //Window Init
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }

    let window = video
        .window("rust-imgui-sdl2 demo", SCR_WIDTH, SCR_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    //OpenGL
    let _gl_context = window
        .gl_create_context()
        .expect("Couldn't create GL context");
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    //ImGui + SDL2
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    /*
    //A Shader
    // let vert_shader = shader::Shader::from_vert_source(
    //     &CString::new(include_str!("data/shaders/triangle.vert")).unwrap(),
    // ).unwrap();
    // let frag_shader = shader::Shader::from_frag_source(
    //     &CString::new(include_str!("data/shaders/triangle.frag")).unwrap(),
    // ).unwrap();
    // let shader_program = shader::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    // shader_program.set_used();
    //A triangle
    // let vertices: Vec<f32> = vec![
    //     // positions      // colors
    //     0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
    //     -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
    //     0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    // ];
    // //VBO
    // let mut vbo: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenBuffers(1, &mut vbo);
    // }
    // unsafe {
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,                                                       // target
    //         (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
    //         vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
    //         gl::STATIC_DRAW,                               // usage
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    // }
    // //VAO
    // let mut vao: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    // }
    // unsafe {
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    //     gl::VertexAttribPointer(
    //         0,         // index of the generic vertex attribute ("layout (location = 0)")
    //         3,         // the number of components per generic vertex attribute
    //         gl::FLOAT, // data type
    //         gl::FALSE, // normalized (int-to-float conversion)
    //         (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
    //         std::ptr::null(),                                     // offset of the first component
    //     );
    //     gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
    //     gl::VertexAttribPointer(
    //         1,         // index of the generic vertex attribute ("layout (location = 0)")
    //         3,         // the number of components per generic vertex attribute
    //         gl::FLOAT, // data type
    //         gl::FALSE, // normalized (int-to-float conversion)
    //         (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
    //         (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid, // offset of the first component
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    //     gl::BindVertexArray(0);
    //     gl::Viewport(0, 0, SCR_WIDTH as GLint, SCR_HEIGHT as GLint); // set viewport
    // }

    */

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame = Instant::now();

    'running: loop {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());
        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        imgui.io_mut().delta_time = delta_s;
        let ui = imgui.frame();
        ui.show_demo_window(&mut true);
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(ui);
        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
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
