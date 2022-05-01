#[derive(Default, Copy, Clone)]
pub struct ProfileInformation {
  pub frame_start: u128,
  pub events: u128,
  pub camera_update: u128,
  pub gamestate_update: u128,
  pub renderer_update: u128,
  pub gui_update: u128,
  pub frame_end: u128,
  pub full_loop: u128,
}
