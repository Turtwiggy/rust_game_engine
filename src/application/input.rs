pub fn create_input() -> Input {
  return Input {};
}

// fn process_events(game_window: &mut GameWindow, event: &sdl2::event::Event) {
//   match event {
//     sdl2::event::Event::Window {
//       win_event: sdl2::event::WindowEvent::Resized(w, h),
//       ..
//     } => {
//       println!("updating viewport");
//       // viewport.update_size(*w, *h);
//       // viewport.set_used(&game_window.gl);
//     }
//     _ => {}
//   }
// }

// fn process_input(game_window: &mut GameWindow, event: &sdl2::event::Event) -> bool {
//   match event {
//     Event::KeyDown {
//       keycode: Some(Keycode::F),
//       ..
//     } => {
//       use sdl2::video::FullscreenType;
//       let is_fullscreen: bool = game_window.is_fullscreen();
//       game_window.set_fullscreen(!is_fullscreen, FullscreenType::Desktop);
//     }
//     Event::KeyDown {
//       keycode: Some(Keycode::M),
//       ..
//     } => {
//       println!("trying to capture mouse");
//       game_window.toggle_grabbed();
//     }
//     Event::KeyDown {
//       keycode: Some(Keycode::O),
//       ..
//     } => {
//       println!("o pressed!");
//     }
//     _ => {}
//   }

//   return true;
// }

pub struct Input {}

impl Input {
  pub fn new_frame(&mut self) -> () {}
  pub fn process_key_down(&mut self) -> () {
    unimplemented!();
  }
  pub fn process_key_up(&mut self) -> () {
    unimplemented!();
  }
  pub fn process_mouse_event(&mut self) -> () {
    unimplemented!();
  }
  pub fn process_controller_added(&mut self) -> () {
    unimplemented!();
  }
  pub fn process_controller_removed(&mut self) -> () {
    unimplemented!();
  }

  // keyboard
  pub fn get_key_down(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_key_up(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_key_held(&mut self) -> bool {
    unimplemented!();
  }

  // mouse
  pub fn get_mouse_lmb_held(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_mouse_rmb_held(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_mouse_mmb_held(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_mouse_lmb_down(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_mouse_rmb_down(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_mouse_mmb_down(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_mouse_pos(&mut self) -> bool {
    unimplemented!();
  }
  pub fn set_mousewheel_y(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_mousewheel_y(&mut self) -> bool {
    unimplemented!();
  }

  //controller
  pub fn get_button_down(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_button_up(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_button_held(&mut self) -> bool {
    unimplemented!();
  }
  pub fn get_axis_dir(&mut self) -> bool {
    unimplemented!();
  }
}
