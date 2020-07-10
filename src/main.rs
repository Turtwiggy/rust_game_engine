extern crate cgmath;
extern crate gl;
extern crate half;
extern crate vec_2_10_10_10;
extern crate nalgebra;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate render_gl_derive;
#[macro_use]
extern crate c_string;
extern crate rand;

pub mod game_window;
pub mod resources;
use game_window::GameWindow;
pub mod renderer;
use renderer::Renderer;
pub mod renderer_gl;
use renderer_gl::Viewport;
pub mod threed;
use threed::camera::{Camera};
use cgmath::{Point3, Vector3, vec3};

use crate::resources::Resources;
use imgui::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::path::Path;
use std::time::Instant;
use rand::{thread_rng, Rng};

//Game Settings
const TARGET_FPS: u32 = 60;
const GAME_TICKS_PER_SECOND: u32 = 1;
const SECONDS_PER_GAMETICK: f32 = 1.0 / GAME_TICKS_PER_SECOND as f32;

pub struct GameState {
    game_objects : [Vector3<f32>; 10],
}

fn handle_events(
    event_pump: &mut sdl2::EventPump,
    game_window: &mut GameWindow,
    im_renderer: &Renderer,
    viewport: &mut Viewport,
) -> bool {
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

fn ui(game_window: &mut GameWindow) {
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

    game_window
        .imgui_sdl2
        .prepare_render(&ui, &game_window.sdl_window);
    game_window.imgui_renderer.render(ui);
}

fn tick(fixed_delta_time : f32, game_state : &mut GameState)
{
    let mut rng = rand::thread_rng();

    println!("ticking game state");

    //Shuffle all cube positions!
    for (i, cube_pos) in game_state.game_objects.iter_mut().enumerate() {
        let rng_x = rng.gen_range(-5.0, 5.0);
        let rng_y = rng.gen_range(-5.0, 5.0);
        let rng_z = rng.gen_range(-5.0, 5.0);
        println!("{0} {1} {2}", rng_x, rng_y, rng_z);
        *cube_pos = vec3(rng_x, rng_y, rng_z);
    }
}

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    //Settings
    let game_name: String = "Fighting Game".to_string();
    let init_width: u32 = 720;
    let init_height: u32 = 480;

    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut game_window = game_window::create_game_window(&game_name, init_width, init_height);
    let mut viewport = renderer_gl::Viewport::for_window(init_width as i32, init_height as i32);
    let renderer = renderer::create_renderer(&game_window.gl, &res);

    // world space positions of our cubes
    let cube_positions: [Vector3<f32>; 10] = [   
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5)
    ];
    let mut state_current : GameState = GameState{game_objects: cube_positions};
    let mut state_previous : GameState = GameState{game_objects: state_current.game_objects};

    let mut event_pump = game_window.sdl_context.event_pump().unwrap();
    let mut state;
    let mut last_frame = Instant::now();
    let mut seconds_since_last_gametick : f32 = 0.0;
    'running: loop {
        let ok: bool = handle_events(&mut event_pump, &mut game_window, &renderer, &mut viewport);
        if !ok {
            break 'running;
        }

        //Prepare frame
        game_window.imgui_sdl2.prepare_frame(
            game_window.imgui.io_mut(),
            &game_window.sdl_window,
            &event_pump.mouse_state(),
        );
        
        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        seconds_since_last_gametick += delta_s;
        game_window.imgui.io_mut().delta_time = delta_s;

        // Game Logic Tick - X ticks per second
        // ------------------------------------
        while seconds_since_last_gametick >= SECONDS_PER_GAMETICK
        {
            //state_previous = state_current;

            tick(SECONDS_PER_GAMETICK, &mut state_current); //this update's state_current

            seconds_since_last_gametick -= SECONDS_PER_GAMETICK;
        }

        // Camera
        // ------
        //glm::vec3 move_input = input.get_move_dir(key_state);
                // get a mouse state using mouse_state() so as not to call
        // relative_mouse_state() twice and get a false position reading
        state = event_pump.relative_mouse_state();
        camera.ProcessMouseMovement(state.x() as f32, state.y() as f32, true);
        //camera.Update(move_input, delta_time_in_seconds);

        // Rendering
        // ---------
        let position = game_window.sdl_window.position();
        let size = game_window.sdl_window.size();
        //println!("size x: {0}, y: {1}", size.0, size.1);
        renderer.render(&game_window.gl, size.0 as i32, size.1 as i32, &camera, &state_current.game_objects);
        //lerp between game states
        //const float alpha = _timeSinceLastUpdate / timePerFrame;
        //game_state state_lerped = state_current * alpha + state_previous * ( 1.0 - alpha );
        //render(window, new_state, net_set);

        ui(&mut game_window);
        game_window.sdl_window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
