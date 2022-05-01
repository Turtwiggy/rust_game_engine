extern crate imgui;
extern crate sdl2;
pub mod application;
use crate::application::game_window::*;
use crate::application::input::*;
use imgui::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::timer;
use std::time::Instant;

fn tick(delta_time: f64) {}

fn gui(gw: &mut GameWindow) -> () {
  let mut framerate: f32 = 0.0;
  {
    framerate = gw.imgui.io_mut().framerate;
  }
  let ui = gw.imgui.frame();

  // ui.show_demo_window(&mut true);

  Window::new("Profiling")
    .size([300.0, 100.0], Condition::FirstUseEver)
    .build(&ui, || {
      ui.text(format!("FPS: {}", framerate));
      ui.separator();
      let mouse_pos = ui.io().mouse_pos;
      ui.text(format!(
        "Mouse Position: ({:.1},{:.1})",
        mouse_pos[0], mouse_pos[1]
      ));
    });

  gw.imgui_sdl2.prepare_render(&ui, &gw.sdl_window);
  gw.imgui_renderer.render(ui);
}

fn main() {
  // let game_icon = res.load_bmp_image("icons/game_icon.bmp").unwrap();
  // gw.set_window_icon(&game_icon);
  const VERSION: &str = env!("CARGO_PKG_VERSION");
  let app_name: String = format!("Fighting Game [{}]", VERSION);
  let mut gw = create_window(&app_name, 720, 480);
  let mut input = create_input();
  let timer = gw.sdl_context.timer().unwrap();
  let mut event_pump = gw.sdl_context.event_pump().unwrap();

  let mut last_frame = Instant::now();
  'running: loop {
    // -----------
    // Begin Frame
    // -----------
    // let profiler_time_continuous = Instant::now();
    // let mut profiler_time: Instant;
    // profiler_time = Instant::now();
    {
      gw.imgui_sdl2
        .prepare_frame(gw.imgui.io_mut(), &gw.sdl_window, &event_pump.mouse_state());
    }
    // profiler_time.elapsed().as_millis();

    // ------
    // Inputs
    // -------
    input.new_frame();
    {
      for event in event_pump.poll_iter() {
        gw.imgui_sdl2.handle_event(&mut gw.imgui, &event);
        if gw.imgui_sdl2.ignore_event(&event) {
          continue;
        }

        // temporary
        match event {
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => {
            break 'running;
          }
          _ => {}
        }
      }
    }

    // ----------
    // Game State
    // ----------
    {
      let now = Instant::now();
      let delta = now - last_frame;
      let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
      last_frame = now;
      gw.imgui.io_mut().delta_time = delta_s;
    }

    // --- fun things

    // ---------
    // Rendering
    // ---------
    unsafe {
      gw.gl.ClearColor(0.6, 0.0, 0.8, 1.0);
      gw.gl.Clear(gl::COLOR_BUFFER_BIT);
    }

    // ---------
    // GUI
    // ---------
    gui(&mut gw);

    // ---------
    // End Frame
    // ---------
    gw.sdl_window.gl_swap_window();
    let frame_end_time = timer.performance_counter();
    // std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
  }
}
