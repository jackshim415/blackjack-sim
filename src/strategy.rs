use crate::util::{Action, Hand};

pub fn strategy(hand: &Hand, dealer_upcard: u8) -> Action {
    over(hand,dealer_upcard,16)
}

pub fn base_strategy(hand: &Hand, dealer_upcard: u8) -> Action {
    over(hand,dealer_upcard,16)
}

pub fn dealer_strategy(hand: &Hand) -> Action {
    over(hand, 0, 17)
}

fn over(hand: &Hand, dealer_upcard: u8, over: u16) -> Action {
    if hand.count < over {
        Action::Hit
    } else {
        Action::Stand
    }
}