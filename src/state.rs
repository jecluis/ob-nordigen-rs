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
    written_at: DateTime<Utc>,
}

impl NordigenState {
    pub fn parse(path: &std::path::PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!(
                "State file at {} does not exist!",
                path.display()
            ));
        }

        let contents = match std::fs::read_to_string(path) {
            Err(error) => {
                return Err(format!(
                    "Error reading file at {}: {}",
                    path.display(),
                    error
                ));
            }
            Ok(value) => value,
        };

        let state: NordigenState = match serde_json::from_str(&contents) {
            Err(error) => {
                return Err(format!(
                    "Unable to parse state file at {}: {}",
                    path.display(),
                    error
                ))
            }
            Ok(value) => value,
        };

        Ok(state)
    }

    pub fn write(
        path: &std::path::PathBuf,
        token: String,
        refresh: String,
        token_ttl: u32,
        refresh_ttl: u32,
    ) -> Result<Self, String> {
        let state: NordigenState = NordigenState {
            token,
            token_expires: token_ttl,
            refresh_token: refresh,
            refresh_expires: refresh_ttl,
            written_at: Utc::now(),
        };

        let buffer = match std::fs::File::create(path) {
            Err(err) => {
                return Err(format!(
                    "Unable to open state file for writing: {}",
                    err
                ));
            }
            Ok(res) => res,
        };

        match serde_json::to_writer_pretty(buffer, &state) {
            Err(err) => {
                return Err(format!("Unable to write state to disk: {}", err));
            }
            Ok(_) => {}
        };

        Ok(state)
    }

    pub fn token_expires_on(&self) -> DateTime<Utc> {
        self.written_at
            .checked_add_signed(Duration::seconds(self.token_expires.into()))
            .expect("Unable to obtain end date!")
    }

    pub fn refresh_expires_on(&self) -> DateTime<Utc> {
        self.written_at
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
