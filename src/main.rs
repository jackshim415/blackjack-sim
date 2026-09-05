mod game;
mod util;

use game::play_game;
use rand;
use rayon::prelude::*;

fn main() {
    let total_simulations: u64 = 10_000_000;

    (0..total_simulations).into_par_iter().for_each(|_| {
        play_game();
    });
}
