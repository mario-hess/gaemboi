mod button;

use rfd::FileDialog;
use sdl2::image::LoadTexture;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::{keyboard::Keycode, EventPump};

use crate::{
    event_handler::EventHandler,
    menu::button::{Button, BTN_HEIGHT, BTN_WIDTH},
    ppu::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH, WHITE},
    window::Window,
    MachineState,
};

enum MenuState {
    Default,
    Keybindings,
}

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
        let keybindings_bytes = include_bytes!("../../images/keybindings.png");
        let keybindings_texture = viewport
            .texture_creator
            .load_texture_bytes(keybindings_bytes)
            .unwrap();

        let keybindings_width = keybindings_texture.query().width;
        let keybindings_height = keybindings_texture.query().height;
        let keybindings_position =
            Point::new(keybindings_width as i32 / 2, keybindings_height as i32 / 2);

        let button_bytes = include_bytes!("../../images/buttons.png");

        let button_texture = viewport
            .texture_creator
            .load_texture_bytes(button_bytes)
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
        let open_btn = Button::new(open_default, open_dest);

        let keys_default = Rect::new(0, BTN_HEIGHT, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let keys_dest = Rect::new(
            center_horizontal as i32,
            center_vertical as i32,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );
        let keys_btn = Button::new(keys_default, keys_dest);

        let exit_default = Rect::new(0, BTN_HEIGHT * 2, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let exit_dest = Rect::new(
            center_horizontal as i32,
            center_vertical as i32 + BTN_HEIGHT + BTN_HEIGHT / 2,
            BTN_WIDTH as u32,
            BTN_HEIGHT as u32,
        );
        let exit_btn = Button::new(exit_default, exit_dest);

        let back_default = Rect::new(0, BTN_HEIGHT * 3, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let back_dest = Rect::new(0, 0, BTN_WIDTH as u32, BTN_HEIGHT as u32);
        let back_btn = Button::new(back_default, back_dest);

        while event_handler.key_pressed != Some(Keycode::Escape) {
            event_handler.poll(event_pump);
            event_handler.check_resized(&mut viewport.canvas);

            if let Some(_file_path) = &event_handler.file_dropped {
                event_handler.machine_state = MachineState::Boot;
                break;
            }

            let (mouse_x, mouse_y) = (&event_handler.mouse_x, &event_handler.mouse_y);

            viewport.canvas.set_draw_color(WHITE);
            viewport.canvas.clear();

            match self.state {
                MenuState::Default => {
                    open_btn.draw(
                        &mut viewport.canvas,
                        &button_texture,
                        open_default,
                        open_dest,
                    );
                    keys_btn.draw(
                        &mut viewport.canvas,
                        &button_texture,
                        keys_default,
                        keys_dest,
                    );
                    exit_btn.draw(
                        &mut viewport.canvas,
                        &button_texture,
                        exit_default,
                        exit_dest,
                    );

                    if check_hovered(mouse_x, mouse_y, open_dest) {
                        if event_handler.mouse_btn_down {
                            open_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                open_btn.clicked,
                                open_dest,
                            );

                            if event_handler.mouse_btn_up {
                                println!("OPEN ACTION");

                                let file = FileDialog::new()
                                    .add_filter("gb", &["gb"])
                                    .set_directory("../")
                                    .pick_file();

                                event_handler.file_dropped =
                                    Some(file.unwrap().into_os_string().into_string().unwrap());
                                event_handler.reset_mouse_buttons();
                                event_handler.machine_state = MachineState::Boot;
                                break;
                            }
                        } else {
                            open_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                open_btn.hover,
                                open_dest,
                            );
                        }
                    } else if check_hovered(mouse_x, mouse_y, keys_dest) {
                        if event_handler.mouse_btn_down {
                            keys_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                keys_btn.clicked,
                                keys_dest,
                            );

                            if event_handler.mouse_btn_up {
                                self.state = MenuState::Keybindings;
                                event_handler.reset_mouse_buttons();
                                return;
                            }
                        } else {
                            keys_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                keys_btn.hover,
                                keys_dest,
                            );
                        }
                    } else if check_hovered(mouse_x, mouse_y, exit_dest) {
                        if event_handler.mouse_btn_down {
                            exit_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                exit_btn.clicked,
                                exit_dest,
                            );

                            if event_handler.mouse_btn_up {
                                event_handler.reset_mouse_buttons();
                                event_handler.key_pressed = Some(Keycode::Escape);
                                break;
                            }
                        } else {
                            exit_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                exit_btn.hover,
                                exit_dest,
                            );
                        }
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
                    open_btn.draw(
                        &mut viewport.canvas,
                        &button_texture,
                        back_default,
                        back_dest,
                    );

                    if check_hovered(mouse_x, mouse_y, back_dest) {
                        if event_handler.mouse_btn_down {
                            back_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                back_btn.clicked,
                                back_dest,
                            );

                            if event_handler.mouse_btn_up {
                                self.state = MenuState::Default;
                                event_handler.reset_mouse_buttons();
                                break;
                            }
                        } else {
                            back_btn.draw(
                                &mut viewport.canvas,
                                &button_texture,
                                back_btn.hover,
                                back_dest,
                            );
                        }
                    }
                }
            }

            if event_handler.mouse_btn_up {
                event_handler.reset_mouse_buttons();
            }

            viewport.canvas.present();
        }
    }
}

fn check_hovered(mouse_x: &i32, mouse_y: &i32, rect: Rect) -> bool {
    mouse_x >= &rect.left()
        && mouse_x < &rect.right()
        && mouse_y >= &rect.top()
        && mouse_y < &rect.bottom()
}
