
pub enum Color {
    White,
    Black,
}

pub enum Piece {
    PAWN = 0,
    ROOK = 1,
    KNIGHT = 2,
    BISHOP = 3,
    QUEEN = 4,
    KING = 5,
}

pub enum Square {
    H8 = 56, G8 = 57, F8 = 58, E8 = 59, D8 = 60, C8 = 61, B8 = 62, A8 = 63,

    H7 = 48, G7 = 49, F7 = 50, E7 = 51, D7 = 52, C7 = 53, B7 = 54, A7 = 55,

    H6 = 40, G6 = 41, F6 = 42, E6 = 43, D6 = 44, C6 = 45, B6 = 46, A6 = 47,

    H5 = 32, G5 = 33, F5 = 34, E5 = 35, D5 = 36, C5 = 37, B5 = 38, A5 = 39,

    H4 = 24, G4 = 25, F4 = 26, E4 = 27, D4 = 28, C4 = 29, B4 = 30, A4 = 31,

    H3 = 16, G3 = 17, F3 = 18, E3 = 19, D3 = 20, C3 = 21, B3 = 22, A3 = 23,

    H2 = 08, G2 = 09, F2 = 10, E2 = 11, D2 = 12, C2 = 13, B2 = 14, A2 = 15,

    H1 = 00, G1 = 01, F1 = 02, E1 = 03, D1 = 04, C1 = 05, B1 = 06, A1 = 07,

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
    let offsets: [i32; 8] = [6,10,15,17,-6,-10,-15,-17];
    let mut mask = 0_u64;
    let current_file = square % 8; // file (column)
    let current_rank = square / 8; // rank (row)
    for &offset in &offsets {
            let target_square = square + offset;

            // Make sure the target square is still on the board
            if target_square >= 0 && target_square <= 63 {
                let target_file = target_square % 8;
                let target_rank = target_square / 8;

                // Ensure the move doesn't wrap around the board
                // Knight moves must stay within two ranks and one or two files
                if (target_rank - current_rank).abs() <= 2 && (target_file - current_file).abs() <= 2 {
                    mask |= 1_u64 << target_square;
                }
            }
    }
    mask
}
    
//   pub fn calc_pawn_moves(square: i32, color: Color) -> u64 {
//       let offsets: [i32; 8] = [];
//       let mut mask = 0_u64;
//       let current_rank = square / 8;
//   }

//pub fn calc_king_moves(square: i32) -> u64 { }

pub fn get_moves(square: i32, piece: Piece) -> u64 {
    match piece {
        //Piece::PAWN => calc_pawn_moves(square),
        Piece::ROOK => calc_rook_moves(square),
        Piece::KNIGHT => calc_knight_moves(square),
        Piece::BISHOP => calc_bishop_moves(square),
        //Piece::QUEEN => calc_rook_moves(square) | calc_bishop_moves(square),
        //Piece::KING => calc_king_moves(square),
        _ => return 0_u64
    }
}
