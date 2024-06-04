use game::*;
use std::net::*;
use std::sync::mpsc::*;
use std::time;
use wolf_interface::*;

struct Client {
    command_receiver: Receiver<Command>,
    server_message_sender: Sender<Vec<ServerMessage>>,
    player_id: Option<PlayerId>,
}

fn main() {
    let mut game = Game::new();
    game.initialise();

    let (client_sender, client_receiver) = channel();
    std::thread::spawn(move || net_thread(client_sender));

    let mut clients: Vec<Client> = Vec::new();

    loop {
        let start_time = time::Instant::now();
        game.step();
        for new_client in client_receiver.try_iter() {
            clients.push(new_client);
        }
        let mut clients_to_remove = Vec::new();
        for (index, mut client) in clients.iter_mut().enumerate() {
            if let Some(player_id) = client.player_id {
                let player = game
                    .player_system
                    .players
                    .get_mut(player_id)
                    .expect("Player got deleted before client was cleaned up!");
                for command in client.command_receiver.try_iter() {
                    player.commands.push(command);
                }
                let mut new_server_messages = Vec::new();
                std::mem::swap(&mut player.server_messages, &mut new_server_messages);
                if let Some(error) = client.server_message_sender.send(new_server_messages).err() {
                    println!("Error sending server messages due to {}", error);
                    clients_to_remove.push(index);
                }
            } else {
                let player_id = Player::create(&mut game);
                client.player_id = Some(player_id);
            }
        }
        clients_to_remove.sort();
        //loop in reverse order to avoid messing up indices
        for index in clients_to_remove.into_iter().rev() {
            clients.swap_remove(index);
        }
        let end_time = time::Instant::now();
        let elapsed = end_time - start_time;
        let target_time = time::Duration::from_millis(20);
        if target_time > elapsed {
            let sleep_time = target_time - elapsed;
            std::thread::sleep(sleep_time);
        } else if game.tick_counter % 100 == 0 {
            println!(
                "elapsed {} millseconds less than target_time",
                (elapsed - target_time).as_millis()
            );
        }
    }
}
fn net_thread(client_sender: Sender<Client>) {
    let host = std::fs::read_to_string("host.txt").expect("unable to read host.txt!");
    println!("Binding to {}", host);
    let listener = TcpListener::bind(host.trim()).expect("Unable to bind to port");
    for input_stream in listener.incoming() {
        if let Ok(input_stream) = input_stream {
            input_stream
                .set_nodelay(true)
                .expect("Could not disable Nagle's!");
            let (command_sender, command_receiver) = channel();
            let (server_message_sender, server_message_receiver) = channel();
            client_sender
                .send(Client {
                    command_receiver,
                    server_message_sender,
                    player_id: None,
                })
                .unwrap();
            std::thread::spawn(move || {
                client_thread(input_stream, command_sender, server_message_receiver)
            });
        }
    }
}
fn client_thread(
    input_stream: TcpStream,
    command_sender: Sender<Command>,
    server_message_receiver: Receiver<Vec<ServerMessage>>,
) {
    let output_stream = input_stream.try_clone();
    if let Ok(output_stream) = output_stream {
        std::thread::spawn(move || client_send_thread(output_stream, server_message_receiver));
        std::thread::spawn(move || client_receive_thread(input_stream, command_sender));
    }
}

fn client_send_thread(
    mut output_stream: TcpStream,
    server_message_receiver: Receiver<Vec<ServerMessage>>,
) {
    for server_messages in server_message_receiver.iter() {
        if let Err(e) = server_messages.wolf_serialise(&mut output_stream) {
            println!("Unable to write server messages due to {}", e);
            //TODO: clean up client
            break;
        }
    }
}
fn client_receive_thread(mut input_stream: TcpStream, command_sender: Sender<Command>) {
    loop {
        let commands = Vec::<Command>::wolf_deserialise(&mut input_stream);
        match commands {
            Ok(commands) => {
                for command in commands {
                    command_sender
                        .send(command)
                        .expect("Could not pass command");
                }
            }
            Err(e) => {
                println!("Unable to read server messages due to {}", e);
                break;
            }
        }
    }
}
