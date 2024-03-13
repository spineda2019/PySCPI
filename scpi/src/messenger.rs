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
    io::{Error, ErrorKind, Write},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    str::FromStr,
};

use crate::duty_cycle::DutyCycleMessage;
use crate::networking::{NetworkMode, NetworkSender};

pub struct Messenger {
    destination_address: SocketAddr,
    sending_socket: NetworkSender,
}

impl Messenger {
    pub fn new(
        local_port: u16,
        remote_port: u16,
        remote_client: &IpAddr,
        mode: &NetworkMode,
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
                    sending_socket: NetworkSender::Udp(local_socket),
                })
            }
            NetworkMode::Tcp => {
                let local_socket: TcpStream = TcpStream::connect(remote_address)?;
                Ok(Self {
                    destination_address: remote_address,
                    sending_socket: NetworkSender::Tcp(local_socket),
                })
            }
            NetworkMode::UdpMulticast => {
                let local_socket: UdpSocket = UdpSocket::bind(local_address)?;
                match [&local_host, remote_client] {
                    [IpAddr::V4(x), IpAddr::V4(y)] => {
                        local_socket.join_multicast_v4(y, x)?;
                    }
                    [_, _] => {
                        const MESSAGE: &str = "Ipv6 Addresses not yet supported";
                        eprint!("Multicast Join Failed: {}", MESSAGE);
                        return Err(Error::new(ErrorKind::Interrupted, MESSAGE));
                    }
                }

                Ok(Self {
                    destination_address: remote_address,
                    sending_socket: NetworkSender::Udp(local_socket),
                })
            }
            NetworkMode::TcpMulticast => {
                const MESSAGE: &str = "Tcp Multicast not yet supported";
                eprint!("Multicast Join Failed: {}", MESSAGE);
                Err(Error::new(ErrorKind::Interrupted, MESSAGE))
            }
        }
    }

    pub fn send_message(&mut self, message: &str) -> Result<usize, Error> {
        let clean_message: String = format!("{}\r\n", message.trim());
        let scpi_message: &[u8] = clean_message.as_bytes();
        match &mut self.sending_socket {
            NetworkSender::Udp(x) => Ok(x.send_to(scpi_message, self.destination_address)?),
            NetworkSender::Tcp(y) => {
                y.write_all(scpi_message)?;
                Ok(scpi_message.len())
            }
        }
    }

    pub fn send_list_of_messages(&mut self, messages: &[&str]) -> Result<(), Error> {
        for message in messages {
            self.send_message(message)?;
        }

        Ok(())
    }

    pub fn send_duty_cycled_message(&mut self, message: &DutyCycleMessage) -> Result<(), Error> {
        let (first_time, second_time): (u64, u64) = message.get_times();
        let (first_message, second_message): (&str, &str) = message.get_messages();

        let first_interval = std::time::Duration::from_micros(first_time);
        let second_interval = std::time::Duration::from_micros(second_time);

        loop {
            let start: std::time::Instant = std::time::Instant::now();
            self.send_message(first_message)?;
            while start.elapsed() < first_interval {
                std::hint::spin_loop();
            }

            let start: std::time::Instant = std::time::Instant::now();
            self.send_message(second_message)?;
            while start.elapsed() < second_interval {
                std::hint::spin_loop();
            }
        }
    }
}
