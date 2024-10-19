use std::cmp::Ordering;

use rand::{prelude::SliceRandom, rngs::ThreadRng, thread_rng};

use crate::card::Card;

#[derive(PartialEq)]
pub enum Input {
    None,
    Hit,
    Stay,
    New,
    CompactMode,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Winner {
    None,
    Me,
    Dealer,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    NewDeck,
    MyTurn,
    Result(Winner),
}

pub struct Game {
    state: State,
    rng: ThreadRng,
    deck: Vec<Card>,
    my_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
    profit: i32,
    bet: i32,
    compact_mode: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: State::NewDeck,
            rng: thread_rng(),
            deck: Vec::new(),
            my_hand: Vec::new(),
            dealer_hand: Vec::new(),
            profit: 0,
            bet: 100,
            compact_mode: false,
        }
    }

    pub fn update(&mut self, input: Input) {
        if input == Input::CompactMode {
            self.compact_mode = !self.compact_mode;
            return;
        }

        match self.state {
            State::NewDeck => {
                self.new_deck();
            }
            State::MyTurn => match input {
                Input::Hit => self.hit(),
                Input::Stay => self.calculate_result(),
                _ => (),
            },
            State::Result(_) => match input {
                Input::New => self.new_deck(),
                _ => (),
            },
        }
    }

    fn new_deck(&mut self) {
        self.state = State::MyTurn;

        if self.deck.len() < 26 {
            self.deck.clear();
            self.deck.append(&mut Card::generate_deck());
            self.deck.shuffle(&mut self.rng);
        }

        self.profit -= self.bet;

        self.my_hand.clear();
        self.dealer_hand.clear();

        self.my_hand.push(self.deck.pop().unwrap());
        self.dealer_hand.push(self.deck.pop().unwrap());
        self.my_hand.push(self.deck.pop().unwrap());
        self.dealer_hand.push(self.deck.pop().unwrap());

        self.dealer_hand[0].hide();

        if Self::hand_value(&self.my_hand) == 21 {
            self.calculate_result();
        }
    }

    fn hit(&mut self) {
        self.my_hand.push(self.deck.pop().unwrap());

        if Self::hand_value(&self.my_hand) > 20 || self.my_hand.len() == 5 {
            self.calculate_result();
        }
    }

    fn calculate_result(&mut self) {
        self.state = State::Result(Winner::None);

        self.dealer_hand[0].show();

        while Self::hand_value(&self.dealer_hand) < 17 && self.dealer_hand.len() < 5 {
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
                self.profit += self.bet;
            } else {
                self.state = State::Result(Winner::Me);
                self.profit += (self.bet as f32 * 1.5) as i32;
            }
        } else if dealer_hand_length == 5 && dealer_hand_value != -1 {
            self.state = State::Result(Winner::Dealer);
        } else {
            match my_hand_value.cmp(&dealer_hand_value) {
                Ordering::Equal => {
                    self.state = State::Result(Winner::None);
                    self.profit += self.bet;
                }
                Ordering::Greater => {
                    self.state = State::Result(Winner::Me);
                    self.profit += (self.bet as f32 * 1.5) as i32;
                }
                Ordering::Less => self.state = State::Result(Winner::Dealer),
            }
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

        while value > 21 && aces != 0 {
            value -= 10;
            aces -= 1;
        }

        value
    }

    pub fn my_hand(&self) -> String {
        let mut output = String::new();

        for card in &self.my_hand {
            match self.compact_mode {
                true => {
                    output += &card.as_compact_string();
                    output += "\n";
                }

                false => {
                    let card_text = card.as_art_string_lines();

                    for line in card_text {
                        output += &line;
                        output += "\n";
                    }
                }
            }
        }

        output
    }

    pub fn dealer_hand(&self) -> String {
        let mut output = String::new();

        for card in &self.dealer_hand {
            match self.compact_mode {
                true => {
                    output += &card.as_compact_string();
                    output += "\n";
                }

                false => {
                    let card_text = card.as_art_string_lines();

                    for line in card_text {
                        output += &line;
                        output += "\n";
                    }
                }
            }
        }

        output
    }

    pub fn bet(&self) -> i32 {
        self.bet
    }

    pub fn compact_mode(&self) -> bool {
        self.compact_mode
    }

    pub fn deck_length(&self) -> usize {
        self.deck.len()
    }

    pub fn profit(&self) -> i32 {
        self.profit
    }

    pub fn state(&self) -> State {
        self.state
    }
}
