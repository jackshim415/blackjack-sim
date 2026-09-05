enum Result {
    Win,
    Push,
    Loss,
    Unknown,
}

enum Action {
    Hit,
    Stand,
}

struct Hand {
    cards: Vec<u8>,
    count: u16,
    result: Result,
}
