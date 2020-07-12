use game_window::GameWindow;
use imgui::*;

pub fn ui(game_window: &mut GameWindow, timer : f64, fps : f64) {
    let ui = game_window.imgui.frame();
    //ui.show_demo_window(&mut true);

    //Seperate Window
    Window::new(im_str!("Hello world"))
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(&ui, || {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.text(format!(
                "Timer: {:.1}", timer
            ));
            ui.text(format!(
                "FPS: {:.1}", fps
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