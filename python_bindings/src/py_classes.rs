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

use pyo3::{pyclass, pymethods};
use std::net::AddrParseError;
use std::{io::Error, net::IpAddr, str::FromStr};

use scpi::duty_cycle::DutyCycleMessage;
use scpi::messenger::Messenger;
use scpi::networking::NetworkMode;

#[derive(Clone)]
#[pyclass]
pub enum ScpiNetworkMode {
    Udp,
    Tcp,
    UdpMulticast,
    TcpMulticast,
}

#[pymethods]
impl ScpiNetworkMode {
    #[new]
    fn new(mode: u8) -> Result<ScpiNetworkMode, Error> {
        match mode {
            0 => Ok(Self::Udp),
            1 => Ok(Self::Tcp),
            2 => Ok(Self::UdpMulticast),
            3 => Ok(Self::TcpMulticast),
            _ => {
                const MSG: &str = "Not a valid enum in range [0, 4]";
                eprintln!("{}", MSG);
                Err(Error::new(std::io::ErrorKind::NotFound, MSG))
            }
        }
    }
}

#[pyclass]
pub struct IpAddress {
    pub address: IpAddr,
}

#[pymethods]
impl IpAddress {
    #[new]
    pub fn new(address: &str) -> Result<Self, AddrParseError> {
        let rust_address: IpAddr = IpAddr::from_str(address)?;
        Ok(Self {
            address: rust_address,
        })
    }
}

#[pyclass]
pub struct ScpiMessenger {
    inner: Messenger,
}

#[pymethods]
impl ScpiMessenger {
    #[new]
    fn new(
        local_port: u16,
        remote_port: u16,
        remote_client: &IpAddress,
        mode: ScpiNetworkMode,
    ) -> Result<Self, Error> {
        let scpi_mode: NetworkMode = match mode {
            ScpiNetworkMode::Udp => NetworkMode::Udp,
            ScpiNetworkMode::Tcp => NetworkMode::Tcp,
            ScpiNetworkMode::UdpMulticast => NetworkMode::UdpMulticast,
            ScpiNetworkMode::TcpMulticast => NetworkMode::TcpMulticast,
        };

        let inner: Messenger =
            Messenger::new(local_port, remote_port, &remote_client.address, &scpi_mode)?;

        Ok(Self { inner })
    }

    fn send_message(&mut self, message: &str) -> isize {
        match self.inner.send_message(message) {
            Ok(x) => x as isize,
            Err(_) => -1,
        }
    }

    fn send_duty_cycled_message(
        &mut self,
        messages: (&str, &str),
        microsecond_times: (u64, u64),
    ) -> Result<(), Error> {
        let message: DutyCycleMessage = DutyCycleMessage::new(
            microsecond_times.0,
            microsecond_times.1,
            messages.0,
            messages.1,
        );

        self.inner.send_duty_cycled_message(&message)
    }
}
