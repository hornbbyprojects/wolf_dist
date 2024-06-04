use overlays::AbilitiesOverlay;
use sdl2;
use std::time;

mod client_side_component;
mod drawing;
mod events;
mod id_types;
mod network;
mod overlays;
mod sprite_info;
mod sprites;

use crate::client_side_component::Game;
use crate::drawing::*;
use crate::sprites::*;
use wolf_interface::*;

pub const SCALING: u32 = 1;
pub const SCREEN_WIDTH: u32 = 1000;
pub const SCREEN_HEIGHT: u32 = 800;

fn main() {
    draw_thread();
}

fn draw_thread() {
    //INITIALISE
    let sdl = sdl2::init().expect("Could not initialize sdl!");
    let video = sdl.video().expect("Could not initialize video!");
    let mut ttf = sdl2::ttf::init().expect("Unable to initialise TTF system");
    let window_builder = video.window(
        "Ted's world",
        SCREEN_WIDTH * SCALING,
        SCREEN_HEIGHT * SCALING,
    );
    let window = window_builder
        .build()
        .expect("Could not initialize window!");
    let canvas_builder = window.into_canvas();
    let mut canvas = canvas_builder
        .target_texture()
        .accelerated()
        .build()
        .expect("Could not initialize canvas!");
    let texture_creator = canvas.texture_creator();
    let mut drawing = Drawing::new();
    let _events = sdl.event().expect("Could not initialize events!");
    let mut event_pump = sdl.event_pump().expect("Could not initialize event pump!");
    let mut event_state = events::EventState::new();

    let _images =
        sdl2::image::init(sdl2::image::InitFlag::PNG).expect("Could not initialize images!");
    let mut sprites = Sprites::load(&texture_creator, &mut ttf);

    let mut server_connection = network::connect_to_server();

    let mut game = client_side_component::Game::new();

    //RUN

    loop {
        let frame_start_time = time::Instant::now();
        parse_server_messages(
            &mut server_connection,
            &mut drawing,
            &mut game,
            &mut canvas,
            &texture_creator,
            &sprites,
        );
        event_state.poll(
            &mut event_pump,
            &mut server_connection,
            drawing.get_view_coords(),
        );
        game.step();
        {
            let (viewport_width, viewport_height) = game.get_viewport_dimensions();
            let mut buffer = texture_creator
                .create_texture(
                    Some(sdl2::pixels::PixelFormatEnum::RGBA8888),
                    sdl2::render::TextureAccess::Target,
                    viewport_width as u32,
                    viewport_height as u32,
                )
                .unwrap();
            canvas
                .with_texture_canvas(&mut buffer, |canvas| {
                    drawing.draw(
                        viewport_width,
                        viewport_height,
                        canvas,
                        &texture_creator,
                        &mut sprites,
                    );
                    game.draw(canvas, &mut sprites, &texture_creator);
                    drawing.abilities_overlay.draw(canvas, &sprites);
                })
                .unwrap();
            canvas.copy(&mut buffer, None, None).unwrap();
        }
        canvas.present();
        let frame_end_time = time::Instant::now();
        let elapsed = frame_end_time - frame_start_time;
        let target_time = time::Duration::from_millis(20);
        if target_time > elapsed {
            std::thread::sleep(target_time - elapsed)
        }
        if event_state.quitting {
            break;
        }
    }
}

fn parse_server_messages<'a, T: sdl2::render::RenderTarget>(
    server_connection: &mut network::ServerConnection,
    drawing: &mut Drawing<'a>,
    game: &mut Game,
    canvas: &mut sdl2::render::Canvas<T>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    sprites: &Sprites,
) {
    for message in server_connection.server_messages.try_iter() {
        match message {
            ServerMessage::UpdateGameObjects(u) => {
                drawing.update_view(&u.view_message);
                game.update_game_objects(u);
            }
            ServerMessage::UpdateComponents(u) => {
                game.update_components(u);
            }
            ServerMessage::ChunkInfo(ci) => {
                drawing.add_chunk(canvas, texture_creator, sprites, ci);
            }
            ServerMessage::ChunkUpdate(ci) => {
                drawing.update_chunk(canvas, sprites, ci);
            }
            ServerMessage::ChunkUnload(cu) => {
                drawing.unload_chunk(cu);
            }
            ServerMessage::SlotMapping(sm) => {
                println!("Slot map updated");
                drawing.abilities_overlay.update_slot_mapping(sm);
            }
            ServerMessage::SetNotifications(notifications) => {
                game.notifications = notifications.notifications;
            }
        }
    }
}
