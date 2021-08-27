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

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table},
    Frame, Terminal,
};

use crossterm::{
    event::{poll, read, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::{io, time::Duration};

use crate::{cards::CardColor, client, player::Player};

use pad::PadStr;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn convert_color(color: CardColor) -> Color {
    match color {
        crate::cards::CardColor::Blue => Color::Blue,
        crate::cards::CardColor::Green => Color::Green,
        crate::cards::CardColor::Yellow => Color::Yellow,
        crate::cards::CardColor::Red => Color::Red,
        crate::cards::CardColor::None => Color::Reset,
    }
}

pub struct GameUI {
    ticks: u64,
    pub name: String,
    selected_card: usize,
    client: client::Client,
    winner: Option<Player>,
}

impl GameUI {
    pub fn new(ip: String, ticks: u64) -> Self {
        GameUI {
            ticks,
            name: String::new(),
            selected_card: 0,
            client: crate::client::Client::new(ip),
            winner: None,
        }
    }

    pub fn join_screen(&mut self) -> bool {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        enable_raw_mode().unwrap();

        let ascii_art = "██╗   ██╗███╗   ██╗ ██████╗ \n██║   ██║████╗  ██║██╔═══██╗\n██║   ██║██╔██╗ ██║██║   ██║\n██║   ██║██║╚██╗██║██║   ██║\n╚██████╔╝██║ ╚████║╚██████╔╝\n ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ";

        let mut run = true;
        let ticks = self.ticks;

        terminal.clear().unwrap();

        while run {
            terminal
                .draw(|f| {
                    let size = f.size();

                    f.render_widget(Clear, size);

                    let middle_rect = centered_rect(40, 50, size);

                    let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Length(7),
                                Constraint::Length(1),
                                Constraint::Length(3),
                                Constraint::Length(3),
                                Constraint::Percentage(100),
                            ]
                            .as_ref(),
                        )
                        .split(middle_rect);

                    let image = Paragraph::new(ascii_art)
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::all()));

                    let input = Paragraph::new(&self.name[..])
                        .alignment(Alignment::Left)
                        .block(Block::default().borders(Borders::all()));

                    let text = Paragraph::new("Enter name").alignment(Alignment::Left);

                    let instruction = Paragraph::new("Press Enter to join")
                        .alignment(Alignment::Left)
                        .block(Block::default().borders(Borders::ALL));

                    f.render_widget(Block::default().borders(Borders::all()), middle_rect);
                    f.render_widget(image, layout[0]);
                    f.render_widget(text, layout[1]);
                    f.render_widget(input, layout[2]);
                    f.render_widget(instruction, layout[3]);
                })
                .unwrap();
            if poll(Duration::from_millis(ticks)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    use crossterm::event::KeyCode::*;

                    match event.code {
                        Char(c) => {
                            if self.name.len() <= 20 {
                                self.name.push(c);
                            }
                        }
                        Esc => {
                            disable_raw_mode().unwrap();
                            terminal.clear().unwrap();
                            return true;
                        }
                        Backspace => {
                            self.name.pop();
                        }
                        Enter => {
                            run = false;
                        }
                        _ => (),
                    }
                }
            }
        }

        disable_raw_mode().unwrap();
        terminal.clear().unwrap();
        false
    }

    fn draw_top_card(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        f.render_widget(Clear, size);

        let last_card = self.client.top_card().unwrap();

        let style = Style::default().bg(convert_color(last_card.color));

        let top_card = Block::default().style(style);

        let area = centered_rect(5, 13, size);

        f.render_widget(top_card, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(area);

        let text1 = Paragraph::new(last_card.value.to_string()).alignment(Alignment::Left);

        let text2 = Paragraph::new(last_card.value.to_string()).alignment(Alignment::Right);
        f.render_widget(text1, layout[0]);
        f.render_widget(text2, layout[2]);
    }

    fn draw_player_cards(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, location: Rect) {
        let cards = self.client.get_cards_for(&self.name).unwrap();
        let mut spans = vec![];

        for (index, card) in cards.iter().enumerate() {
            let mut card_style = Style::default().bg(convert_color(card.color));
            let pad_width = 4;

            if index == self.selected_card {
                card_style = card_style.add_modifier(Modifier::UNDERLINED);
            }

            let card_text =
                Span::styled(card.value.to_string().pad_to_width(pad_width), card_style);

            spans.push(card_text);
            spans.push(Span::raw(" "))
        }

        let p = Paragraph::new(Spans::from(spans))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(p, location);
    }

    fn draw_status_bar(
        &self,
        f: &mut Frame<CrosstermBackend<io::Stdout>>,
        location: Rect,
        name: &str,
    ) {
        let status = format!("Current turn: {}", name);

        let p = Paragraph::new(status);

        f.render_widget(p, location);
    }

    fn draw_controls(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, location: Rect) {
        let text = vec![
            Spans::from("Right/Left - Card Choosing"),
            Spans::from("Up/Down - Cycle colors for card"),
            Spans::from("Enter - Use card"),
            Spans::from("Z - Take card"),
            Spans::from("Esc - Quit"),
        ];

        let p = Paragraph::new(text).alignment(Alignment::Center);

        f.render_widget(p, location);
    }

    fn draw_player_table(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let (name_space, number_space) = (15, 8);
        let mut location = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(f.size())[0];

        location = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(name_space + number_space),
                    Constraint::Percentage(70),
                ]
                .as_ref(),
            )
            .split(location)[0];

        let players = self.client.get_players().unwrap();

        let mut rows = vec![];

        for player in players.iter() {
            rows.push(Row::new(vec![
                player.name.clone(),
                player.card_num().to_string(),
            ]));
        }

        let widths = [
            Constraint::Length(name_space),
            Constraint::Length(number_space),
        ];
        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL))
            .widths(&widths)
            .header(
                Row::new(vec!["Name", "Cards"])
                    .style(Style::default().add_modifier(Modifier::UNDERLINED)),
            );
        f.render_widget(table, location);
    }

    fn draw_winner(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, winner: &Player) {
        let area = centered_rect(50, 40, f.size());
        let area = Layout::default()
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(1),
                Constraint::Percentage(100),
            ])
            .split(area)[1];
        let p = Paragraph::new(format!("{} won!", winner.name))
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::UNDERLINED));
        f.render_widget(p, area);
    }

    fn get_winner(&self) -> Option<Player> {
        let players = self.client.get_players().unwrap();

        let winner = players.into_iter().filter(|val| val.won()).next();

        winner
    }

    pub fn game_screen(&mut self) {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.clear().unwrap();

        self.client.join(&self.name).unwrap();

        enable_raw_mode().unwrap();

        let ticks = self.ticks;

        let mut run = true;

        while run {
            if let Some(w) = self.get_winner() {
                terminal
                    .draw(|f| {
                        self.draw_winner(f, &w);
                    })
                    .unwrap();
            } else {
                let current_turn = self.client.current_turn().unwrap();
                terminal
                    .draw(|f| {
                        //f.render_widget(Clear, f.size());

                        let layout = Layout::default()
                            .constraints(
                                [
                                    Constraint::Percentage(56),
                                    Constraint::Min(7),
                                    Constraint::Percentage(30),
                                    Constraint::Min(1),
                                ]
                                .as_ref(),
                            )
                            .split(f.size());

                        self.draw_top_card(f);
                        self.draw_controls(f, layout[1]);
                        self.draw_player_cards(f, layout[2]);
                        self.draw_status_bar(f, layout[3], &current_turn);
                        self.draw_player_table(f);
                    })
                    .unwrap();
            }

            let winner = self.get_winner();

            if winner.is_some() {
                self.winner = winner;
            }

            if poll(Duration::from_millis(ticks)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    use crossterm::event::KeyCode::*;

                    if self.winner.is_none() {
                        match event.code {
                            Char(c) => {
                                if c == 'z' {
                                    let plus = self.client.get_plus().unwrap();
                                    if plus == 0 {
                                        self.client.take_card(&self.name, 1).unwrap();
                                        continue;
                                    } else {
                                        self.client.take_card(&self.name, plus).unwrap();
                                        self.client.reset_plus().unwrap();
                                    }
                                }
                            }
                            Esc => run = false,
                            Left => {
                                if self.selected_card > 0 {
                                    self.selected_card -= 1;
                                }
                            }
                            Right => {
                                if self.selected_card
                                    < self.client.get_cards_for(&self.name).unwrap().len() - 1
                                {
                                    self.selected_card += 1;
                                }
                            }
                            Up => {
                                self.client.cycle_color_up(&self.name, self.selected_card);
                            }
                            Down => {
                                self.client.cycle_color_down(&self.name, self.selected_card);
                            }
                            Enter => {
                                let _ = self.client.use_card(&self.name, self.selected_card);
                                if self.client.get_cards_for(&self.name).unwrap().len() != 0
                                    && self.selected_card + 1
                                        > self.client.get_cards_for(&self.name).unwrap().len()
                                {
                                    self.selected_card -= 1;
                                }
                            }
                            _ => (),
                        }
                    } else {
                        match event.code {
                            Esc => run = false,
                            _ => (),
                        };
                    }
                }
            }
        }
        terminal.clear().unwrap();
        disable_raw_mode().unwrap();
    }
}
