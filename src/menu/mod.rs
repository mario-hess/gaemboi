/**
 * @file    menu/mod.rs
 * @brief   Manages the menu.
 * @author  Mario Hess
 * @date    November 11, 2023
 */
mod button;

use rfd::FileDialog;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::EventPump;

use crate::{
    event_handler::EventHandler,
    menu::button::{Button, ButtonState, ButtonType, BTN_HEIGHT, BTN_WIDTH},
    ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH, WHITE},
    window::Window,
    MachineState,
};

pub fn run(event_handler: &mut EventHandler, event_pump: &mut EventPump, viewport: &mut Window) {
    let buttons_image = include_bytes!("../../images/buttons.png");
    let button_texture = viewport
        .texture_creator
        .load_texture_bytes(buttons_image)
        .unwrap();

    let center_horizontal = VIEWPORT_WIDTH / 2 - BTN_WIDTH as usize / 2;
    let center_vertical = VIEWPORT_HEIGHT / 2 - BTN_HEIGHT as usize / 2;

    let open_default = Rect::new(0, 0, BTN_WIDTH as u32, BTN_HEIGHT as u32);
    let open_dest = Rect::new(
        center_horizontal as i32,
        center_vertical as i32 - BTN_HEIGHT,
        BTN_WIDTH as u32,
        BTN_HEIGHT as u32,
    );
    let mut open_btn = Button::new(ButtonType::Open, open_default, open_dest);

    let exit_default = Rect::new(0, BTN_HEIGHT * 2, BTN_WIDTH as u32, BTN_HEIGHT as u32);
    let exit_dest = Rect::new(
        center_horizontal as i32,
        center_vertical as i32 + BTN_HEIGHT,
        BTN_WIDTH as u32,
        BTN_HEIGHT as u32,
    );
    let mut exit_btn = Button::new(ButtonType::Exit, exit_default, exit_dest);

    let mut menu_buttons = [&mut open_btn, &mut exit_btn];

    while !event_handler.pressed_escape {
        event_handler.poll(event_pump);
        event_handler.check_resized(&mut viewport.canvas);

        if event_handler.file_path.is_some() {
            event_handler.machine_state = MachineState::Boot;
            break;
        }

        let (mouse_x, mouse_y) = (event_handler.mouse_x, event_handler.mouse_y);

        viewport.canvas.set_draw_color(WHITE);
        viewport.canvas.clear();

        for button in &mut menu_buttons {
            match (
                button.check_hovered(&mouse_x, &mouse_y),
                event_handler.mouse_btn_down,
                event_handler.mouse_btn_up,
            ) {
                (true, true, true) => handle_clicked(event_handler, &button.btn_type),
                (true, true, _) => button.btn_state = ButtonState::Clicked,
                (true, _, _) => button.btn_state = ButtonState::Hovered,
                _ => button.btn_state = ButtonState::Default,
            };

            button.draw(&mut viewport.canvas, &button_texture, button.dest_rect);
        }

        if event_handler.mouse_btn_up {
            event_handler.reset_mouse_buttons();
        }

        viewport.canvas.present();
    }
}

fn handle_clicked(event_handler: &mut EventHandler, button_type: &ButtonType) {
    event_handler.reset_mouse_buttons();

    match button_type {
        ButtonType::Open => {
            let file = FileDialog::new()
                .add_filter("gb", &["gb"])
                .set_directory("../")
                .pick_file();

            if let Some(file) = file {
                event_handler.file_path = Some(file.into_os_string().into_string().unwrap());
            }
        }
        ButtonType::Exit => {
            event_handler.quit = true;
            event_handler.pressed_escape = true;
        }
    }
}
