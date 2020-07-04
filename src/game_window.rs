extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

use sdl2::video::FullscreenType;

pub fn create_game_window(n: &str, w: u32, h: u32) -> GameWindow {
    println!("creating game window: {0}", n);
    let _sdl_context = sdl2::init().unwrap();

    let _video = _sdl_context.video().unwrap();

    {
        let gl_attr = _video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
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
    gl::load_with(|s| _video.gl_get_proc_address(s) as _);

    let mut _imgui = imgui::Context::create();
    _imgui.set_ini_filename(None);
    let mut _imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut _imgui, &_window);

    let _renderer =
        imgui_opengl_renderer::Renderer::new(&mut _imgui, |s| _video.gl_get_proc_address(s) as _);

    return GameWindow {
        name: n.to_string(),
        width: w,
        height: h,
        is_fullscreen: false,

        sdl_context: _sdl_context,
        gl_context: _gl_context,
        sdl_window: _window,
        imgui: _imgui,
        imgui_sdl2: _imgui_sdl2,
        renderer: _renderer,
    };
}

pub struct GameWindow {
    name: String,
    width: u32,
    height: u32,
    is_fullscreen: bool,

    pub sdl_context: sdl2::Sdl,
    pub sdl_window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,
    pub imgui: imgui::Context,
    pub imgui_sdl2: imgui_sdl2::ImguiSdl2,
    pub renderer: imgui_opengl_renderer::Renderer,
}

impl GameWindow {
    pub fn get_width_and_height(&self) -> (u32, u32) {
        return self.sdl_window.size();
    }

    pub fn get_position(&self) -> (i32, i32) {
        return self.sdl_window.position();
    }

    pub fn is_grabbed(&self) -> bool{
        return self.sdl_window.grab();
    }

    pub fn set_grabbed(&mut self, grab : bool) {
        self.sdl_window.set_grab(grab);
    }

    //pub enum FullscreenType {
    //     Off = 0,
    //     True = 0x00_00_00_01,
    //     Desktop = 0x00_00_10_01,
    // }
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
}
