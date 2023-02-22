mod board;

pub trait Card {
    fn get_x(&self) -> u32;
}

pub trait Player {
    fn get_x(&self) -> u32;
}

pub trait Piece {
    fn get_x(&self) -> u32;
}

pub trait Counter {

}

#[derive(Debug)]
pub struct BoardGame<B, PL, CD, P, CT> where B: board::Board, PL: Player, CD: Card, P: Piece, CT: Counter {
    board: B,
    players: Vec<PL>,
    cards: Vec<CD>,
    pieces: Vec<P>,
    counters: Vec<CT>,
}