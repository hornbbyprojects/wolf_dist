use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc;
use std::time::Duration;
use wolf_interface::*;

pub struct ServerConnection {
    pub server_messages: mpsc::Receiver<ServerMessage>,
    pub commands: mpsc::Sender<Command>,
}
pub fn connect_to_server() -> ServerConnection {
    let (server_message_sender, server_message_receiver) = mpsc::channel();
    let (command_sender, command_receiver) = mpsc::channel();
    std::thread::spawn(move || net_thread(server_message_sender, command_receiver));
    ServerConnection {
        server_messages: server_message_receiver,
        commands: command_sender,
    }
}

fn net_thread(
    server_message_sender: mpsc::Sender<ServerMessage>,
    command_receiver: mpsc::Receiver<Command>,
) {
    println!("Starting net thread...");
    let mut host = std::fs::read_to_string("host.txt").expect("Unable to read host.txt!");
    host.retain(|c| !c.is_whitespace());
    println!("Connecting to {}", host);
    let out_stream = TcpStream::connect(host).expect("Unable to connect to server!");
    out_stream
        .set_nodelay(true)
        .expect("Could not disable Nagle's!");
    let mut in_stream = out_stream.try_clone().expect("Unable to clone TCP stream!");
    std::thread::spawn(move || send_thread(out_stream, command_receiver));
    loop {
        let server_messages = Vec::<ServerMessage>::wolf_deserialise(&mut in_stream)
            .expect("Failed tto read server messages!");
        for message in server_messages {
            server_message_sender
                .send(message)
                .expect("Failed to send server messages across threads!");
        }
    }
}

fn send_thread(mut out_stream: TcpStream, command_receiver: mpsc::Receiver<Command>) {
    loop {
        let commands = command_receiver.try_iter().collect::<Vec<Command>>();
        let mut buffer = Vec::new();
        commands
            .wolf_serialise(&mut buffer)
            .expect("Failed to serialise commands!");
        out_stream
            .write(&buffer)
            .expect("Failed to write to server!");
        std::thread::sleep(Duration::from_millis(20));
    }
}
