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

use serde_json::Value;
use std::{
    io::{Read, Write},
    sync::{Arc, Mutex},
};

use crate::player::Player;

fn handle_connection(mut stream: std::net::TcpStream, game: Arc<Mutex<crate::game::Game>>) {
    let mut buf = [0u8; 1024];

    while match stream.read(&mut buf) {
        Ok(size) => {
            if size == 0 {
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                return;
            }

            let json: Value = serde_json::from_slice(&buf[0..(size)]).unwrap();
            let mut game_lock = game.lock().unwrap();

            match json["request_type"].as_str().unwrap() {
                "join" => {
                    let name = json["name"]
                        .as_str()
                        .unwrap()
                        .trim_matches('\"')
                        .to_string();
                    if (*game_lock)
                        .players
                        .iter()
                        .filter(|val| val.name == name)
                        .count()
                        == 0
                    {
                        let p = Player::new(name);

                        (*game_lock).add_player(p);
                    }
                }
                "get_players" => {
                    let players_json = serde_json::json!((*game_lock).players).to_string();
                    stream.write_all(players_json.as_bytes()).unwrap();
                }
                "reset_plus" => {
                    (*game_lock).plus = 0;
                }
                "take_cards" => {
                    let name = json["name"]
                        .as_str()
                        .unwrap()
                        .trim_matches('\"')
                        .to_string();
                    let num = json["num"].as_i64().unwrap();
                    let id = (*game_lock).players.iter().position(|val| val.name == name);

                    if let Some(id) = id {
                        if (*game_lock).current_turn == id as u8 {
                            for _ in 0..num {
                                (*game_lock).take_card(id as u8);
                            }
                            (*game_lock).cycle_turn();
                        }
                    }
                }
                "use_card" => {
                    let name = json["name"]
                        .as_str()
                        .unwrap()
                        .trim_matches('\"')
                        .to_string();
                    let card_index = json["card_index"].as_i64().unwrap() as usize;
                    let player_index = (*game_lock)
                        .players
                        .iter()
                        .position(|val| val.name == name)
                        .unwrap();

                    let his_turn = (*game_lock).current_turn == player_index as u8;
                    if his_turn {
                        if (*game_lock)
                            .can_use((*game_lock).players[player_index].cards[card_index])
                        {
                            let card = (*game_lock).players[player_index].take_card(card_index);
                            let result = (*game_lock).play_card(card);
                            stream
                                .write_all(if result { &[0u8] } else { &[1u8] })
                                .unwrap();
                        } else {
                            stream.write_all(&[1u8]).unwrap();
                        }
                    } else {
                        stream.write_all(&[1u8]).unwrap();
                    }
                }
                "get_cards" => {
                    let name = json["name"]
                        .as_str()
                        .unwrap()
                        .trim_matches('\"')
                        .to_string();
                    let player_index = (*game_lock)
                        .players
                        .iter()
                        .position(|val| val.name == name)
                        .unwrap();

                    let cards = &(*game_lock).players[player_index].cards;

                    let json = serde_json::json!(cards).to_string();

                    stream.write_all(json.as_bytes()).unwrap();
                }
                "get_plus" => {
                    let response = [(*game_lock).plus];
                    stream.write_all(&response).unwrap();
                }
                "get_card_num" => {
                    let name = json["name"]
                        .as_str()
                        .unwrap()
                        .trim_matches('\"')
                        .to_string();
                    let player_index = (*game_lock)
                        .players
                        .iter()
                        .position(|val| val.name == name)
                        .unwrap();

                    let cards = (*game_lock).players[player_index].card_num();

                    let buf = [cards as u8];

                    stream.write_all(&buf).unwrap();
                }
                "current_turn" => {
                    let player_index = (*game_lock).current_turn;
                    stream
                        .write_all((*game_lock).players[player_index as usize].name.as_bytes())
                        .unwrap();
                }
                "top_card" => {
                    let json: String = serde_json::json!(&(*game_lock).last_card).to_string();

                    stream.write_all(json.as_bytes()).unwrap();
                }
                "cycle_color_up" => {
                    let name = json["name"]
                        .as_str()
                        .unwrap()
                        .trim_matches('\"')
                        .to_string();
                    let card_index = json["card_index"].as_i64().unwrap() as usize;
                    let player_index = (*game_lock)
                        .players
                        .iter()
                        .position(|val| val.name == name)
                        .unwrap();

                    (*game_lock).players[player_index].cards[card_index].cycle_colors_up();
                }
                "cycle_color_down" => {
                    let name = json["name"]
                        .as_str()
                        .unwrap()
                        .trim_matches('\"')
                        .to_string();
                    let card_index = json["card_index"].as_i64().unwrap() as usize;
                    let player_index = (*game_lock)
                        .players
                        .iter()
                        .position(|val| val.name == name)
                        .unwrap();

                    (*game_lock).players[player_index].cards[card_index].cycle_colors_down();
                }
                _ => (),
            };

            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn start_server(ip: String) {
    let listener =
        std::net::TcpListener::bind(&ip[..]).expect(&format!("Failed listening on {}", &ip)[..]);

    let game = Arc::new(Mutex::new(crate::game::Game::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let game_clone = game.clone();
                std::thread::spawn(move || {
                    handle_connection(stream, game_clone);
                });
            }
            Err(_) => {
                println!("Couln't handle connection");
            }
        }
    }
}
