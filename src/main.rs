mod game;
mod strategy;
mod util;

use game::play_round;
use util::{get_shuffled_deck, Result};

fn main() {
    let simulations: usize = 10_000_000;

    let mut deck = get_shuffled_deck(6);

    let mut wins: u64 = 0;
    let mut losses: u64 = 0;
    let mut pushes: u64 = 0;

    for _ in 0..simulations {
        if deck.len() < 52 {
            deck = get_shuffled_deck(6);
        }

        match play_round(&mut deck) {
            Result::Win => wins += 1,
            Result::Loss => losses += 1,
            Result::Push => pushes += 1,
            Result::Unknown => unreachable!(),
        }
    }

    let total = wins + losses + pushes;

    println!("Hands:   {}", total);
    println!("Wins:    {}", wins);
    println!("Losses:  {}", losses);
    println!("Pushes:  {}", pushes);

    println!();
    println!("Win rate:   {:.4}%", wins as f64 / total as f64 * 100.0);
    println!("Loss rate:  {:.4}%", losses as f64 / total as f64 * 100.0);
    println!("Push rate:  {:.4}%", pushes as f64 / total as f64 * 100.0);

    // Simple EV where:
    // Win = +1
    // Loss = -1
    // Push = 0
    let ev = (wins as f64 - losses as f64) / total as f64;

    println!("EV:         {:.6}", ev);
}