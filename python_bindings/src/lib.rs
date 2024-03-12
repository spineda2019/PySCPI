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

use std::isize;
use std::net::IpAddr;
use std::str::FromStr;

use pyo3::prelude::*;
use scpi::networking::NetworkMode;
use scpi::send_repeated_scpi_message;
use scpi::send_scpi_message;

#[pyfunction]
fn send_message(
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

    match send_scpi_message(
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
fn send_repeated_message(
    message: &str,
    mode: u8,
    remote_client: &str,
    remote_port: u16,
    local_port: u16,
    repititions: isize,
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

    let message_repititons: Option<usize> = match repititions {
        x if x >= 0 => Some(x as usize),
        _ => None,
    };

    match send_repeated_scpi_message(
        message,
        network_mode,
        &remote_client_address,
        remote_port,
        local_port,
        message_repititons,
    ) {
        Ok(x) => x as isize,
        Err(_) => -3,
    }
}

#[pymodule]
fn py_scpi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_message, m)?)?;
    m.add_function(wrap_pyfunction!(send_repeated_message, m)?)?;
    Ok(())
}
