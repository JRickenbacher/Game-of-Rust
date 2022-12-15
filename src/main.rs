const SIZE_X: usize = 100;
const SIZE_Y: usize = 23;
const DIRECTIONS: [[isize; 2]; 8] = [[1,0], [-1,0], [0,1], [0,-1], [1,1], [1,-1], [-1,1], [-1,-1],];
const INITIALSTATES:[[usize; 2]; 20] = [[9, 50], [10, 49], [10, 51], [11, 49], [11, 51], [12, 50], [13, 49], [13, 51],
                                        [9,45], [9,46], [10,45], [9,55], [9,54], [10,55],
                                        [15,45], [16,45], [16,46], [15,55], [16,55], [16,54],];

type BoardT = [[bool;SIZE_X]; SIZE_Y];

fn main() {
    // unsafe{
    //     let t: libc::termios;
    //     // libc::tcgetattr(fd, optional_actions, termios)
    //     libc::tcsetattr(libc::STDOUT_FILENO, libc::TCSANOW, &t);
    // }
    
    let mut board: BoardT = [[false; SIZE_X as usize]; SIZE_Y as usize];
    for init_state in INITIALSTATES.iter(){
        board[init_state[0]+3][init_state[1]] = true;
    }
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

fn update_board(board: BoardT) -> BoardT {
    let mut new_board: BoardT = [[false; SIZE_X as usize]; SIZE_Y as usize];
    for (row_idx, row) in board.iter().enumerate(){
        for (col_idx, cell) in row.iter().enumerate() {
            let count = count_living_neighbors(board, row_idx, col_idx);
            new_board[row_idx][col_idx] = (*cell && (count == 2 || count == 3)) || (!*cell && count == 3)
        }
    }
    return new_board;
}

fn count_living_neighbors(board: BoardT, curr_row: usize, curr_col: usize) -> u8 {
    let mut count: u8 = 0;
    for direction in DIRECTIONS.iter() {
        count += match (curr_row + 1, direction[0], curr_col + 1, direction[1]) {
            (1, -1, _, _) | (SIZE_Y, 1, _, _) | (_, _, 1, -1) | (_, _, SIZE_X, 1) => 0,
            _ => if board[(curr_row as isize + direction[0]) as usize][(curr_col as isize + direction[1]) as usize] {1} else {0},
        };
    }
    return count
}

fn print_board(board: &BoardT) {
    for row in board.iter(){
        print!("\x1B[0;30m|");
        for cell in row.iter() {
            if *cell {
                print!("\x1B[0;33mX\x1B[0;30m");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    print!("\x1B[{}A", SIZE_Y + 1);
}