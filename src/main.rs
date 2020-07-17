extern crate gl;
extern crate half;
extern crate imgui;
// extern crate imgui_opengl_renderer;
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
extern crate tobj;
extern crate image;

pub mod gui;
pub mod game_window;
pub mod renderer;
pub mod renderer_gl;
pub mod threed;
pub mod game;
pub mod util;
pub mod data;
use threed::mesh::FGVertex;
use crate::game_window::GameWindow;
use crate::renderer::Renderer;
use crate::renderer_gl::Viewport;
use crate::threed::camera::{Camera, CameraMovement};
use crate::threed::mesh::{FGMesh};
use crate::threed::model::{FGModel};
use crate::game::GameState;
use crate::util::profiling::ProfileInformation;
use crate::util::resources::Resources;
use crate::data::materials;

use cgmath::{vec3, Point3, Vector3};
use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::path::Path;
use std::time::Instant;

//Game Settings
// const GAME_TICKS_PER_SECOND: u32 = 2;
// const SECONDS_PER_GAMETICK: f64 = 1.0 / GAME_TICKS_PER_SECOND as f64;

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

fn tick(delta_time: f64, game_state: &mut GameState, timer_seconds: &f64) {
    let mut rng = rand::thread_rng();

    //println!("ticking game state");

    //Shuffle all cube positions!
    for (i, cube_pos) in game_state.game_objects.iter_mut().enumerate() {
        let rng_x = rng.gen_range(-5.0, 5.0);
        let rng_y = rng.gen_range(-5.0, 5.0);
        let rng_z = rng.gen_range(-5.0, 5.0);
        //println!("{0} {1} {2}", rng_x, rng_y, rng_z);
        //*cube_pos = vec3(rng_x, rng_y, rng_z);
    }

    //Make light source move around
    //println!("timer_seconds value: {0}", timer_seconds);
    let mut light_pos = game_state.light_objects[0];
    let new_x = timer_seconds.sin();
    let new_y = light_pos.y;
    let new_z = timer_seconds.cos();
    light_pos = vec3(new_x as f32, new_y as f32, new_z as f32);
    game_state.light_objects[0] = light_pos;
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
    game_window.set_window_icon(&res, "icons/game_icon.bmp");

    let mut viewport = renderer_gl::Viewport::for_window(init_width as i32, init_height as i32);
    let renderer = renderer::create_renderer(&game_window.gl, &res);

    // Initial Settings
    // ----------------
    //let current_display = game_window.get_current_display_mode();
    //let target_fps : f64 = current_display.refresh_rate as f64;
    //println!("Target FPS set to: {0}", target_fps);

    // Load models 
    // -----------
    let cube_model = FGModel::new(&game_window.gl, &res, "models/lizard_wizard/lizard_wizard.obj");
    println!("model size: {}", std::mem::size_of_val(&cube_model));

    let sponza_model = FGModel::new(&game_window.gl, &res, "models/cube/cube.obj");
    println!("sponza size: {}", std::mem::size_of_val(&sponza_model));
    println!("sponza has {} meshes", sponza_model.meshes.len());

    let (plane_vao, plane_vbo) = FGMesh::create_plane(&game_window.gl);
    //let (mesh_vao, mesh_vbo) = FGMesh::create_transparent_mesh(&game_window.gl);

    //let plane_mesh = FGMesh::new(gl, vertices: Vec<FGVertex>, indices: Vec<u32>)
    //let transparent_mesh = FGMesh::new(gl, vertices: Vec<FGVertex>, indices: Vec<u32>)

    // Game State
    // ----------
    let mut light_positions = Vec::new();
    light_positions.push(vec3( 0.0,  0.0,  0.0));
    light_positions.push(vec3( 1.0,  1.0,  1.0));
    light_positions.push(vec3( 5.0,  5.0,  5.0));
    light_positions.push(vec3(-2.0, -2.0, -2.0));
    light_positions.push(vec3(-6.0, -6.0, -6.0));
    let mut light_colours = Vec::new();
    light_colours.push(vec3( 1.0,  1.0,  1.0));
    light_colours.push(vec3( 1.0,  0.0,  0.0));
    light_colours.push(vec3( 0.0,  1.0,  0.0));
    light_colours.push(vec3( 0.0,  0.0,  1.0));
    light_colours.push(vec3( 0.5,  0.5,  0.5));

    let mut cube_positions = Vec::new();
    cube_positions.push(vec3(0.0, 0.0, 0.0));
    cube_positions.push(vec3(2.0, 5.0, -15.0));
    cube_positions.push(vec3(-1.5, -2.2, -2.5));
    cube_positions.push(vec3(-3.8, -2.0, -12.3));
    cube_positions.push(vec3(2.4, -0.4, -3.5));
    cube_positions.push(vec3(-1.7, 3.0, -7.5));
    cube_positions.push(vec3(1.3, -2.0, -2.5));
    cube_positions.push(vec3(1.5, 2.0, -2.5));
    cube_positions.push(vec3(1.5, 0.2, -1.5));
    cube_positions.push(vec3(-1.3, 1.0, -1.5));

    let mut sponza_position = Vec::new();
    sponza_position.push(vec3(0.0, 0.0, 0.0));

    let mut plane_position = Vec::new();
    plane_position.push(vec3(0.0, 0.0, 0.0));

    let mut grass_position = Vec::new();
    grass_position.push(vec3(0.0, 0.0, 0.0));
    grass_position.push(vec3(1.0, 0.0, 1.0));
    grass_position.push(vec3(2.0, 0.0, 2.0));
    grass_position.push(vec3(3.0, 0.0, 3.0));
    grass_position.push(vec3(4.0, 0.0, 4.0));

    let mut state_current: GameState = GameState {
        game_objects: cube_positions,
        light_objects: light_positions,
        light_colours: light_colours,
        sponza_position: sponza_position,
        plane_position: plane_position,
        grass_position: grass_position
    };
    println!("gamestate bytes: {0}", std::mem::size_of::<GameState>());

    // Camera
    // ------
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    // FPS Info & Profiling
    // --------
    const fps_buffer_length : usize = 50;
    let mut fps_buffer : [f32; fps_buffer_length] = [0.0;fps_buffer_length];
    let mut fps_buffer_idx = 0;
    let mut timer_seconds : f64 = 0.0;
    let mut current_profile_information : ProfileInformation = Default::default();
    let mut previous_profile_information : ProfileInformation = Default::default();

    // Game Loop
    // ---------
    let mut event_pump = game_window.sdl_context.event_pump().unwrap();
    let mut last_frame = Instant::now();
    //let mut seconds_since_last_fixed_tick: f64 = 0.0;
    'running: loop {

        // Events
        // ------
        let mut time_now = Instant::now();
        let mut time_continuous = Instant::now();
        
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
        // relative_mouse_state() twice and get a false position reading
        let mouse_state = event_pump.relative_mouse_state();
        current_profile_information.events = time_now.elapsed().as_millis();

        //Prepare frame
        time_now = Instant::now();
        game_window.imgui_sdl2.prepare_frame(
            game_window.imgui.io_mut(),
            &game_window.sdl_window,
            &event_pump.mouse_state(),
        );
        let now = Instant::now();
        let delta = now - last_frame;
        let mut delta_s = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / 1_000_000_000.0;
        timer_seconds += delta_s;
        if delta_s > 0.25 //Clamp delta_s to avoid spiral of death
        {
            delta_s = 0.25;
        }
        last_frame = now;
        game_window.imgui.io_mut().delta_time = delta_s as f32;
        current_profile_information.frame_start = time_now.elapsed().as_millis();

        // Update Camera
        // -------------
        time_now = Instant::now();
        let invert_mouse : bool = true;
        let mut y : i32 = mouse_state.y();
        if game_window.get_mouse_grabbed()
        {
            if invert_mouse
            {
                y *= -1;
            }
            camera.ProcessMouseMovement(mouse_state.x() as f32, y as f32, true);
        };
        camera.Update(delta_s);
        current_profile_information.camera_update = time_now.elapsed().as_millis();

        // Update Game State
        // -----------------
        time_now = Instant::now();
        tick(delta_s, &mut state_current, &timer_seconds); //this update's state_current
        current_profile_information.gamestate_update = time_now.elapsed().as_millis();

        // Update Rendering
        // ---------
        time_now = Instant::now();
        renderer.render(
            &game_window.gl,
            game_window.sdl_window.size(),
            &camera,
            &state_current,
            &cube_model,
            &sponza_model,
            &plane_vao,
            &plane_vao
        );
        current_profile_information.renderer_update = time_now.elapsed().as_millis();

        // Show Profiling 
        // ---------
        fps_buffer[fps_buffer_idx] = 1.0 / delta_s as f32;
        fps_buffer_idx = (fps_buffer_idx + 1) % fps_buffer_length;
        let fps_buffer_avg = fps_buffer.iter().sum::<f32>() / fps_buffer.len() as f32;
        //println!("fps avg: {0}", fps_buffer_avg);

        // UI
        // --
        time_now = Instant::now();
        gui::ui(&mut game_window, timer_seconds, fps_buffer_avg, &previous_profile_information);
        current_profile_information.gui_update = time_now.elapsed().as_millis();

        // End Frame
        // ---------
        time_now = Instant::now();
        game_window.sdl_window.gl_swap_window();
        current_profile_information.frame_end = time_now.elapsed().as_millis();

        // ::std::thread::sleep(::std::time::Duration::new(0, (1000000000 as f64 / target_fps)));

        current_profile_information.full_loop = time_continuous.elapsed().as_millis();
        previous_profile_information = current_profile_information.clone();
    }
}


// //let mut state_previous: GameState = state_current.clone();
// // Game Logic Tick - X ticks per second
// // ------------------------------------
// // If you want to do a replay, take snapshots of input and delta times
// seconds_since_last_fixed_tick += delta_s;
// while seconds_since_last_fixed_tick >= SECONDS_PER_GAMETICK {
//     //Copying the entire gamestate every frame...
//     //Could probably do this better eventually
//     state_previous = state_current.clone();
// 
//     TICK HERE
//     seconds_since_last_fixed_tick -= SECONDS_PER_GAMETICK;
// }

// // Fixed Gamestep Lerp
// // --------------
// //Produces a value [0, 1] based on how to  processing another gametick
// //This i s used to perform a linear interpolation between the two physics states to get the current state to render.
//let alpha : f64 = seconds_since_last_fixed_tick / SECONDS_PER_GAMETICK;
//println!("alpha: {0}", alpha);
//let lerped_gamestate = state_current * alpha + state_previous * (1.0 - alpha);