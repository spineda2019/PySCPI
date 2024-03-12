/*
    Copyright 2024 Sebastian Pineda (spineda@wpi.edu)

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

mod messenger;
mod networking;
mod unit_tests;

use std::{
    io::{Error, ErrorKind, Write},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    str::FromStr,
};

use messenger::Messenger;
use networking::NetworkMode;

pub fn send_scpi_message(
    message: &str,
    mode: &NetworkMode,
    remote_client: &IpAddr,
    remote_port: u16,
    local_port: u16,
) -> Result<usize, Error> {
    let clean_message: String = format!("{}\r\n", message.trim());
    let scpi_message: &[u8] = clean_message.as_bytes();
    let local_host: IpAddr = match IpAddr::from_str("0.0.0.0") {
        Ok(x) => x,
        Err(_) => {
            let msg: &str = "Creating localhost object failed...";
            eprint!("{}", msg);
            return Err(Error::new(ErrorKind::Interrupted, msg));
        }
    };
    let local_address = SocketAddr::new(local_host, local_port);

    let remote_address = SocketAddr::new(*remote_client, remote_port);

    match mode {
        NetworkMode::Udp => {
            let local_socket: UdpSocket = UdpSocket::bind(local_address)?;
            Ok(local_socket.send_to(scpi_message, remote_address)?)
        }
        NetworkMode::Tcp => {
            let mut local_socket: TcpStream = TcpStream::connect(remote_address)?;
            local_socket.write_all(scpi_message)?;
            Ok(scpi_message.len())
        }
        NetworkMode::UdpMulticast => todo!(),
        NetworkMode::TcpMulticast => todo!(),
    }
}

pub fn send_repeated_scpi_message(
    message: &str,
    mode: NetworkMode,
    remote_client: &IpAddr,
    remote_port: u16,
    local_port: u16,
    repititions: Option<usize>,
) -> Result<usize, Error> {
    let messenger: Messenger = Messenger::new(local_port, remote_port, remote_client, mode)?;

    match repititions {
        Some(number) => {
            let result: usize = 0;
            for _ in 0..number {
                // This is not as good for performance
                todo!();
            }
            Ok(result)
        }
        None => loop {
            todo!()
        },
    }
}
