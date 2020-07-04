extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

pub mod game_window;
pub mod shader;

use game_window::GameWindow;
use gl::types::GLint;
use std::ffi::CString;
use std::time::Instant;

fn main() {
    let game_name = "Fighting Game";
    let game_window = game_window::create_game_window(game_name, 720, 480);
    let (width, height) = game_window.get_width_and_height();
    println!("width: {0} height: {1}", width, height);

    let mut imgui_sdl2 = game_window.imgui_sdl2;
    let mut imgui = game_window.imgui;
    let window = game_window.sdl_window;
    let sdl_context = game_window.sdl_context;
    let renderer = game_window.renderer;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame = Instant::now();
    'running: loop {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) { continue; }
        
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                break 'running
                },
                Event::KeyDown{keycode:Some(Keycode::O), ..} => {
                    println!("o pressed!");
                }
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
