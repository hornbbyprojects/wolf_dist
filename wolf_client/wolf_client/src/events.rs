use crate::network;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use coords::PixelCoords;
use sdl2::event::*;
use sdl2::keyboard::*;
use sdl2::mouse::*;
use wolf_interface::*;

pub struct EventState {
    pub quitting: bool,
    current_velocity: Option<(f64, f64)>,
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
    mouse_x: i32,
    mouse_y: i32,
}
impl EventState {
    pub fn new() -> Self {
        EventState {
            quitting: false,
            current_velocity: None,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            mouse_x: 0,
            mouse_y: 0,
        }
    }
    pub fn poll(
        &mut self,
        event_pump: &mut sdl2::EventPump,
        server_connection: &mut network::ServerConnection,
        view_coords: PixelCoords,
    ) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {
                    timestamp: _timestamp,
                } => {
                    self.quitting = true;
                }
                Event::KeyDown { keycode, .. } => {
                    self.parse_key_event(view_coords, keycode, server_connection, true);
                }
                Event::KeyUp { keycode, .. } => {
                    self.parse_key_event(view_coords, keycode, server_connection, false);
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.parse_mouse_button_event(view_coords, server_connection, mouse_btn);
                }
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_x = x;
                    self.mouse_y = y;
                }
                _ => {}
            }
        }
    }
    fn parse_mouse_button_event(
        &self,
        view_coords: PixelCoords,
        server_connection: &mut network::ServerConnection,
        mouse_button: MouseButton,
    ) {
        let slot = match mouse_button {
            MouseButton::Left => 0,
            _ => 1,
        };
        self.cast_ability(view_coords, server_connection, slot);
    }
    fn cast_ability(
        &self,
        view_coords: PixelCoords,
        server_connection: &mut network::ServerConnection,
        slot: u8,
    ) {
        let target_coords = view_coords
            .translate(self.mouse_x, -self.mouse_y)
            .translate(-((SCREEN_WIDTH / 2) as i32), (SCREEN_HEIGHT / 2) as i32); //translate to server-side coords
        let ability_command = AbilityCommand {
            target_coords,
            slot,
        };
        server_connection
            .commands
            .send(Command::Ability(ability_command))
            .unwrap();
    }
    fn parse_key_event(
        &mut self,
        view_coords: PixelCoords,
        keycode: Option<Keycode>,
        server_connection: &mut network::ServerConnection,
        key_down: bool,
    ) {
        if let Some(keycode) = keycode {
            match keycode {
                Keycode::W => {
                    self.moving_up = key_down;
                    self.update_current_velocity(server_connection);
                }
                Keycode::S => {
                    self.moving_down = key_down;
                    self.update_current_velocity(server_connection);
                }
                Keycode::A => {
                    self.moving_left = key_down;
                    self.update_current_velocity(server_connection);
                }
                Keycode::D => {
                    self.moving_right = key_down;
                    self.update_current_velocity(server_connection);
                }
                Keycode::Q => {
                    if key_down {
                        self.cast_ability(view_coords, server_connection, 2);
                    }
                }
                Keycode::E => {
                    if key_down {
                        self.cast_ability(view_coords, server_connection, 3);
                    }
                }
                Keycode::LCtrl => {
                    if key_down {
                        server_connection
                            .commands
                            .send(Command::TraverseDoorsCommand)
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
    }
    fn update_current_velocity(&mut self, server_connection: &mut network::ServerConnection) {
        let dx = if self.moving_left {
            -1.0
        } else if self.moving_right {
            1.0
        } else {
            0.0
        };
        let dy = if self.moving_up {
            -1.0
        } else if self.moving_down {
            1.0
        } else {
            0.0
        };
        if Some((dx, dy)) == self.current_velocity {
            return;
        }
        server_connection
            .commands
            .send(Command::Move(MoveCommand {
                dx,
                dy: -dy, //server y goes up, SDL y goes down
            }))
            .expect("unable to send move command");
        self.current_velocity = Some((dx, dy));
    }
}
