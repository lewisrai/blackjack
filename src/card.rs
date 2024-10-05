use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub const SUITS: [Suit; 4] = [Suit::Diamond, Suit::Heart, Suit::Club, Suit::Spade];
pub const RANKS: [Rank; 13] = [
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

#[derive(Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
    hidden: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
            hidden: false,
        }
    }

    pub fn hide(&mut self) {
        self.hidden = true;
    }

    pub fn show(&mut self) {
        self.hidden = false;
    }

    pub fn is_ace(&self) -> bool {
        self.rank == Rank::Ace
    }

    pub fn value(&self) -> i32 {
        match self.rank {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hidden {
            return write!(f, "???");
        }

        let suit_icon = match self.suit {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };

        let rank_text = match self.rank {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };

        write!(f, "{} {}", suit_icon, rank_text)
    }
}
