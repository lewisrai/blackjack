pub const DECK_SIZE: usize = 52;

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

    pub fn as_compact_string(&self) -> String {
        if self.hidden {
            return "?".to_string();
        }

        let suit_text = match self.suit {
            Suit::Club => ("♣", "Clubs"),
            Suit::Diamond => ("♦", "Diamonds"),
            Suit::Heart => ("♥", "Hearts"),
            Suit::Spade => ("♠", "Spades"),
        };

        match self.rank {
            Rank::Ace => format!("Ace of {}", suit_text.0),
            Rank::Two => format!("Two of {}", suit_text.0),
            Rank::Three => format!("Three of {}", suit_text.0),
            Rank::Four => format!("Four of {}", suit_text.0),
            Rank::Five => format!("Five of {}", suit_text.0),
            Rank::Six => format!("Six of {}", suit_text.0),
            Rank::Seven => format!("Seven of {}", suit_text.0),
            Rank::Eight => format!("Eight of {}", suit_text.0),
            Rank::Nine => format!("Nine of {}", suit_text.0),
            Rank::Ten => format!("Ten of {}", suit_text.0),
            Rank::Jack => format!("Jack of {}", suit_text.0),
            Rank::Queen => format!("Queen of {}", suit_text.0),
            Rank::King => format!("King of {}", suit_text.0),
        }
    }

    pub fn as_art_string_lines(&self) -> Vec<String> {
        let mut text = vec!["┌───────┐".to_string()];

        if self.hidden {
            text.push("│       │".to_string());
            text.push("│       │".to_string());
            text.push("│   ?   │".to_string());
            text.push("│       │".to_string());
            text.push("│       │".to_string());
            text.push("└───────┘".to_string());

            return text;
        }

        let suit_icon = match self.suit {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };

        match self.rank {
            Rank::Ace => {
                text.push("│A      │".to_string());
                text.push("│       │".to_string());
                text.push(format!("│   {}   │", suit_icon));
                text.push("│       │".to_string());
                text.push("│      A│".to_string());
            }
            Rank::Two => {
                text.push("│2      │".to_string());
                text.push(format!("│   {}   │", suit_icon));
                text.push("│       │".to_string());
                text.push(format!("│   {}   │", suit_icon));
                text.push("│      2│".to_string());
            }
            Rank::Three => {
                text.push("│3      │".to_string());
                text.push(format!("│   {}   │", suit_icon));
                text.push(format!("│   {}   │", suit_icon));
                text.push(format!("│   {}   │", suit_icon));
                text.push("│      3│".to_string());
            }
            Rank::Four => {
                text.push("│4      │".to_string());
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push("│       │".to_string());
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push("│      4│".to_string());
            }
            Rank::Five => {
                text.push("│5      │".to_string());
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│   {}   │", suit_icon));
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push("│      5│".to_string());
            }
            Rank::Six => {
                text.push("│6      │".to_string());
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push("│      6│".to_string());
            }
            Rank::Seven => {
                text.push(format!("│7 {} {}  │", suit_icon, suit_icon));
                text.push(format!("│   {}   │", suit_icon));
                text.push("│       │".to_string());
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {} 7│", suit_icon, suit_icon));
            }
            Rank::Eight => {
                text.push(format!("│8 {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push("│       │".to_string());
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {} 8│", suit_icon, suit_icon));
            }
            Rank::Nine => {
                text.push(format!("│9 {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│   {}   │", suit_icon));
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {} 9│", suit_icon, suit_icon));
            }
            Rank::Ten => {
                text.push(format!("│1 {} {}  │", suit_icon, suit_icon));
                text.push(format!("│0 {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {}  │", suit_icon, suit_icon));
                text.push(format!("│  {} {} 1│", suit_icon, suit_icon));
                text.push(format!("│  {} {} 0│", suit_icon, suit_icon));
            }
            Rank::Jack => {
                text.push("│J      │".to_string());
                text.push(format!("│ {}     │", suit_icon));
                text.push("│       │".to_string());
                text.push(format!("│     {} │", suit_icon));
                text.push("│      J│".to_string());
            }
            Rank::Queen => {
                text.push("│Q      │".to_string());
                text.push(format!("│ {}     │", suit_icon));
                text.push("│       │".to_string());
                text.push(format!("│     {} │", suit_icon));
                text.push("│      Q│".to_string());
            }
            Rank::King => {
                text.push("│K      │".to_string());
                text.push(format!("│ {}     │", suit_icon));
                text.push("│       │".to_string());
                text.push(format!("│     {} │", suit_icon));
                text.push("│      K│".to_string());
            }
        }

        text.push("└───────┘".to_string());

        text
    }

    pub fn generate_deck(number_of_decks: usize) -> Vec<Card> {
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

        let mut deck = Vec::new();

        for suit in &suits {
            for rank in &ranks {
                for _ in 0..number_of_decks {
                    deck.push(Card::new(suit.clone(), rank.clone()));
                }
            }
        }

        deck
    }
}
