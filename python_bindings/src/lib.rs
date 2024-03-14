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

mod py_classes;
mod py_functions;

use py_classes::{IpAddress, ScpiMessenger, ScpiNetworkMode};
use py_functions::{
    send_dutycycled_message, send_list_of_messages, send_message, send_repeated_message,
};
use pyo3::prelude::*;

#[pymodule]
fn py_scpi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send_message, m)?)?;
    m.add_function(wrap_pyfunction!(send_list_of_messages, m)?)?;
    m.add_function(wrap_pyfunction!(send_repeated_message, m)?)?;
    m.add_function(wrap_pyfunction!(send_dutycycled_message, m)?)?;
    m.add_class::<ScpiNetworkMode>()?;
    m.add_class::<ScpiMessenger>()?;
    m.add_class::<IpAddress>()?;
    Ok(())
}
