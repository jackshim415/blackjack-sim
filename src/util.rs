use rand::{rng, seq::SliceRandom};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Result {
    Win,
    Push,
    Loss,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Hit,
    Stand,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<u8>,
    pub count: u16,
    pub result: Result,
}

impl Hand {
    pub fn new(card1: u8, card2: u8) -> Self {
        let cards = vec![card1, card2];

        Self {
            count: hand_value(&cards),
            cards,
            result: Result::Unknown,
        }
    }

    pub fn add_card(&mut self, card: u8) {
        self.cards.push(card);
        self.count = hand_value(&self.cards);
    }

    pub fn is_bust(&self) -> bool {
        self.count > 21
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.count == 21
    }

    pub fn is_soft(&self) -> bool {
        let raw: u16 = self.cards.iter().map(|&x| x as u16).sum();
        raw != self.count
    }
}

pub fn hand_value(cards: &[u8]) -> u16 {
    let mut total: u16 = cards.iter().map(|&x| x as u16).sum();
    let mut aces = cards.iter().filter(|&&x| x == 11).count();

    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }

    total
}

pub fn get_shuffled_deck(decks: u8) -> Vec<u8> {
    let single_deck: [u8; 52] = [
        2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11,
    ];

    let mut deck = single_deck.repeat(decks as usize);
    deck.shuffle(&mut rng());

    deck
}

pub fn draw(deck: &mut Vec<u8>) -> u8 {
    deck.pop().expect("DECK_EMPTY")
}
