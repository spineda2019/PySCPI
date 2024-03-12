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

use std::{
    io::{Error, ErrorKind},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    str::FromStr,
};

use crate::networking::{NetworkMode, NetworkSender};

pub struct Messenger {
    destination_address: SocketAddr,
    local_address: SocketAddr,
    sending_socket: NetworkSender,
}

impl Messenger {
    pub fn new(
        local_port: u16,
        remote_port: u16,
        remote_client: &IpAddr,
        mode: NetworkMode,
    ) -> Result<Self, Error> {
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
                Ok(Self {
                    destination_address: remote_address,
                    local_address,
                    sending_socket: NetworkSender::Udp(local_socket),
                })
            }
            NetworkMode::Tcp => {
                let local_socket: TcpStream = TcpStream::connect(remote_address)?;
                Ok(Self {
                    destination_address: remote_address,
                    local_address,
                    sending_socket: NetworkSender::Tcp(local_socket),
                })
            }
            NetworkMode::UdpMulticast => todo!(),
            NetworkMode::TcpMulticast => todo!(),
        }
    }

    pub fn send_message(&self, message: &str) -> Result<usize, Error> {
        match &self.sending_socket {
            NetworkSender::Udp(x) => todo!(),
            NetworkSender::Tcp(y) => todo!(),
        }
    }
}
