const SIZE_X: usize = 40;
const SIZE_Y: usize = 20;
const DIRECTIONS: [[isize; 2]; 8] = [[1,0], [-1,0], [0,1], [0,-1], [1,1], [1,-1], [-1,1], [-1,-1],];

type BOARD_T = [[bool;SIZE_X]; SIZE_Y];

fn main() {
    let mut board: BOARD_T = [[false; SIZE_X as usize]; SIZE_Y as usize];
    board[14][14] = true;
    board[15][15] = true;
    board[16][16] = true;
    board[14][15] = true;
    board[15][16] = true;
    board[16][17] = true;
    board[11][15] = true;
    board[13][16] = true;
    board[14][17] = true;
    board[13][15] = true;
    board[18][16] = true;
    board[17][17] = true;
    loop {
        print_board(&board);
        board = update_board(board);
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line){
            Ok(0) => {println!(); break;}
            Err(_err) => {println!(); break;}
            _ => {}
        }
    }
}

fn update_board(board: BOARD_T) -> BOARD_T {
    let mut new_board: BOARD_T = [[false; SIZE_X as usize]; SIZE_Y as usize];
    for (row_idx, row) in board.iter().enumerate(){
        for (col_idx, cell) in row.iter().enumerate() {
            let count = count_living_neighbors(board, row_idx, col_idx);
            new_board[row_idx][col_idx] = (*cell && (count == 2 || count == 3)) || (!*cell && count == 3)
        }
    }
    return new_board;
}

fn count_living_neighbors(board: BOARD_T, curr_row: usize, curr_col: usize) -> u8 {
    let mut count: u8 = 0;
    for direction in DIRECTIONS.iter() {
        
        count += match (curr_row + 1, direction[0], curr_col + 1, direction[1]) {
            (1, -1, _, _) | (SIZE_Y, 1, _, _) | (_, _, 1, -1) | (_, _, SIZE_X, 1) => 0,
            _ => if board[(curr_row as isize + direction[0]) as usize][(curr_col as isize + direction[1]) as usize] {1} else {0},
        };

        // if (curr_row == 0 && direction[0] == -1) {
        //     continue;
        // }
        // if (curr_row == 0 && next_row == -1) {
        //     continue;
        // }
        // let (next_row, overflow_row) = curr_row.overflowing_add_signed(direction[0]);
        // let (next_col, overflow_col) = curr_col.overflowing_add_signed(direction[1]);
        // if (next_row != -1 && next_row != SIZE_Y as i8) && (next_col != -1 && next_col != SIZE_X as i8) && board[next_row.abs()][next_col] {
            
        // }
    }

    return count
}

fn print_board(board: &BOARD_T) {
    println!("|=========================================================================|");
    for row in board.iter(){
        print!("|");
        for cell in row.iter() {
            if *cell {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    println!("|=========================================================================|");
}