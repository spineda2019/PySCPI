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

use std::{io::Error, net::IpAddr, str::FromStr};

use pyo3::{pyclass, pymethods};
use scpi::messenger::Messenger;
use scpi::networking::NetworkMode;

#[derive(Clone)]
#[pyclass]
enum ScpiNetworkMode {
    Udp = 0,
    Tcp = 1,
    UdpMulticast = 2,
    TcpMulticast = 3,
}

#[pyclass]
struct ScpiMessenger {
    inner: Messenger,
}

#[pymethods]
impl ScpiMessenger {
    #[new]
    fn new(
        local_port: u16,
        remote_port: u16,
        remote_client: &str,
        mode: ScpiNetworkMode,
    ) -> Result<Self, Error> {
        let remote_client_address: IpAddr = match IpAddr::from_str(remote_client) {
            Ok(x) => x,
            Err(_) => {
                const MSG: &str = "Error Parsing ip address";
                eprintln!("{}", MSG);
                return Err(Error::new(std::io::ErrorKind::Interrupted, MSG));
            }
        };

        let scpi_mode: NetworkMode = match mode {
            ScpiNetworkMode::Udp => NetworkMode::Udp,
            ScpiNetworkMode::Tcp => NetworkMode::Tcp,
            ScpiNetworkMode::UdpMulticast => NetworkMode::UdpMulticast,
            ScpiNetworkMode::TcpMulticast => NetworkMode::TcpMulticast,
        };

        let inner: Messenger =
            Messenger::new(local_port, remote_port, &remote_client_address, scpi_mode)?;

        Ok(Self { inner })
    }
}
