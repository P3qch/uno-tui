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

mod cards;
mod client;
mod game;
mod player;
mod server;
mod ui;

use std::thread;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Uno tui")
        .version("0.1.0")
        .author("P3qch")
        .about("A basic Uno game that is played threw the terminal")
        .arg(
            Arg::with_name("host")
                .short("s")
                .long("host")
                .takes_value(false)
                .help("Choose if you wish to host"),
        )
        .arg(
            Arg::with_name("ip")
                .short("a")
                .long("address")
                .takes_value(true)
                .help("The IP address on which you wish to open the server"),
        )
        .arg(
            Arg::with_name("ticks")
                .short("t")
                .long("ticks")
                .takes_value(true)
                .help("The number milisecons to wait for an event"),
        )
        .get_matches();

    let ip = matches
        .value_of("ip")
        .unwrap_or("127.0.0.1:8080")
        .to_string();
    let ip2 = ip.clone();
    if matches.is_present("host") {
        thread::spawn(|| server::start_server(ip2));
    }

    let mut ui = ui::GameUI::new(
        ip,
        matches.value_of("ticks").unwrap_or("100").parse().unwrap(),
    );
    if ui.join_screen() {
        return;
    }
    ui.game_screen();
}
