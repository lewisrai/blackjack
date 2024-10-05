use std::fmt::{Debug, Display};

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
        let mut text = "┌───────┐\n".to_string();

        if self.hidden {
            text += "│       │\n│       │\n│   ?   │\n│       │\n│       │\n└───────┘\n";
            return write!(f, "{}", text);
        }

        let suit_icon = match self.suit {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };

        match self.rank {
            Rank::Ace => {
                text += "│A      │\n";
                text += "│       │\n";
                text += &format!("│   {}   │\n", suit_icon);
                text += "│       │\n";
                text += "│      A│\n";
            }
            Rank::Two => {
                text += "│2      │\n";
                text += &format!("│   {}   │\n", suit_icon);
                text += "│       │\n";
                text += &format!("│   {}   │\n", suit_icon);
                text += "│      2│\n";
            }
            Rank::Three => {
                text += "│3      │\n";
                text += &format!("│   {}   │\n", suit_icon);
                text += &format!("│   {}   │\n", suit_icon);
                text += &format!("│   {}   │\n", suit_icon);
                text += "│      3│\n";
            }
            Rank::Four => {
                text += "│4      │\n";
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += "│       │\n";
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += "│      4│\n";
            }
            Rank::Five => {
                text += "│5      │\n";
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│   {}   │\n", suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += "│      5│\n";
            }
            Rank::Six => {
                text += "│6      │\n";
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += "│      6│\n";
            }
            Rank::Seven => {
                text += "│7      │\n";
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {}{}{}  │\n", suit_icon, suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += "│      7│\n";
            }
            Rank::Eight => {
                text += &format!("│8 {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += "│       │\n";
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {} 8│\n", suit_icon, suit_icon);
            }
            Rank::Nine => {
                text += &format!("│9 {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│   {}   │\n", suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {} 9│\n", suit_icon, suit_icon);
            }
            Rank::Ten => {
                text += &format!("│10{} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}  │\n", suit_icon, suit_icon);
                text += &format!("│  {} {}10│\n", suit_icon, suit_icon);
            }
            Rank::Jack => {
                text += "│J      │\n";
                text += &format!("│ {}     │\n", suit_icon);
                text += "│       │\n";
                text += &format!("│     {} │\n", suit_icon);
                text += "│      J│\n";
            }
            Rank::Queen => {
                text += "│Q      │\n";
                text += &format!("│ {}     │\n", suit_icon);
                text += "│       │\n";
                text += &format!("│     {} │\n", suit_icon);
                text += "│      Q│\n";
            }
            Rank::King => {
                text += "│K      │\n";
                text += &format!("│ {}     │\n", suit_icon);
                text += "│       │\n";
                text += &format!("│     {} │\n", suit_icon);
                text += "│      K│\n";
            }
        };

        write!(f, "{}", text + "└───────┘\n")
    }
}
