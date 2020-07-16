use sdl2::video::FullscreenType;
use sdl2::surface::Surface;

use crate::util::resources::{Resources};

pub fn create_game_window(n: &str, w: u32, h: u32) -> GameWindow {
    println!("creating game window: {0}", n);
    let _sdl_context = sdl2::init().unwrap();

    let _video = _sdl_context.video().unwrap();

    {
        let gl_attr = _video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        gl_attr.set_stencil_size(1);
    }

    let _window = _video
        .window(n, w, h)
        .position_centered()
        .resizable()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    let _gl_context = _window
        .gl_create_context()
        .expect("Couldn't create GL context");
        
    let _gl = gl::Gl::load_with(|s| {
        _video.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut _imgui = imgui::Context::create();
    _imgui.set_ini_filename(None);
    let mut _imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut _imgui, &_window);

    let _renderer =
        imgui_opengl_renderer::Renderer::new(&mut _imgui, |s| _video.gl_get_proc_address(s) as _);

    return GameWindow {
        sdl_context: _sdl_context,
        gl_context: _gl_context,
        gl : _gl,
        sdl_window: _window,
        imgui: _imgui,
        imgui_sdl2: _imgui_sdl2,
        imgui_renderer: _renderer,
    };
}

pub struct GameWindow {
    pub sdl_context: sdl2::Sdl,
    pub sdl_window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,
    pub gl : gl::Gl,
    pub imgui: imgui::Context,
    pub imgui_sdl2: imgui_sdl2::ImguiSdl2,
    pub imgui_renderer: imgui_opengl_renderer::Renderer,
}

impl GameWindow {
    pub fn get_width_and_height(&self) -> (u32, u32) {
        return self.sdl_window.size();
    }

    pub fn get_position(&self) -> (i32, i32) {
        return self.sdl_window.position();
    }


    /*  pub enum FullscreenType {
        Off = 0,
        True = 0x00_00_00_01,
        Desktop = 0x00_00_10_01 (borderless) } */
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        if fullscreen {
            let result = self.sdl_window.set_fullscreen(FullscreenType::Desktop);
        } else {
            let result = self.sdl_window.set_fullscreen(FullscreenType::Off);
        }
    }

    pub fn is_fullscreen(&mut self) -> bool {
        let state = self.sdl_window.fullscreen_state();

        if state == FullscreenType::True 
        || state == FullscreenType::Desktop {
            return true;
        }
        return false;
    }
    
    // pub fn get_monitor_refresh_rate(&self, display: i32) -> f32 {
    //     let display_mode = self.sdl_window.display_mode().unwrap();

    //     display_mode.refresh_rate();
    // }

    pub fn get_current_display_mode(&self) -> sdl2::video::DisplayMode {
        return self.sdl_window.display_mode().unwrap();
    }

    pub fn set_window_icon(&mut self, res : &Resources, path: &str) {

        //todo better error chacking
        let image = res.load_image(path).unwrap();

        self.sdl_window.set_icon(&image);
    }

    /* MOUSE */

    pub fn set_relative_mouse_mode(&self, on: bool) {
        self.sdl_context.mouse().set_relative_mouse_mode(on);
    }

    pub fn relative_mouse_mode(&self) -> bool {
        return self.sdl_context.mouse().relative_mouse_mode();
    }

    pub fn get_mouse_grabbed(&self) -> bool{
        return self.sdl_window.grab();
    }

    pub fn set_mouse_grabbed(&mut self, grab : bool) {
        self.sdl_window.set_grab(grab);
    }

    pub fn capture_mouse(&mut self, capture: bool) {
        return self.sdl_context.mouse().capture(capture);
    }

    pub fn toggle_grabbed(&mut self)
    {
        let grabbed : bool = self.get_mouse_grabbed();
        self.set_relative_mouse_mode(!grabbed);
        self.set_mouse_grabbed(!grabbed);
    }
}
