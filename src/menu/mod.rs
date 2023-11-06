/**
 * @file    menu/mod.rs
 * @brief   Manages the menu.
 * @author  Mario Hess
 * @date    November 06, 2023
 */
mod button;

use rfd::FileDialog;
use sdl2::image::LoadTexture;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::EventPump;

use crate::{
    event_handler::EventHandler,
    menu::button::{Button, ButtonState, ButtonType, BTN_HEIGHT, BTN_WIDTH},
    ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH, WHITE},
    window::Window,
    MachineState,
};

#[derive(Copy, Clone)]
enum MenuState {
    Default,
    Keybindings,
}

#[derive(Copy, Clone)]
pub struct Menu {
    state: MenuState,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            state: MenuState::Default,
        }
    }

    pub fn run(
        &mut self,
        event_handler: &mut EventHandler,
        event_pump: &mut EventPump,
        viewport: &mut Window,
    ) {
        let keybindings_image = include_bytes!("../../images/keybindings.png");
        let keybindings_texture = viewport
            .texture_creator
            .load_texture_bytes(keybindings_image)
            .unwrap();

        let keybindings_width = keybindings_texture.query().width;
        let keybindings_height = keybindings_texture.query().height;
        let keybindings_position =
            Point::new(keybindings_width as i32 / 2, keybindings_height as i32 / 2);

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
            center_vertical as i32 - BTN_HEIGHT - BTN_HEIGHT / 2,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );
        let mut open_btn = Button::new(ButtonType::Open, open_default, open_dest);

        let keys_default = Rect::new(0, BTN_HEIGHT, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let keys_dest = Rect::new(
            center_horizontal as i32,
            center_vertical as i32,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );
        let mut keys_btn = Button::new(ButtonType::Keys, keys_default, keys_dest);

        let exit_default = Rect::new(0, BTN_HEIGHT * 2, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let exit_dest = Rect::new(
            center_horizontal as i32,
            center_vertical as i32 + BTN_HEIGHT + BTN_HEIGHT / 2,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );
        let mut exit_btn = Button::new(ButtonType::Exit, exit_default, exit_dest);

        let back_default = Rect::new(0, BTN_HEIGHT * 3, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let back_dest = Rect::new(0, 0, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let mut back_btn = Button::new(ButtonType::Back, back_default, back_dest);

        let mut menu_buttons = [&mut open_btn, &mut keys_btn, &mut exit_btn];

        while !event_handler.escape_pressed {
            event_handler.poll(event_pump);
            event_handler.check_resized(&mut viewport.canvas);

            if event_handler.file_path.is_some() {
                event_handler.machine_state = MachineState::Boot;
                break;
            }

            let (mouse_x, mouse_y) = (event_handler.mouse_x, event_handler.mouse_y);

            viewport.canvas.set_draw_color(WHITE);
            viewport.canvas.clear();

            match self.state {
                MenuState::Default => {
                    for button in &mut menu_buttons {
                        match (
                            button.check_hovered(&mouse_x, &mouse_y),
                            event_handler.mouse_btn_down,
                            event_handler.mouse_btn_up,
                        ) {
                            (true, true, true) => {
                                self.handle_clicked(event_handler, &button.btn_type)
                            }
                            (true, true, _) => button.btn_state = ButtonState::Clicked,
                            (true, _, _) => button.btn_state = ButtonState::Hovered,
                            _ => button.btn_state = ButtonState::Default,
                        };

                        button.draw(&mut viewport.canvas, &button_texture, button.dest_rect);
                    }
                }
                MenuState::Keybindings => {
                    viewport
                        .canvas
                        .copy(
                            &keybindings_texture,
                            None,
                            Rect::from_center(
                                keybindings_position,
                                keybindings_width,
                                keybindings_height,
                            ),
                        )
                        .unwrap();

                    match (
                        back_btn.check_hovered(&mouse_x, &mouse_y),
                        event_handler.mouse_btn_down,
                        event_handler.mouse_btn_up,
                    ) {
                        (true, true, true) => {
                            self.handle_clicked(event_handler, &back_btn.btn_type);
                        }
                        (true, true, _) => back_btn.btn_state = ButtonState::Clicked,
                        (true, _, _) => back_btn.btn_state = ButtonState::Hovered,
                        _ => back_btn.btn_state = ButtonState::Default,
                    };

                    back_btn.draw(&mut viewport.canvas, &button_texture, back_dest);
                }
            }

            if event_handler.mouse_btn_up {
                event_handler.reset_mouse_buttons();
            }

            viewport.canvas.present();
        }
    }

    fn handle_clicked(&mut self, event_handler: &mut EventHandler, button_type: &ButtonType) {
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
            ButtonType::Keys => {
                self.state = MenuState::Keybindings;
            }
            ButtonType::Exit => {
                event_handler.quit = true;
                event_handler.escape_pressed = true;
            }
            ButtonType::Back => {
                self.state = MenuState::Default;
            }
        }
    }
}
