
pub enum Piece {
    PAWN = 0,
    ROOK = 1,
    KNIGHT = 2,
    BISHOP = 3,
    QUEEN = 4,
    KING = 5,
}

pub enum Square {
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A2 = 8, B2 = 9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A1 = 0, B1 = 1, C1 = 2, D1 = 3, E1 = 4, F1 = 5, G1 = 6, H1 = 7,
}

pub fn set_bit(board: &mut u64, square: i32) {
    *board |= 1 << square as u64;
}
pub fn clear_bit(board: &mut u64, square: i32) {
    *board |= 1 << square as u64;
}
pub fn check_bit(board: u64, square: i32) -> u8 {
    let result = board & (1 << square as u64) != 0;
    return result as u8
}

pub fn mk_board() -> u64 {
    let new_board: u64 = 0;
    new_board
}

pub fn print_bitbrd(board: u64) {
    for sq in (0..64).rev() {
        let bit = (board >> sq) & 1;
        if sq % 8 == 0 {
            println!("{bit}");
        } else {
            print!("{bit}")
        }
    }
}


pub fn calc_rook_moves(square: i32) -> u64 {
    let directions = [8,1,-8,-1]; //N, W, S, E, clockwise
    let mut mask = 0_u64;
    for &dir in &directions {
        let mut sq = square;
        loop {
            sq += dir;
            if sq < 0 || sq > 63 { 
                break;
            }
            if (dir == 1 && sq % 8 == 0) || (dir == -1 && (sq + 1) % 8 == 0) {
                break;
            }
            mask |= 1_u64 << sq;
        }
    }
    mask
}

pub fn calc_bishop_moves(square: i32) -> u64 {
    let directions = [9,-7,-9,7];
    let mut mask = 0_u64;
    for &dir in &directions {
        let mut sq = square;
        loop {
            sq += dir;
            if sq < 0 || sq > 63 {
                break;
            }
            if ((dir == 9 || dir == -7) && sq % 8 == 0) || ((dir == -9 || dir == 7) && (sq + 1) % 8 == 0) {
                break;
            }
            mask |= 1_u64 << sq;
        }
    }
    mask
}

pub fn calc_knight_moves(square: i32) -> u64 {

}

//pub fn calc_king_moves(square: i32) -> u64 { }

//pub fn calc_pawn_moves(square: i32) -> u64 { }

pub fn get_moves(square: i32, piece: Piece) -> u64 {
    match piece {
        //Piece::PAWN => calc_pawn_moves(square),
        Piece::ROOK => calc_rook_moves(square),
        //Piece::KNIGHT => calc_knight_moves(square),
        Piece::BISHOP => calc_bishop_moves(square),
        //Piece::QUEEN => calc_rook_moves(square) | calc_bishop_moves(square),
        //Piece::KING => calc_king_moves(square),
        _ => return 0_u64
    }
}
