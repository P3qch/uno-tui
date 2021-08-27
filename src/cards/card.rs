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
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum CardColor {
    Red,
    Green,
    Blue,
    Yellow,
    None,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum CardValue {
    Num(u8),
    Reverse,
    PlusTwo,
    Skip,
    Wild,
    WildPlusFour,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Card {
    pub value: CardValue,
    pub color: CardColor,
}

impl ToString for CardValue {
    fn to_string(&self) -> String {
        match self {
            CardValue::Num(n) => n.to_string(),
            CardValue::PlusTwo => String::from("+2"),
            CardValue::Reverse => String::from("🔃"),
            CardValue::Skip => String::from("⛔"),
            CardValue::Wild => String::from("Wild"),
            CardValue::WildPlusFour => String::from("+4"),
        }
    }
}

impl Card {
    pub fn new(value: CardValue, color: CardColor) -> Self {
        Card { value, color }
    }

    pub fn cycle_colors_up(&mut self) {
        if !(self.value == CardValue::Wild || self.value == CardValue::WildPlusFour) {
            return;
        }
        let colors = [
            CardColor::Red,
            CardColor::Green,
            CardColor::Blue,
            CardColor::Yellow,
        ];

        if self.color == CardColor::None {
            self.color = CardColor::Red;
        } else {
            let current_color = colors.iter().position(|val| val == &self.color).unwrap();
            if current_color == 3 {
                self.color = CardColor::Red;
            } else {
                self.color = colors[current_color + 1];
            }
        }
    }

    pub fn cycle_colors_down(&mut self) {
        if !(self.value == CardValue::Wild || self.value == CardValue::WildPlusFour) {
            return;
        }
        let colors = [
            CardColor::Red,
            CardColor::Green,
            CardColor::Blue,
            CardColor::Yellow,
        ];

        if self.color == CardColor::None {
            self.color = CardColor::Red;
        } else {
            let current_color = colors.iter().position(|val| val == &self.color).unwrap();
            if current_color == 0 {
                self.color = CardColor::Yellow;
            } else {
                self.color = colors[current_color - 1];
            }
        }
    }

    pub fn skip(color: CardColor) -> Self {
        Card {
            value: CardValue::Skip,
            color,
        }
    }

    pub fn reverse(color: CardColor) -> Self {
        Card {
            value: CardValue::Reverse,
            color,
        }
    }

    pub fn plus_two(color: CardColor) -> Self {
        Card {
            value: CardValue::PlusTwo,
            color,
        }
    }

    pub fn wild() -> Self {
        Card {
            value: CardValue::Wild,
            color: CardColor::None,
        }
    }

    pub fn wild_plus_four() -> Self {
        Card {
            value: CardValue::WildPlusFour,
            color: CardColor::None,
        }
    }
}
