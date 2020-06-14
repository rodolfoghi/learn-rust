use std::fmt::{ Display };

fn main() {
    let mut deck = Deck {
        cards: Vec::new(),
    };
    deck.populate();

    for c in deck.cards {
        println!("{}", c);
    }
}

#[derive(Debug, Copy, Clone)]
enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let suit = match self {
            Suit::Hearts => "hearts",
            Suit::Clubs => "clubs",
            Suit::Diamonds => "diamonds",
            Suit::Spades => "spades",
        };
        write!(f, "{}", suit)
    }
}

struct Card {
    suit: Suit,
    number: String,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.number, self.suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn populate(&mut self) {
        let suits = vec![Suit::Hearts, Suit::Clubs, Suit::Diamonds, Suit::Spades];
        let numbers = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

        for s in suits {
            for n in &numbers {
                let card = Card {
                    suit: s,
                    number: n.to_string(),
                };

                self.cards.push(card);
            }
        }
    }
}
