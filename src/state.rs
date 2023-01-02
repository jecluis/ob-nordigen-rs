// ob-nordigen-rs: Nordigen Open Banking API
// Copyright 2023 Joao Eduardo Luis <joao@abysmo.io>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NordigenState {
    pub token: String,
    pub token_expires: u32,
    pub refresh_token: String,
    pub refresh_expires: u32,
    pub last_updated: DateTime<Utc>,
}

impl NordigenState {
    pub fn new(
        token: String,
        token_expires: u32,
        refresh_token: String,
        refresh_expires: u32,
    ) -> NordigenState {
        NordigenState {
            token,
            token_expires,
            refresh_token,
            refresh_expires,
            last_updated: Utc::now(),
        }
    }

    pub fn token_expires_on(&self) -> DateTime<Utc> {
        self.last_updated
            .checked_add_signed(Duration::seconds(self.token_expires.into()))
            .expect("Unable to obtain end date!")
    }

    pub fn refresh_expires_on(&self) -> DateTime<Utc> {
        self.last_updated
            .checked_add_signed(Duration::seconds(self.refresh_expires.into()))
            .expect("Unable to obtain end date!")
    }

    pub fn is_token_expired(&self) -> bool {
        self.token_expires_on() < Utc::now()
    }

    pub fn is_refresh_expired(&self) -> bool {
        self.refresh_expires_on() < Utc::now()
    }
}
