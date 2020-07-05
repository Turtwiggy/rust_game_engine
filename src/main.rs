extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

pub mod game_window;
use game_window::GameWindow;
pub mod renderer;
use renderer::Renderer;
pub mod resources;

use imgui::*;
use std::time::Instant;

use crate::resources::Resources;
use std::path::Path;

//Game Settings
const TARGET_FPS: u32 = 60;
const GAME_TICKS_PER_SECOND: u32 = 1;
const SECONDS_PER_GAMETICK: f32 = 1.0 / GAME_TICKS_PER_SECOND as f32;

fn handle_events(
    event_pump: &mut sdl2::EventPump, 
    game_window: &mut GameWindow,
    renderer : &Renderer ) -> bool {
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
                println!("width: {0} height: {1}", width, height);
                renderer.set_viewport(&game_window.gl, width as i32, height as i32);
            }
            sdl2::event::Event::Window {
                win_event: sdl2::event::WindowEvent::Resized(w, h),
                ..
            } => {
                renderer.set_viewport(&game_window.gl, w, h);
            },
            _ => {}
        }
    }
    return true;
}

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    let game_name: String = "Fighting Game".to_string();
    let mut game_window = game_window::create_game_window(&game_name, 720, 480);

    let renderer = renderer::create_renderer(&game_window.gl, &res);

    let mut event_pump = game_window.sdl_context.event_pump().unwrap();
    let mut last_frame = Instant::now();
    'running: loop {

        let ok : bool = handle_events(&mut event_pump, &mut game_window, &renderer);
        if !ok { break 'running; }

        let position = game_window.sdl_window.position();
        let size = game_window.sdl_window.size();
        println!("size x: {0}, y: {1}", size.0, size.1);
        renderer.set_viewport(&game_window.gl, size.0 as i32, size.1 as i32);

        game_window.imgui_sdl2.prepare_frame(
            game_window.imgui.io_mut(),
            &game_window.sdl_window,
            &event_pump.mouse_state(),
        );

        // // get a mouse state using mouse_state() so as not to call
        // // relative_mouse_state() twice and get a false position reading
        // if events.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
        //     state = events.relative_mouse_state();
        //     println!("Relative - X = {:?}, Y = {:?}", state.x(), state.y());
        // }

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        game_window.imgui.io_mut().delta_time = delta_s;

        let ui = game_window.imgui.frame();
        ui.show_demo_window(&mut true);

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
        renderer.render(&game_window.gl);

        //UI
        game_window
            .imgui_sdl2
            .prepare_render(&ui, &game_window.sdl_window);
        game_window.renderer.render(ui);
        
        game_window.sdl_window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
