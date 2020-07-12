extern crate gl;
extern crate half;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate nalgebra;
extern crate sdl2;
extern crate vec_2_10_10_10;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate render_gl_derive;
#[macro_use]
extern crate c_string;
extern crate cgmath;
extern crate rand;

pub mod resources;
use resources::Resources;
pub mod game_window;
use game_window::GameWindow;
pub mod renderer;
use renderer::Renderer;
pub mod renderer_gl;
use renderer_gl::Viewport;
pub mod threed;
use threed::camera::{Camera, CameraMovement};
pub mod game;
use game::GameState;

use cgmath::{vec3, Point3, Vector3};
use imgui::*;
use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::path::Path;
use std::time::Instant;

//Game Settings
const TARGET_FPS: u32 = 144;
const GAME_TICKS_PER_SECOND: u32 = 1;
const SECONDS_PER_GAMETICK: f32 = 1.0 / GAME_TICKS_PER_SECOND as f32;

fn process_events(
    game_window: &mut GameWindow,
    event: &sdl2::event::Event,
    viewport: &mut Viewport,
) {
    match event {
        sdl2::event::Event::Window {
            win_event: sdl2::event::WindowEvent::Resized(w, h),
            ..
        } => {
            println!("updating viewport");
            viewport.update_size(*w, *h);
            viewport.set_used(&game_window.gl);
        }
        _ => {}
    }
}

fn process_input(
    game_window: &mut GameWindow,
    event: &sdl2::event::Event,
    camera: &mut Camera,
) -> bool {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            return false;
        }
        Event::KeyDown {
            keycode: Some(Keycode::F),
            ..
        } => {
            let is_fullscreen: bool = game_window.is_fullscreen();
            game_window.set_fullscreen(!is_fullscreen);
        }
        Event::KeyDown {
            keycode: Some(Keycode::M),
            ..
        } => {
            println!("trying to capture mouse");
            game_window.toggle_grabbed();
        }
        Event::KeyDown{
            keycode: Some(Keycode::N),
            ..
        } => {
            game_window.capture_mouse(true)
        }
        Event::KeyDown {
            keycode: Some(Keycode::O),
            ..
        } => {
            println!("o pressed!");
        }
        _ => {}
    }

    match_camera_event(camera, event);
    return true;
}

fn match_camera_event(camera : &mut Camera, event: &sdl2::event::Event) {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::Space),
            ..
        } => {
            camera.ProcessKeyboardDown(CameraMovement::UP);
        }
        Event::KeyUp {
            keycode: Some(Keycode::Space),
            ..
        } => {
            camera.ProcessKeyboardUp(CameraMovement::UP);
        }
        Event::KeyDown {
            keycode: Some(Keycode::LShift),
            ..
        } => {
            camera.ProcessKeyboardDown(CameraMovement::DOWN);
        }
        Event::KeyUp {
            keycode: Some(Keycode::LShift),
            ..
        } => {
            camera.ProcessKeyboardUp(CameraMovement::DOWN);
        }
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => {
            //println!("w pressed!");
            camera.ProcessKeyboardDown(CameraMovement::FORWARD);
        }
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => {
            //println!("s pressed!");
            camera.ProcessKeyboardDown(CameraMovement::BACKWARD);
        }
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => {
            //println!("a pressed!");
            camera.ProcessKeyboardDown(CameraMovement::LEFT);
        }
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => {
            //println!("d pressed!");
            camera.ProcessKeyboardDown(CameraMovement::RIGHT);
        }
        Event::KeyUp {
            keycode: Some(Keycode::W),
            ..
        } => {
            println!("w released!");
            camera.ProcessKeyboardUp(CameraMovement::FORWARD);
        }
        Event::KeyUp {
            keycode: Some(Keycode::S),
            ..
        } => {
            println!("s released!");
            camera.ProcessKeyboardUp(CameraMovement::BACKWARD);
        }
        Event::KeyUp {
            keycode: Some(Keycode::A),
            ..
        } => {
            println!("a released!");
            camera.ProcessKeyboardUp(CameraMovement::LEFT);
        }
        Event::KeyUp {
            keycode: Some(Keycode::D),
            ..
        } => {
            println!("d released!");
            camera.ProcessKeyboardUp(CameraMovement::RIGHT);
        }
        _ => {}
    }
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

