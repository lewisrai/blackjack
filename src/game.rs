use rand::{prelude::SliceRandom, thread_rng};

use crate::card::{Card, RANKS, SUITS};

pub enum Input {
    None,
    Hit,
    Stay,
    New,
}

enum State {
    Start,
    MyTurn,
    Result,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Game {
    state: State,
    deck: Vec<Card>,
    my_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
}

impl Game {
    pub fn update(&mut self, input: Input) {
        match self.state {
            State::Start => {
                self.generate_deck();
                self.reset_hands();
            }
            State::MyTurn => match input {
                Input::Hit => {
                    self.my_hand.push(self.deck.pop().unwrap());

                    if Self::hand_value(&self.my_hand) > 20 || self.my_hand.len() == 5 {
                        self.calculate_result();
                    }
                }
                Input::Stay => self.calculate_result(),
                _ => (),
            },
            State::Result => match input {
                Input::New => {
                    self.reset_hands();
                }
                _ => (),
            },
        }
    }

    fn calculate_result(&mut self) {
        self.state = State::Result;

        self.dealer_hand[0].show();

        while Self::hand_value(&self.dealer_hand) < 17 {
            self.dealer_hand.push(self.deck.pop().unwrap());
        }
    }

    fn reset_hands(&mut self) {
        self.my_hand.clear();
        self.dealer_hand.clear();

        self.my_hand.push(self.deck.pop().unwrap());
        self.dealer_hand.push(self.deck.pop().unwrap());
        self.dealer_hand[0].hide();
        self.my_hand.push(self.deck.pop().unwrap());
        self.dealer_hand.push(self.deck.pop().unwrap());

        self.state = State::MyTurn;

        if Self::hand_value(&self.my_hand) == 21 {
            self.calculate_result();
        }
    }

    fn hand_value(hand: &Vec<Card>) -> i32 {
        let mut value = 0;
        let mut aces = 0;

        for card in hand {
            value += card.value();

            if card.is_ace() {
                aces += 1;
            };
        }

        while value > 21 {
            if aces != 0 {
                value -= 10;
                aces -= 1;
            } else {
                break;
            }
        }

        value
    }

    pub fn my_hand(&self) -> String {
        let mut output = String::new();

        for card in &self.my_hand {
            output += &card.to_string();
            output += "\n";
        }

        output
    }

    pub fn dealer_hand(&self) -> String {
        let mut output = String::new();

        for card in &self.dealer_hand {
            output += &card.to_string();
            output += "\n";
        }

        output
    }

    pub fn deck_length(&self) -> String {
        self.deck.len().to_string()
    }

    fn generate_deck(&mut self) {
        for suit in &SUITS {
            for rank in &RANKS {
                self.deck.push(Card::new(suit.clone(), rank.clone()));
            }
        }

        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
}
