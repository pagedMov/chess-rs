mod boards;

use boards::Piece::*;
use boards::Square;
use boards::Color;


pub enum Outcome {
    Checkmate,
    Stalemate,
    Draw,
}

struct Game {
    players: [Player; 2],
    turn: i32,
    player_to_move: i8,
    check: bool,
}

impl Game {
    pub fn new() -> Self {
        let white = Player::new(Color::White);
        let black = Player::new(Color::Black);
        Game {
            players: [white,black],
            turn: 0,
            player_to_move: 0, 
            check: false,
        }
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
        self.player_to_move += 1; 
        self.player_to_move %= 2; 
    }

    pub fn start_game(&mut self) { }
}

struct Player {
    piece_boards: [u64; 6],
}

impl Player {
    pub fn new(color: Color) -> Self {
        let boards: [u64; 6] = Player::init_boards(color);
        Player {
            piece_boards: boards,
        }
    }

    pub fn setup_piece(bit_positions:u64, is_white: bool) -> u64 {
        if is_white {
            bit_positions
        } else {
            bit_positions.reverse_bits()
        }
    }

    pub fn init_boards(color: Color) -> [u64; 6] {
        let mut boards: [u64; 6] = [0; 6];
        let is_white = match color {
            Color::White => true,
            Color::Black => false,
        };
        boards[ROOK as usize] |= Player::setup_piece(1_u64 | (1_u64 << 7), is_white);
        boards[KNIGHT as usize] |= Player::setup_piece(2_u64 | (2_u64 << 6), is_white);
        boards[BISHOP as usize] |= Player::setup_piece(4_u64 | (4_u64 << 5), is_white);
        boards[QUEEN as usize] |= Player::setup_piece(8_u64, is_white);
        boards[KING as usize] |= Player::setup_piece(16_u64, is_white);
        boards[PAWN as usize] |= Player::setup_piece(0xFF_u64 << 8, is_white);

        boards
    }

}

fn main() {

    let rookboard: u64 = 1_u64 << Square::H8 as usize;
    let mut positions = Vec::new();
    for i in 0..64 {
        if (rookboard >> i) & 1 == 1 {
            positions.push(i)
        }
    }

    let mut all_rook_moves = 0_u64;
    for square in positions {
        let moves = boards::get_moves(square, KNIGHT);
        all_rook_moves |= moves;
    }

    boards::print_bitbrd(all_rook_moves);

}
