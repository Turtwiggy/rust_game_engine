extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;
extern crate half; 
extern crate nalgebra;
extern crate cgmath;
extern crate vec_2_10_10_10;
#[macro_use] 
extern crate failure;
#[macro_use]
extern crate render_gl_derive;
#[macro_use]
extern crate c_string;

pub mod resources;
pub mod game_window;
use game_window::GameWindow;
pub mod renderer;
use renderer::Renderer;
pub mod renderer_gl;
use renderer_gl::Viewport;
pub mod threed;
use threed::camera::{Camera, Point3};

use crate::resources::Resources;
use imgui::*;
use std::time::Instant;
use std::path::Path;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;

//Game Settings
const TARGET_FPS: u32 = 60;
const GAME_TICKS_PER_SECOND: u32 = 1;
const SECONDS_PER_GAMETICK: f32 = 1.0 / GAME_TICKS_PER_SECOND as f32;

fn handle_events(
    event_pump: &mut sdl2::EventPump, 
    game_window: &mut GameWindow,
    im_renderer : &Renderer,
    viewport : &mut Viewport ) -> bool {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    for event in event_pump.poll_iter() {

        game_window
            .imgui_sdl2
            .handle_event(&mut game_window.imgui, &event);

        if game_window.imgui_sdl2.ignore_event(&event) {
            continue;
        }
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            } => {
                println!("o pressed!");
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                let is_fullscreen: bool = game_window.is_fullscreen();
                game_window.set_fullscreen(!is_fullscreen);

                let (width, height) = game_window.get_width_and_height();
                let (x, y) = game_window.get_position();
                println!("width: {0} height: {1}", width, height);

                viewport.update_size(width as i32, height as i32);
                viewport.set_used(&game_window.gl);
            }
            sdl2::event::Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(w, h),
                ..
            } => {
                viewport.update_size(w, h);
                viewport.set_used(&game_window.gl);
            }
            _ => {}
        }
    }
    return true;
}

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let init_width : u32 = 720;
    let init_height : u32 = 480;

    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut firstMouse = true;
    let mut lastX: f32 = init_width as f32 / 2.0;
    let mut lastY: f32 = init_height as f32 / 2.0;

    let game_name: String = "Fighting Game".to_string();
    let mut game_window = game_window::create_game_window(&game_name, init_width, init_height);
    let mut viewport = renderer_gl::Viewport::for_window(init_width as i32, init_height as i32);

    let renderer = renderer::create_renderer(&game_window.gl, &res);

    let mut event_pump = game_window.sdl_context.event_pump().unwrap();
    let mut state;
    let mut last_frame = Instant::now();
    'running: loop {

        let ok : bool = handle_events(&mut event_pump, &mut game_window, &renderer, &mut viewport);
        if !ok { break 'running; }

        game_window.imgui_sdl2.prepare_frame(
            game_window.imgui.io_mut(),
            &game_window.sdl_window,
            &event_pump.mouse_state(),
        );

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        game_window.imgui.io_mut().delta_time = delta_s;

        // get a mouse state using mouse_state() so as not to call
        // relative_mouse_state() twice and get a false position reading
        state = event_pump.relative_mouse_state();
        camera.ProcessMouseMovement(state.x() as f32, state.y() as f32, true);

        let ui = game_window.imgui.frame();
        //ui.show_demo_window(&mut true);

        //Seperate Window
        Window::new(im_str!("Hello world"))
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(&ui, || {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });

        //Graphics
        let position = game_window.sdl_window.position();
        let size = game_window.sdl_window.size();
        println!("size x: {0}, y: {1}", size.0, size.1);
        renderer.render(&game_window.gl, size.0 as i32, size.1 as i32, &camera);

        //UI
        game_window
            .imgui_sdl2
            .prepare_render(&ui, &game_window.sdl_window);
        game_window.imgui_renderer.render(ui);
        
        game_window.sdl_window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
