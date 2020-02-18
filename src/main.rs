extern crate spark_engine;
//use SparkEngineLibrary::SparkCore;

extern crate gl;
extern crate glfw;

use glfw::{Action, Context, JoystickId, Key};

use spark_engine::engine::shaders::render_gl;

use std::ffi::CString;
use std::time::Instant;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    #[cfg(target_os = "windows")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "SparkyEngine",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_polling(true);
    window.set_char_polling(true);
    //window.set_cursor_pos_polling(should_poll: bool)

    gl::load_with(|symbol| window.get_proc_address(symbol));

    unsafe {
        gl::ClearColor(0.3, 0.3, 1.0, 1.0);
        gl::Viewport(0, 0, 800, 600);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Disable(gl::DEPTH_TEST);
    }

    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("data/shaders/triangle.vert")).unwrap(),
    )
    .unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("data/shaders/triangle.frag")).unwrap(),
    )
    .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vertices: Vec<f32> = vec![
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0,
    ];
    let indices: Vec<i32> = vec![0, 1, 3, 1, 2, 3];

    let mut VBO: gl::types::GLuint = 0;
    let mut VAO: gl::types::GLuint = 0;
    let mut EBO: gl::types::GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);
        gl::BindVertexArray(VAO);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, // target
            (indices.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr, // size of data in bytes
            indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                              // usage
        );

        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        gl::BindVertexArray(0);
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    let mut last_frame: Instant = Instant::now();
    let mut r: f32 = 0.0;
    let mut increasing = true;
    let desired_seconds_for_chanage = 5.0;

    while !window.should_close() {
        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        let joystick = glfw.get_joystick(JoystickId::Joystick1);
        println!("Joystick enabled: {0}", joystick.is_present());

        // retrieve some necessary information from glfw
        let (width, height) = window.get_size();
        let mouse_pos = {
            let (x, y) = window.get_cursor_pos();
            (x as i32, height - (y as i32))
        };
        let mouse_btn_state = window.get_mouse_button(glfw::MouseButtonLeft);
        let mouse_scroll = {
            // emulate mouse scroll with up and down key
            if window.get_key(Key::Up) == Action::Press {
                -1
            } else if window.get_key(Key::Down) == Action::Press {
                1
            } else {
                0
            }
        };

        //Clear GL buffer
        // clear the screen and set the viewport to the window size
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, width, height);
            gl::ClearColor(r, 0.3, 1.0, 1.0);
        }

        //change colour
        if increasing {
            r += delta_s / desired_seconds_for_chanage;
            if r >= 1.0 {
                increasing = false;
            }
        } else {
            r -= delta_s / desired_seconds_for_chanage;
            if r <= 0.0 {
                r = 0.0;
                increasing = true;
            }
        }

        //Draw our triangles
        shader_program.set_used();

        unsafe {
            gl::BindVertexArray(VAO);
            // gl::DrawArrays(
            //     gl::TRIANGLES, // mode
            //     0,             // starting index in the enabled arrays
            //     6,             // number of indices to be rendered
            // );
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        //Window resize event
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dimensions; note that width and
            // height will be significantly larger than specified on retina displays.
            //println!("Window resized to width {} height {}", width, height);
            unsafe { gl::Viewport(0, 0, width, height) }
        }

        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
