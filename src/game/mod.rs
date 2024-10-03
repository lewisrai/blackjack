use std::io::stdin;

use rand::prelude::*;

mod deck;
use deck::{Card, Rank, Suit};

#[derive(Debug, PartialEq)]
enum State {
    MyTurn,
    Bust,
    Check,
    Reset,
}

pub struct Game {
    rng: ThreadRng,
    deck: Vec<Card>,
    should_quit: bool,
    my_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
    state: State,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut deck = Self::generate_deck();

        deck.shuffle(&mut rng);

        Self {
            rng,
            deck,
            should_quit: false,
            my_hand: Vec::new(),
            dealer_hand: Vec::new(),
            state: State::Reset,
        }
    }

    pub fn run(&mut self) {
        while !self.should_quit {
            match self.state {
                State::MyTurn => {
                    let mut total = 0;
                    for hand in &self.my_hand {
                        total += hand.value();
                    }

                    if total > 21 {
                        self.state = State::Bust;
                    }
                }
                State::Bust => (),
                State::Check => {
                    let mut total = 0;
                    for hand in &self.dealer_hand {
                        total += hand.value();
                    }

                    while total <= 16 {
                        let card = self.deck.pop().unwrap();
                        total += card.value();
                        self.dealer_hand.push(card);
                    }
                }
                State::Reset => {
                    self.state = State::MyTurn;
                    self.my_hand.clear();
                    self.dealer_hand.clear();

                    self.my_hand.push(self.deck.pop().unwrap());
                    self.dealer_hand.push(self.deck.pop().unwrap());
                    self.my_hand.push(self.deck.pop().unwrap());
                    self.dealer_hand.push(self.deck.pop().unwrap());
                }
            }

            self.print_state();

            let mut user_input = String::new();

            stdin()
                .read_line(&mut user_input)
                .expect("Error > Failed to read line");

            match user_input.trim().to_lowercase().as_str() {
                "quit" => self.should_quit = true,
                "stay" => self.state = State::Check,
                "hit" => self.my_hand.push(self.deck.pop().unwrap()),
                _ => {
                    if self.state == State::Check || self.state == State::Bust {
                        self.state = State::Reset;
                    }
                }
            }
        }
    }

    fn print_state(&self) {
        println!("");
        println!("{:?}", self.state);
        println!("My Hand: {:?}", self.my_hand);

        if self.state == State::MyTurn {
            println!("Dealer Hand: ?, {:?}", self.dealer_hand[1]);
        } else {
            println!("Dealer Hand: {:?}", self.dealer_hand);
        }
    }

    fn generate_deck() -> Vec<Card> {
        let mut deck = Vec::new();

        let suits = [Suit::Diamond, Suit::Heart, Suit::Club, Suit::Spade];
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];

        for suit in &suits {
            for rank in &ranks {
                deck.push(Card {
                    suit: suit.clone(),
                    rank: rank.clone(),
                });
            }
        }

        deck
    }
}
