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
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn send_response(stream: &mut TcpStream) {
    let response_vec = vec![
        "HTTP/1.1 200 OK",
        "Content-Type: text/html; charset=UTF-8",
        "",
        "<html>",
        "<body>",
        "<h2>Thank You!</h2>",
        "<p>You can now go back to the tool :)</p>",
        "</body>",
        "</html>",
        "",
    ];
    let response = response_vec.join("\r\n");
    match stream.write(response.as_bytes()) {
        Ok(_) => {}
        Err(error) => {
            println!("Error sending response: {}", error);
        }
    }
}

fn parse_request(req: &Vec<String>) -> Result<HashMap<&str, &str>, String> {
    if req.len() == 0 {
        return Err(String::from("empty request"));
    }
    let request_line: Vec<_> = req[0].split_whitespace().collect();
    if request_line.len() < 3 {
        return Err(format!("Unexpected request line: {}", req[0]));
    }
    let (method, target) = (request_line[0], request_line[1]);
    if method.to_lowercase() != "get" {
        return Err(format!("Unexpected method: {}", method));
    }

    let p = target.find("?");
    if p.is_none() {
        return Err(String::from("No parameters provided!"));
    }
    let pos = p.unwrap();
    if target.len() < pos + 1 {
        return Err(String::from("Parameters not provided."));
    }
    let params_str = &target[pos + 1..];
    // println!("Parameters: {}", params_str);
    let mut map: HashMap<&str, &str> = HashMap::new();
    params_str.split("&").for_each(|v| {
        let kv: Vec<&str> = v.split("=").collect();
        if kv.len() != 2 {
            return;
        }
        let (key, value) = (kv[0], kv[1]);
        map.insert(key, value);
    });
    Ok(map)
}

pub fn wait_for_response() -> Result<String, String> {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    let mut stream = listener
        .incoming()
        .filter_map(Result::ok)
        .take(1)
        .next()
        .unwrap();
    let reader = BufReader::new(&mut stream);
    let request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let res: Result<String, String> = match parse_request(&request) {
        Err(error) => Err(format!("Error obtaining ref: {}", error)),
        Ok(map) => {
            if let Some(val) = map.get("ref") {
                Ok(String::from(*val))
            } else {
                Err(String::from("Callback did not provide ref"))
            }
        }
    };
    send_response(&mut stream);
    return res;
}
