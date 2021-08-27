/*
MIT License

Copyright (c) 2021 P3qch

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::io::{Read, Write};
use std::net::TcpStream;

use serde_json::json;

use crate::cards::Card;
use crate::player::Player;

pub struct Client {
    ip: String,
}
impl Client {
    pub fn new(ip: String) -> Self {
        Client { ip }
    }
    pub fn join(&self, name: &str) -> Result<(), String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "join",
                    "name": name,
                })
                .to_string();
                stream.write_all(request.as_bytes()).unwrap();

                Ok(())
            }
            Err(_) => Err(String::from("Faild connecting")),
        }
    }

    pub fn take_card(&self, name: &str, num: u8) -> Result<(), String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "take_cards",
                    "name": name,
                    "num": num
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();

                Ok(())
            }
            Err(_) => Err(String::from("Failed connecting")),
        }
    }

    pub fn get_cards_for(&self, name: &str) -> Result<Vec<crate::cards::Card>, String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "get_cards",
                    "name": name,
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();
                let mut buf = [0u8; 2048];

                let size = stream.read(&mut buf).unwrap();

                let cards: Vec<crate::cards::Card> = serde_json::from_slice(&buf[0..size]).unwrap();

                Ok(cards)
            }
            Err(_) => Err(String::from("Failed connecting")),
        }
    }

    pub fn get_players(&self) -> Result<Vec<Player>, String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "get_players",
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();
                let mut buf = [0; 2048];

                let size = stream.read(&mut buf).unwrap();

                let players = serde_json::from_slice(&buf[0..size]).unwrap();

                Ok(players)
            }
            Err(_) => Err(String::from("Failed connecting")),
        }
    }

    pub fn use_card(&self, name: &str, card_index: usize) -> Result<(), u8> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "use_card",
                    "name": name,
                    "card_index": card_index,
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();

                let mut buf = [0u8];
                stream.read_exact(&mut buf).unwrap();

                if buf[0] == 0 {
                    Ok(())
                } else {
                    Err(buf[0])
                }
            }
            Err(_) => Err(4),
        }
    }

    pub fn current_turn(&self) -> Result<String, String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "current_turn",
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();

                let mut buf = [0u8; 128];
                let size = stream.read(&mut buf).unwrap();

                Ok(String::from_utf8(buf[0..size].to_vec()).unwrap())
            }
            Err(_) => Err(String::from("Failed connecting")),
        }
    }

    pub fn get_plus(&self) -> Result<u8, String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "get_plus",
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();

                let mut buf = [0u8];
                stream.read_exact(&mut buf).unwrap();

                Ok(buf[0])
            }
            Err(_) => Err(String::from("Failed connecting")),
        }
    }

    pub fn reset_plus(&self) -> Result<u8, String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "reset_plus",
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();

                Ok(0)
            }
            Err(_) => Err(String::from("Failed connecting")),
        }
    }
    pub fn top_card(&self) -> Result<Card, String> {
        match TcpStream::connect(&self.ip[..]) {
            Ok(mut stream) => {
                let request: String = json!({
                    "request_type": "top_card",
                })
                .to_string();

                stream.write_all(request.as_bytes()).unwrap();

                let mut buf = [0u8; 128];
                let size = stream.read(&mut buf).unwrap();

                let card: Card = serde_json::from_slice(&buf[0..size]).unwrap();

                Ok(card)
            }
            Err(_) => Err(String::from("Failed connecting")),
        }
    }

    pub fn cycle_color_down(&self, name: &str, card_index: usize) {
        if let Ok(mut stream) = TcpStream::connect(&self.ip[..]) {
            let request: String = json!({
                "request_type": "cycle_color_down",
                "name": name,
                "card_index": card_index
            })
            .to_string();

            stream.write_all(request.as_bytes()).unwrap();
        }
    }

    pub fn cycle_color_up(&self, name: &str, card_index: usize) {
        if let Ok(mut stream) = TcpStream::connect(&self.ip[..]) {
            let request: String = json!({
                "request_type": "cycle_color_up",
                "name": name,
                "card_index": card_index
            })
            .to_string();

            stream.write_all(request.as_bytes()).unwrap();
        }
    }
}
