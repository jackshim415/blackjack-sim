mod game;
mod strategy;
mod util;

use game::play_round;
use rayon::prelude::*;
use util::{get_shuffled_deck, Result};
use std::time::Instant;

#[derive(Default)]
struct Stats {
    wins: u64,
    losses: u64,
    pushes: u64,
}

impl Stats {
    fn add(&mut self, result: Result) {
        match result {
            Result::Win => self.wins += 1,
            Result::Loss => self.losses += 1,
            Result::Push => self.pushes += 1,
            Result::Unknown => unreachable!(),
        }
    }

    fn combine(&mut self, other: Stats) {
        self.wins += other.wins;
        self.losses += other.losses;
        self.pushes += other.pushes;
    }
}

fn main() {
    const SIMULATIONS: usize = 100_000_000;
    const HANDS_PER_BATCH: usize = 100_000;
    const DECKS: u8 = 6;

    let batches = SIMULATIONS / HANDS_PER_BATCH;

    let start = Instant::now();

    let stats = (0..batches)
        .into_par_iter()
        .map(|_| {
            let mut deck = get_shuffled_deck(DECKS);
            let mut stats = Stats::default();

            for _ in 0..HANDS_PER_BATCH {
                // Reshuffle after 75% penetration.
                if deck.len() < (52 * DECKS as usize) / 4 {
                    deck = get_shuffled_deck(DECKS);
                }

                let result = play_round(&mut deck);
                stats.add(result);
            }

            stats
        })
        .reduce(
            Stats::default,
            |mut a, b| {
                a.combine(b);
                a
            },
        );

    let elapsed = start.elapsed();

    let total = stats.wins + stats.losses + stats.pushes;

    let win_rate = stats.wins as f64 / total as f64;
    let loss_rate = stats.losses as f64 / total as f64;
    let push_rate = stats.pushes as f64 / total as f64;

    // Win = +1 unit
    // Loss = -1 unit
    // Push = 0 units
    let ev = (stats.wins as f64 - stats.losses as f64) / total as f64;

    println!("Blackjack Simulation");
    println!("====================");
    println!("Hands:      {total}");
    println!("Wins:       {}", stats.wins);
    println!("Losses:     {}", stats.losses);
    println!("Pushes:     {}", stats.pushes);
    println!();
    println!("Win rate:   {:.4}%", win_rate * 100.0);
    println!("Loss rate:  {:.4}%", loss_rate * 100.0);
    println!("Push rate:  {:.4}%", push_rate * 100.0);
    println!("EV/hand:    {:.6}", ev);
    println!("House edge: {:.4}%", -ev * 100.0);
    println!();
    println!("Time:       {:.3?}", elapsed);
    println!(
    "Hands/sec:  {:.2}M",
    total as f64 / elapsed.as_secs_f64() / 1_000_000.0
);
}