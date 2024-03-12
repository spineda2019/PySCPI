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

pub struct DutyCycleMessage<'a> {
    first_microsecond_period: u64,
    second_microsecond_period: u64,
    first_message: &'a str,
    second_message: &'a str,
}

impl<'a> DutyCycleMessage<'a> {
    pub fn new(
        first_time: u64,
        second_time: u64,
        first_message: &'a str,
        second_message: &'a str,
    ) -> Self {
        Self {
            first_microsecond_period: first_time,
            second_microsecond_period: second_time,
            first_message,
            second_message,
        }
    }
}

/* ********************************************************************************************** */
/*                                       Boilerplate Getters                                      */
/* ********************************************************************************************** */

impl<'a> DutyCycleMessage<'a> {
    pub fn get_times(&self) -> (u64, u64) {
        (
            self.first_microsecond_period,
            self.second_microsecond_period,
        )
    }

    pub fn get_messages(&self) -> (&'a str, &'a str) {
        (self.first_message, self.second_message)
    }
}