fn tick(fixed_delta_time: f32, game_state: &mut GameState) {
    let mut rng = rand::thread_rng();

    println!("ticking game state");

    //Shuffle all cube positions!
    for (i, cube_pos) in game_state.game_objects.iter_mut().enumerate() {
        let rng_x = rng.gen_range(-5.0, 5.0);
        let rng_y = rng.gen_range(-5.0, 5.0);
        let rng_z = rng.gen_range(-5.0, 5.0);
        //println!("{0} {1} {2}", rng_x, rng_y, rng_z);
        //*cube_pos = vec3(rng_x, rng_y, rng_z);
    }
}

fn main() {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    //Settings
    let game_name: String = "Fighting Game".to_string();
    let init_width: u32 = 720;
    let init_height: u32 = 480;

    // Game Window
    // -----------
    let mut game_window = game_window::create_game_window(&game_name, init_width, init_height);
    let mut viewport = renderer_gl::Viewport::for_window(init_width as i32, init_height as i32);
    let renderer = renderer::create_renderer(&game_window.gl, &res);

    // Game Objects
    // ------------
    let light_positions: [Vector3<f32>; 1] = [vec3(1.0, 1.0, 1.0)];
    // Cubes
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
        vec3(-1.3, 1.0, -1.5),
    ];

    // Game State
    // ----------
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };
    let mut state_current: GameState = GameState {
        game_objects: cube_positions,
        light_objects: light_positions,
    };
    println!("gamestate bytes: {0}", std::mem::size_of::<GameState>());
    // let mut state_previous: GameState = state_current.clone();

    let mut event_pump = game_window.sdl_context.event_pump().unwrap();
    let mut last_frame = Instant::now();
    let mut seconds_since_last_gametick: f32 = 0.0;
    'running: loop {

        // Events
        // ------
        for event in event_pump.poll_iter() {
            game_window
                .imgui_sdl2
                .handle_event(&mut game_window.imgui, &event);

            if game_window.imgui_sdl2.ignore_event(&event) {
                continue;
            }

            process_events(&mut game_window, &event, &mut viewport );
            let ok = process_input(&mut game_window, &event, &mut camera );
            if !ok {
                break 'running;
            }
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
        while seconds_since_last_gametick >= SECONDS_PER_GAMETICK {
            //Copying the entire gamestate every frame...
            //Could probably do this better eventually
            //state_previous = state_current.clone();

            tick(SECONDS_PER_GAMETICK, &mut state_current); //this update's state_current

            seconds_since_last_gametick -= SECONDS_PER_GAMETICK;
        }

        // Update Camera
        // ------
        // relative_mouse_state() twice and get a false position reading
        let mut mouse_state = event_pump.relative_mouse_state();

        let invert_mouse : bool = true;
        let mut y : i32 = mouse_state.y();
        if invert_mouse
        {
            y *= -1;
        }
        camera.ProcessMouseMovement(mouse_state.x() as f32, y as f32, true);
        camera.Update(delta_s);

        // Update Rendering
        // ---------
        let position = game_window.sdl_window.position();
        let size = game_window.sdl_window.size();
        //println!("size x: {0}, y: {1}", size.0, size.1);
        renderer.render(
            &game_window.gl,
            size.0 as i32,
            size.1 as i32,
            &camera,
            &state_current,
        );
        //lerp between game states
        //const float alpha = _timeSinceLastUpdate / timePerFrame;
        //game_state state_lerped = state_current * alpha + state_previous * ( 1.0 - alpha );

        ui(&mut game_window);
        game_window.sdl_window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
