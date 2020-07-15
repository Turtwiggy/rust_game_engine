use game_window::GameWindow;
use imgui::*;
use util::profiling::ProfileInformation;

pub fn ui(
    game_window: &mut GameWindow, 
    timer : f64, 
    fps : f32, 
    profie_info : &ProfileInformation ) {
    let ui = game_window.imgui.frame();
    //ui.show_demo_window(&mut true);

    //Seperate Window
    Window::new(im_str!("Hello world"))
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(&ui, || {
            ui.text(format!(
                "Timer: {:.1}", timer
            ));
            ui.text(format!(
                "FPS: {}", fps
            ));
            ui.separator();

            ui.text(im_str!("Previous Frame Information"));
            ui.text(format!(
                "Frame Start: {}", profie_info.frame_start
            ));
            ui.text(format!(
                "Events: {}", profie_info.events
            ));
            ui.text(format!(
                "Camera Update: {}", profie_info.camera_update
            ));
            ui.text(format!(
                "Gamestate Update: {}", profie_info.gamestate_update
            ));
            ui.text(format!(
                "Renderer Update: {}", profie_info.renderer_update
            ));
            ui.text(format!(
                "Gui Update: {}", profie_info.gui_update
            ));
            ui.text(format!(
                "Frame End: {}", profie_info.frame_end
            ));
            ui.text(format!(
                "Total Loop: {}", profie_info.full_loop
            ));
        
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