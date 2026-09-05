use crate::util::{Action, Hand};

pub fn strategy(hand: &Hand, dealer_upcard: u8) -> Action {
    armoir(hand, dealer_upcard)
    //over(hand,dealer_upcard,16)
}

pub fn base_strategy(hand: &Hand, dealer_upcard: u8) -> Action {
    over(hand, dealer_upcard, 16)
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

fn armoir(hand: &Hand, dealer_upcard: u8) -> Action {
    const FOURFIVESIX: [u8; 3] = [4, 5, 6];
    match hand.count {
        17.. => Action::Stand,
        13..=16 => {
            if dealer_upcard < 7 {
                Action::Stand
            } else {
                Action::Hit
            }
        }
        12 => {
            if FOURFIVESIX.contains(&dealer_upcard) {
                Action::Stand
            } else {
                Action::Hit
            }
        }
        ..=11 => Action::Hit,
    }
}
