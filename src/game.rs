use std::cmp::Ordering;

use rand::{prelude::SliceRandom, thread_rng};

use crate::card::{Card, RANKS, SUITS};

pub enum Input {
    None,
    Hit,
    Stay,
    New,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    NewDeck,
    MyTurn,
    Result,
}

#[derive(Clone, Copy)]
pub enum Winner {
    None,
    Me,
    Dealer,
}

impl Default for State {
    fn default() -> Self {
        State::NewDeck
    }
}

impl Default for Winner {
    fn default() -> Self {
        Winner::None
    }
}

#[derive(Default)]
pub struct Game {
    state: State,
    deck: Vec<Card>,
    my_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
    winner: Winner,
    profit: i32,
    bet: i32,
}

impl Game {
    pub fn update(&mut self, input: Input) {
        match self.state {
            State::NewDeck => {
                self.set_bet();
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
                    if self.deck.len() < 26 {
                        self.state = State::NewDeck;
                    } else {
                        self.reset_hands();
                    }
                }
                _ => (),
            },
        }
    }

    pub fn profit(&self) -> i32 {
        self.profit
    }

    fn set_bet(&mut self) {
        self.bet = 100;
    }

    fn calculate_result(&mut self) {
        self.state = State::Result;

        self.dealer_hand[0].show();

        while Self::hand_value(&self.dealer_hand) < 17 || self.dealer_hand.len() > 4 {
            self.dealer_hand.push(self.deck.pop().unwrap());
        }

        let mut my_hand_value = Self::hand_value(&self.my_hand);
        let mut dealer_hand_value = Self::hand_value(&self.dealer_hand);

        if my_hand_value > 21 {
            my_hand_value = -1;
        }

        if dealer_hand_value > 21 {
            dealer_hand_value = -1;
        }

        let my_hand_length = self.my_hand.len();
        let dealer_hand_length = self.dealer_hand.len();

        if my_hand_length == 5 && my_hand_value != -1 {
            if dealer_hand_length == 5 && dealer_hand_value != -1 {
                self.winner = Winner::None;
                self.profit += self.bet;
                return;
            } else {
                self.winner = Winner::Me;
                self.profit += (self.bet as f32 * 1.5) as i32;
                return;
            }
        } else if dealer_hand_length == 5 && dealer_hand_value != -1 {
            self.winner = Winner::Dealer;
            return;
        }

        match my_hand_value.cmp(&dealer_hand_value) {
            Ordering::Equal => {
                self.winner = Winner::None;
                self.profit += self.bet;
            }
            Ordering::Greater => {
                self.winner = Winner::Me;
                self.profit += (self.bet as f32 * 1.5) as i32;
            }
            Ordering::Less => self.winner = Winner::Dealer,
        }
    }

    fn reset_hands(&mut self) {
        self.profit -= self.bet;

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

    pub fn bet(&self) -> i32 {
        self.bet
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

    pub fn deck_length(&self) -> usize {
        self.deck.len()
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn winner(&self) -> Winner {
        self.winner
    }

    fn generate_deck(&mut self) {
        self.deck.clear();

        for suit in &SUITS {
            for rank in &RANKS {
                self.deck.push(Card::new(suit.clone(), rank.clone()));
                self.deck.push(Card::new(suit.clone(), rank.clone()));
            }
        }

        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
}
