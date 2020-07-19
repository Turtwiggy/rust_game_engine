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
pub mod util;
pub mod game;
//pub mod data;
//use threed::mesh::FGVertex;
use crate::game_window::GameWindow;
//use crate::renderer::Renderer;
use crate::renderer_gl::Viewport;
use crate::threed::camera::{Camera, CameraMovement};
use crate::threed::mesh::{FGMesh};
use crate::threed::model::{FGModel};
use crate::game::GameState;
use crate::util::profiling::ProfileInformation;
use crate::util::resources::Resources;
// use crate::threed::textures;
// use crate::data::materials;

use cgmath::{Vector2, vec3, Point3};
use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use std::borrow::Cow;
use std::path::{PathBuf, Path};

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
            use sdl2::video::FullscreenType;
            let is_fullscreen: bool = game_window.is_fullscreen();
            game_window.set_fullscreen(!is_fullscreen, FullscreenType::Desktop);
        }
        Event::KeyDown {
            keycode: Some(Keycode::M),
            ..
        } => {
            println!("trying to capture mouse");
            game_window.toggle_grabbed();
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

    //println!("ticking game state");

    let mut rng = rand::thread_rng();

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

use sdl2::audio::{AudioCallback, AudioSpecDesired,AudioSpecWAV,AudioCVT};
struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            // With channel type u8 the "silence" value is 128 (middle of the 0-2^8 range) so we need
            // to both fill in the silence and scale the wav data accordingly. Filling the silence
            // once the wav is finished is trivial, applying the volume is more tricky. We need to:
            // * Change the range of the values from [0, 255] to [-128, 127] so we can multiply
            // * Apply the volume by multiplying, this gives us range [-128*volume, 127*volume]
            // * Move the resulting range to a range centered around the value 128, the final range
            //   is [128 - 128*volume, 128 + 127*volume] – scaled and correctly positioned
            //
            // Using value 0 instead of 128 would result in clicking. Scaling by simply multiplying
            // would not give correct results.
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}

