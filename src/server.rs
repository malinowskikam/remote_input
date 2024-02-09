use std::{
    net::UdpSocket,
    thread::{spawn, JoinHandle},
};
use crate::{events::{lmb_down, lmb_up, move_mouse}, packet::Packet};

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub fn listen(&self) -> JoinHandle<()> {
        let port = self.port;
        spawn(move || {
            println!("Listening on port: {}", port);

            let socket = match UdpSocket::bind(("0.0.0.0", port)) {
                Ok(socket) => socket,
                Err(e) => panic!("Failed to bind 127.0.0.1:({}): {:?}", port, e),
            };

            loop {
                let mut buff = [0u8; 1024];
                let (msg_size, source) = match socket.recv_from(&mut buff) {
                    Ok((msg_size, source)) => (msg_size, source),
                    Err(e) => panic!("Failed to receive data: {}", e),
                };

                let p = Packet::from_bytes(&buff[..msg_size]);
                match p {
                    Packet::Exit => break,
                    Packet::Ping => {
                        // ping
                        socket.send_to(&[1u8; 1], source).unwrap();
                    },
                    Packet::MouseMove { dx, dy } => move_mouse(dx as i32, dy as i32),
                    Packet::LMBDown => lmb_down(),
                    Packet::LMBUp => lmb_up(),
                    Packet::Unknown => println!("unknown bytes: {:?}", &buff[..msg_size]),
                }
            }
            println!("Stopping server");
        })
    }
}
