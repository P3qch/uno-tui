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

use super::card::*;
use crate::player::Player;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>,
}

impl Iterator for Deck {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        self.take_card()
    }
}

impl ExactSizeIterator for Deck {
    fn len(&self) -> usize {
        self.cards.len()
    }
}

impl Deck {
    pub fn new() -> Self {
        use CardColor::*;
        let mut result = Vec::<Card>::new();
        let colors = [Red, Green, Blue, Yellow];

        for i in 0u8..10u8 {
            for color in colors {
                let card = Card::new(CardValue::Num(i), color);
                result.push(card);
                result.push(card);
            }
        }

        for color in colors {
            result.push(Card::skip(color));
            result.push(Card::skip(color));

            result.push(Card::reverse(color));
            result.push(Card::reverse(color));

            result.push(Card::plus_two(color));
            result.push(Card::plus_two(color));
        }

        for _ in 0..4 {
            result.push(Card::wild());
            result.push(Card::wild_plus_four());
        }

        let mut result = Deck { cards: result };
        result.shuffle();

        result
    }

    pub fn give_card(&mut self, player: &mut Player) {
        player.add_card(self.take_card().unwrap());
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn return_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn take_card(&mut self) -> Option<Card> {
        if !self.cards.is_empty() {
            Some(self.cards.remove(0))
        } else {
            None
        }
    }
}
