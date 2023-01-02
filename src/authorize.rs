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

use std::collections::HashMap;

use serde::Deserialize;

use crate::config::NordigenConfig;

#[derive(Deserialize)]
pub struct AuthorizeReply {
    pub access: String,
    pub access_expires: u32,
    pub refresh: String,
    pub refresh_expires: u32,
}

#[derive(Deserialize)]
struct RefreshReply {
    pub access: String,
    pub access_expires: u32,
}

pub async fn authorize(
    config: &NordigenConfig,
) -> Result<AuthorizeReply, String> {
    let mut map: HashMap<&str, &String> = HashMap::new();
    map.insert("secret_id", &config.secret_id);
    map.insert("secret_key", &config.secret_key);

    let client = reqwest::Client::new();
    let res = match client
        .post("https://ob.nordigen.com/api/v2/token/new/")
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
    {
        Err(error) => {
            return Err(format!("Unable to obtain token: {}", error));
        }
        Ok(res) => res,
    };

    let value: AuthorizeReply = match res.json::<AuthorizeReply>().await {
        Err(error) => {
            return Err(format!("Unable to obtain response value: {error}"));
        }
        Ok(res) => res,
    };

    Ok(value)
}

pub async fn refresh(refresh_token: &String) -> Result<(String, u32), String> {
    let mut map: HashMap<&str, &String> = HashMap::new();
    map.insert("refresh", refresh_token);

    let client = reqwest::Client::new();
    let res = match client
        .post("https://ob.nordigen.com/api/v2/token/refresh/")
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&map)
        .send()
        .await
    {
        Err(error) => {
            return Err(format!("Unable to refresh token: {}", error));
        }
        Ok(res) => res,
    };

    let value: RefreshReply = match res.json::<RefreshReply>().await {
        Err(error) => {
            return Err(format!("Unable to obtain response value: {}", error));
        }
        Ok(res) => res,
    };

    Ok((value.access, value.access_expires))
}
