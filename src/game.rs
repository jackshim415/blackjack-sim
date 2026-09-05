use crate::strategy::{base_strategy, dealer_strategy, strategy};
use crate::util::{Action, Hand, Result, draw};

pub fn play_round(deck: &mut Vec<u8>) -> Result {
    // Deal player
    let mut player = Hand::new(draw(deck), draw(deck));

    // Dealer's first card is visible
    let dealer_upcard = draw(deck);

    // Dealer hole card
    let mut dealer = Hand::new(dealer_upcard, draw(deck));

    // Handle player blackjack
    if player.is_blackjack() {
        if dealer.is_blackjack() {
            return Result::Push;
        }

        return Result::Win;
    }

    // Player plays
    loop {
        player.count = crate::util::hand_value(&player.cards);

        if player.is_bust() {
            return Result::Loss;
        }

        let action = strategy(&player, dealer_upcard);

        match action {
            Action::Hit => {
                player.add_card(draw(deck));
            }

            Action::Stand => {
                break;
            }
        }
    }

    // Dealer blackjack
    if dealer.is_blackjack() {
        return Result::Loss;
    }

    // Dealer plays
    loop {
        dealer.count = crate::util::hand_value(&dealer.cards);

        match dealer_strategy(&dealer) {
            Action::Hit => {
                dealer.add_card(draw(deck));
            }

            Action::Stand => {
                break;
            }
        }
    }

    // Dealer bust
    if dealer.is_bust() {
        return Result::Win;
    }

    // Compare hands
    if player.count > dealer.count {
        Result::Win
    } else if player.count == dealer.count {
        Result::Push
    } else {
        Result::Loss
    }
}
