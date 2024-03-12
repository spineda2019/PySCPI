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

#[cfg(test)]
mod tests {
    use std::{
        net::{AddrParseError, IpAddr},
        str::FromStr,
    };

    use crate::{networking::NetworkMode, send_repeated_scpi_message, send_scpi_message};

    #[test]
    fn test_send_udp_message() -> Result<(), AddrParseError> {
        const MESSAGE: &str = "*IDN";
        const MODE: NetworkMode = NetworkMode::Udp;
        let remote_client: IpAddr = IpAddr::from_str("192.168.1.70")?;
        const REMOTE_PORT: u16 = 5025;
        const LOCAL_PORT: u16 = 42;

        assert!(send_scpi_message(MESSAGE, MODE, &remote_client, REMOTE_PORT, LOCAL_PORT).is_ok());

        Ok(())
    }

    #[test]
    fn test_repeated_udp_messages() -> Result<(), AddrParseError> {
        const MESSAGE: &str = "*IDN";
        const MODE: NetworkMode = NetworkMode::Udp;
        let remote_client: IpAddr = IpAddr::from_str("192.168.1.70")?;
        const REMOTE_PORT: u16 = 5025;
        const LOCAL_PORT: u16 = 42;

        assert!(send_repeated_scpi_message(
            MESSAGE,
            MODE,
            &remote_client,
            REMOTE_PORT,
            LOCAL_PORT,
            Some(10)
        )
        .is_ok());

        Ok(())
    }
}
