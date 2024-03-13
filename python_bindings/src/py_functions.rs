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

use std::net::IpAddr;
use std::str::FromStr;

use pyo3::pyfunction;
use scpi::duty_cycle::DutyCycleMessage;
use scpi::networking::NetworkMode;
use scpi::send_duty_cycled_message as lib_send_duty_cycled_message;
use scpi::send_repeated_scpi_message as lib_send_repeated_scpi_message;
use scpi::send_scpi_message as lib_send_scpi_message;

#[pyfunction]
pub fn send_dutycycled_message(
    messages: (&str, &str),
    times: (u64, u64),
    mode: u8,
    remote_client: &str,
    remote_port: u16,
    local_port: u16,
) -> isize {
    let network_mode: NetworkMode = match mode {
        0 => NetworkMode::Udp,
        1 => NetworkMode::Tcp,
        2 => NetworkMode::UdpMulticast,
        3 => NetworkMode::TcpMulticast,
        _ => return -1,
    };

    let remote_client_address: IpAddr = match IpAddr::from_str(remote_client) {
        Ok(x) => x,
        Err(_) => return -2,
    };

    let (first_message, second_message): (&str, &str) = messages;
    let (first_time, second_time): (u64, u64) = times;

    let dutycycled_message: DutyCycleMessage =
        DutyCycleMessage::new(first_time, second_time, first_message, second_message);

    match lib_send_duty_cycled_message(
        &dutycycled_message,
        network_mode,
        &remote_client_address,
        remote_port,
        local_port,
    ) {
        Ok(_) => 0, // impossible
        Err(_) => -3,
    }
}

#[pyfunction]
pub fn send_message(
    message: &str,
    mode: u8,
    remote_client: &str,
    remote_port: u16,
    local_port: u16,
) -> isize {
    let network_mode: NetworkMode = match mode {
        0 => NetworkMode::Udp,
        1 => NetworkMode::Tcp,
        2 => NetworkMode::UdpMulticast,
        3 => NetworkMode::TcpMulticast,
        _ => return -1,
    };

    let remote_client_address: IpAddr = match IpAddr::from_str(remote_client) {
        Ok(x) => x,
        Err(_) => return -2,
    };

    match lib_send_scpi_message(
        message,
        network_mode,
        &remote_client_address,
        remote_port,
        local_port,
    ) {
        Ok(x) => x as isize,
        Err(_) => -3,
    }
}

#[pyfunction]
#[pyo3(signature = (message, mode, remote_client, remote_port, local_port, repititions=None))]
pub fn send_repeated_message(
    message: &str,
    mode: u8,
    remote_client: &str,
    remote_port: u16,
    local_port: u16,
    repititions: Option<usize>,
) -> isize {
    let network_mode: NetworkMode = match mode {
        0 => NetworkMode::Udp,
        1 => NetworkMode::Tcp,
        2 => NetworkMode::UdpMulticast,
        3 => NetworkMode::TcpMulticast,
        _ => return -1,
    };

    let remote_client_address: IpAddr = match IpAddr::from_str(remote_client) {
        Ok(x) => x,
        Err(_) => return -2,
    };

    match lib_send_repeated_scpi_message(
        message,
        network_mode,
        &remote_client_address,
        remote_port,
        local_port,
        repititions,
    ) {
        Ok(x) => x as isize,
        Err(_) => -3,
    }
}
