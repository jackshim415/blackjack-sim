use rand::{rng, seq::SliceRandom};
use util::*;

pub fn play_game() {
    let deck: Vec<u8> = get_shuffled_deck(6);

    let players: usize = 6;
}

fn get_shuffled_deck(decks: u8) -> Vec<u8> {
    let single_deck: [u8; 52] = [
        2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2,
        3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11,
    ];

    let mut deck: Vec<u8> = single_deck.repeat(usize::from(decks));

    deck.shuffle(&mut rng());
    deck
}

fn play_round(deck: Vec<u8>, players: usize) {
    let mut hands: Vec<Hand> = vec![];

    // deal cards
    for _ in 1..players {
        hands.push(Hand {
            cards: vec![draw(&mut deck), draw(&mut deck)],
            result: Result.Unknown,
        });
        hands[-1].count = hands[-1].cards[0] + hands[-1].cards[1]
    }

    let mut dealer: hand = Hand {
        cards: vec![draw(&mut deck), draw(&mut deck)],
        result: Result.Unknown,
    };

    dealer.count = dealer.cards[0] + dealer.cards[1];

    for (x, hand) in hands.iter_mut().enumerate() {
        let mut action: Action = Action.Stand;
        if x == 0 {
            action = strategy(hand);
        } else {
            action = baseStrategy(hand);
        }

        if (action == Action.Hit) {
            hand.cards.push(draw(&mut deck));
            hand.count += hand.cards[-1];
            if hand.count > 21 {
                hand.result = Result.Loss;
            }
        }
    }

    if (dealerStrategy(dealer) == Action.Hit) {
        let card = draw(&mut deck);
        dealer.cards.push(card);
        dealer.count += card;
        if dealer.count > 21 {
            dealer.result = Result.Loss;
            for hand in hands {
                if hand.result != Result.Loss {
                    hand.result = Result.Win;
                }
            }
        } else {
            for hand in hands {
                if hand.result != Result.Loss && hand.count > dealer.count {
                    hand.result = Result.Win;
                } else if hand.result != Result.Loss && hand.count == dealer.count {
                    hand.result = Result.Push
                } else {
                    hand.result = Result.Loss
                }
            }
        }
    } else {
        for hand in hands {
            if hand.result != Result.Loss && hand.count > dealer.count {
                hand.result = Result.Win;
            } else if hand.result != Result.Loss && hand.count == dealer.count {
                hand.result = Result.Push
            } else {
                hand.result = Result.Loss
            }
        }
    }
    hands[0].result
}
fn draw(deck: &mut Vec<u8>) -> u8 {
    deck.pop().expect("Deck is empty")
}