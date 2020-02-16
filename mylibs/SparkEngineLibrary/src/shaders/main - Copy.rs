extern crate glfw;
extern crate gl;
extern crate imgui;

use self::glfw::{Context, Key, Action};

use std::sync::mpsc::Receiver;

// settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "windows")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw: window creation
    // ---------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "SparkyEngine", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_polling(true);
    window.set_char_polling(true);

    // gl: load all OpenGL function pointers
    // -------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // ImGUI UI bindings
    // -----------------
    let mut imgui = ImGui::init();
    let mut imgui_glfw = ImguiGLFW::new(&mut imgui);

    // render loop
    // -----------
    while !window.should_close() {
        // input events
        // ------------
        process_events(&mut window, &events);

        // rendering commands here
        // -----------------------
        unsafe 
        {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

// NOTE: not the same version as in common.rs!
fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event 
        {
            //Window resize event
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                
                //println!("Window resized to width {} height {}", width, height);
                unsafe { gl::Viewport(0, 0, width, height) }
            }

            //ESC pressed - quit
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}