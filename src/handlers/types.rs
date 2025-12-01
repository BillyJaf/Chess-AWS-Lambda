use::serde::Serialize;

#[derive(Serialize)]
pub enum GameOver {
    White,
    Black,
    Stalemate,
}

#[derive(Serialize)]
pub struct ResultingGameState {
    pub uci_move: String,
    pub resulting_fen: String,
    pub game_over: Option<GameOver>,
}