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

use crate::{
    cards::{self, CardColor, CardValue},
    player::{self, Player},
};

enum Direction {
    Right,
    Left,
}

impl Direction {
    pub fn flip(&mut self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Game {
    pub players: Vec<player::Player>,
    deck: cards::Deck,
    direction: Direction,
    pub current_turn: u8,
    pub last_card: cards::Card,
    pub plus: u8,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = cards::Deck::new();

        let mut starting_card = deck.take_card().unwrap();

        if starting_card.color == cards::CardColor::None {
            starting_card.color = cards::CardColor::Red;
        }

        Game {
            players: Vec::<Player>::new(),
            deck,
            direction: Direction::Right,
            current_turn: 0,
            last_card: starting_card,
            plus: 0,
        }
    }

    pub fn can_use(&self, card: cards::Card) -> bool {
        if self.plus != 0 {
            return (card.value == CardValue::WildPlusFour || card.value == CardValue::PlusTwo)
                && (self.last_card.value == card.value || self.last_card.color == card.color);
        }

        if (card.value == cards::CardValue::Wild || card.value == cards::CardValue::WildPlusFour)
            && card.color != CardColor::None
        {
            return true;
        }

        if self.last_card.value == card.value || self.last_card.color == card.color {
            return true;
        }

        false
    }

    pub fn add_player(&mut self, player: player::Player) {
        self.players.push(player);

        let last_player = self.players.len() - 1;
        let player = &mut self.players[last_player];

        for _ in 0..7 {
            self.deck.give_card(player);
        }
    }

    pub fn next_player(&self) -> u8 {
        let mut result = self.current_turn as i8;

        match self.direction {
            Direction::Left => result += 1,
            Direction::Right => result -= 1,
        }

        if result < 0 {
            result = (self.players.len() - 1) as i8;
        } else if result >= self.players.len() as i8 {
            result = 0i8;
        }

        result as u8
    }

    pub fn cycle_turn(&mut self) {
        self.current_turn = self.next_player();
    }

    pub fn take_card(&mut self, id: u8) {
        self.deck.give_card(&mut self.players[id as usize]);
    }

    pub fn play_card(&mut self, card: cards::Card) -> bool {
        if !self.can_use(card) {
            return false;
        }

        self.last_card = card;

        match card.value {
            cards::CardValue::Num(_) => {}
            cards::CardValue::PlusTwo => {
                self.plus += 2;
            }
            cards::CardValue::WildPlusFour => {
                self.plus += 4;
            }
            cards::CardValue::Reverse => {
                self.direction.flip();
                if self.players.len() == 2 {
                    self.cycle_turn(); // To make the reverse card work as skip when there are only 2 players
                }
            }
            cards::CardValue::Skip => {
                self.cycle_turn();
            }
            cards::CardValue::Wild => {}
        };
        self.deck.return_card(card);
        self.cycle_turn();
        true
    }
}
