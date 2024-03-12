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

use std::{io::Error, net::IpAddr};

use messenger::Messenger;
use networking::NetworkMode;

pub fn send_scpi_message(
    message: &str,
    mode: NetworkMode,
    remote_client: &IpAddr,
    remote_port: u16,
    local_port: u16,
) -> Result<usize, Error> {
    let mut messenger: Messenger = Messenger::new(local_port, remote_port, remote_client, mode)?;
    messenger.send_message(message)
}

pub fn send_repeated_scpi_message(
    message: &str,
    mode: NetworkMode,
    remote_client: &IpAddr,
    remote_port: u16,
    local_port: u16,
    repititions: Option<usize>,
) -> Result<usize, Error> {
    let mut messenger: Messenger = Messenger::new(local_port, remote_port, remote_client, mode)?;

    match repititions {
        Some(number) => {
            let mut result: usize = 0;
            for _ in 0..number {
                result = messenger.send_message(message)?;
            }
            Ok(result)
        }
        None => loop {
            messenger.send_message(message)?;
        },
    }
}