fn main() {

    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();

    // Default Settings
    // ----------------
    let name: String = "Fighting Game".to_string();
    let w: u32 = 720;
    let h: u32 = 480;
    let game_icon = res.load_bmp_image("icons/game_icon.bmp").unwrap();

    // Load Images & Textures
    // ----------------------
    // let faces = [
    //     "skybox/skybox-default/right.jpg",
    //     "skybox/skybox-default/left.jpg",
    //     "skybox/skybox-default/top.jpg",
    //     "skybox/skybox-default/bottom.jpg",
    //     "skybox/skybox-default/back.jpg",
    //     "skybox/skybox-default/front.jpg"
    // ];
    //loading this is super slow! disabled for now.
    //let cubemap_texture : u32 =  res.load_cubemap(&game_window.gl, &faces);
    // let cubeTexture = textures::loadTexture(&game_window.gl, "resources/textures/container.jpg");

    // Game Window
    // -----------
    let mut game_window = game_window::create_default(&name, w, h);
    game_window.set_window_icon(&game_icon);

    let mut renderer = renderer::create_default(&game_window.gl, &res, w as i32, h as i32);

    // Load models 
    // -----------
    let cube_model = FGModel::new(&game_window.gl, &res, "models/cube/cube.obj");
    println!("model size: {}", std::mem::size_of_val(&cube_model));

    let lizard_model = FGModel::new(&game_window.gl, &res, "models/lizard_wizard/lizard_wizard.obj");
    println!("lizard size: {}", std::mem::size_of_val(&lizard_model));
    println!("lizard has {} meshes", lizard_model.meshes.len());

    let (plane_vao, plane_vbo) = FGMesh::create_plane(&game_window.gl);
    //let (mesh_vao, mesh_vbo) = FGMesh::create_transparent_mesh(&game_window.gl);

    // Game State
    // ----------
    let mut state_current = game::game_state::create_default_gamestate();
    println!("gamestate bytes: {0}", std::mem::size_of_val(&state_current));

    // Skybox
    // ------
    let (cubemap_vao, cubemap_vbo) = FGMesh::create_skybox_vertices(&game_window.gl);

    // Camera
    // ------
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    // Audio
    // -----
    let wav_file : Cow<'static, Path> = match std::env::args().nth(1) {
        None => Cow::from(Path::new("./assets/audio/example_wav.wav")),
        Some(s) => Cow::from(PathBuf::from(s))
    };
    let mut audio_subsystem = game_window.sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None      // default
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav(wav_file)
            .expect("Could not load test WAV file");

        let cvt = AudioCVT::new(
                wav.format, wav.channels, wav.freq,
                spec.format, spec.channels, spec.freq)
            .expect("Could not convert WAV file");

        let data = cvt.convert(wav.buffer().to_vec());

        // initialize the audio callback
        Sound {
            data: data,
            volume: 0.25,
            pos: 0,
        }
    }).unwrap();

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
    'running: loop {

        device.resume();

        // Begin profiling
        // ---------------
        let profiler_time_continuous = Instant::now();
        let mut profiler_time : Instant;
        
        // Events
        // ------
        profiler_time = Instant::now();

        for event in event_pump.poll_iter() {
            game_window
                .imgui_sdl2
                .handle_event(&mut game_window.imgui, &event);

            if game_window.imgui_sdl2.ignore_event(&event) {
                continue;
            }

            process_events(&mut game_window, &event, &mut renderer.viewport);
            let ok = process_input(&mut game_window, &event, &mut camera );
            if !ok {
                break 'running;
            }
        }   

        // Mouse Events
        // ------------
        let mouse_state = &event_pump.mouse_state();
        let rel_mouse_state = event_pump.relative_mouse_state();
        // Invert mouse Y
        let invert_mouse_y : bool = true;
        let mut rel_mouse_y = rel_mouse_state.y();
        if invert_mouse_y
        {
            rel_mouse_y *= -1;
        }
        //Mouse Pos
        let rel_mouse_pos : Vector2<i32> = Vector2 {
            x: rel_mouse_state.x(),
            y: rel_mouse_y,
        };
        current_profile_information.events = profiler_time.elapsed().as_millis();

        // New frame
        // ---------
        profiler_time = Instant::now();

        let now = Instant::now();
        let delta_s = GameWindow::calculate_delta_time(now, last_frame);
        last_frame = now;
        timer_seconds += delta_s;

        game_window.new_frame(delta_s, &mouse_state);

        current_profile_information.frame_start = profiler_time.elapsed().as_millis();

        // Update Camera
        // -------------
        profiler_time = Instant::now();

        if game_window.get_mouse_grabbed()
        {
            camera.ProcessMouseMovement(rel_mouse_pos.x as f32, rel_mouse_pos.y as f32, true);
        };
        camera.Update(delta_s);
        current_profile_information.camera_update = profiler_time.elapsed().as_millis();

        // Update Game State
        // -----------------
        profiler_time = Instant::now();

        tick(delta_s, &mut state_current, &timer_seconds); //this update's state_current
        current_profile_information.gamestate_update = profiler_time.elapsed().as_millis();

        // Update Rendering
        // ---------
        profiler_time = Instant::now();

        renderer.render(
            &game_window.gl,
            game_window.sdl_window.size(),
            &camera,
            &state_current,
            //models and textures below... could be improved
            &cube_model,
            &lizard_model,
            &plane_vao,
            //&cubemap_texture,
            //&cubemap_vao
        );
        current_profile_information.renderer_update = profiler_time.elapsed().as_millis();

        // Calculate Average FPS
        // ---------------------
        fps_buffer[fps_buffer_idx] = 1.0 / delta_s as f32;
        fps_buffer_idx = (fps_buffer_idx + 1) % fps_buffer_length;
        let fps_buffer_avg = fps_buffer.iter().sum::<f32>() / fps_buffer.len() as f32;
        //println!("fps avg: {0}", fps_buffer_avg);

        // UI
        // --
        profiler_time = Instant::now();

        gui::ui(&mut game_window, timer_seconds, fps_buffer_avg, &previous_profile_information);
        current_profile_information.gui_update = profiler_time.elapsed().as_millis();

        // End Frame
        // ---------
        profiler_time = Instant::now();

        game_window.sdl_window.gl_swap_window();
        current_profile_information.frame_end = profiler_time.elapsed().as_millis();

        current_profile_information.full_loop = profiler_time_continuous.elapsed().as_millis();
        previous_profile_information = current_profile_information.clone();
    }
}

// ::std::thread::sleep(::std::time::Duration::new(0, (1000000000 as f64 / target_fps)));

//Code below is for a fixed timestep, as/when physics is implemented

//let mut seconds_since_last_fixed_tick: f64 = 0.0;
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
//     tick();
//     seconds_since_last_fixed_tick -= SECONDS_PER_GAMETICK;
// }

// // Fixed Gamestep Lerp
// // --------------
// //Produces a value [0, 1] based on how to  processing another gametick
// //This i s used to perform a linear interpolation between the two physics states to get the current state to render.
//let alpha : f64 = seconds_since_last_fixed_tick / SECONDS_PER_GAMETICK;
//println!("alpha: {0}", alpha);
//let lerped_gamestate = state_current * alpha + state_previous * (1.0 - alpha);