// size of the game grid
static GRID_SIZE:usize = 9;

// checks to tell if the number placement is valid
fn is_number_in_row(board:&[[i32; 9]; 9], number:i32, row:usize) -> bool {
    for i in 0..GRID_SIZE {
        if board[row][i] == number {
            return true;
        }
    }
    return false;
}

fn is_number_in_column(board:&[[i32; 9]; 9], number:i32, column:usize) -> bool {
    for i in 0..GRID_SIZE {
        if board[i][column] == number {
            return true;
        }
    }
    return false;
}

fn is_number_in_box(board:&[[i32; 9]; 9], number:i32, row:usize, column:usize) -> bool {
    let local_box_row:usize = row - row % 3;
    let local_box_column:usize = column - column % 3;
    for i in local_box_row..local_box_row + 3 {
        for j in local_box_column..local_box_column + 3 {
            if board[i][j] == number {
                return true;
            }
        }
    }
    return false;
}

// single function to check all conditions

fn is_valid_placement(board:&[[i32; 9]; 9], number:i32, row:usize, column:usize) -> bool {
    return !is_number_in_row(board, number, row) && !is_number_in_column(board, number, column) && !is_number_in_box(board, number, row, column);
}

// helper to print board
fn print_board(board:&[[i32; 9]; 9]) {
    for row in 0..GRID_SIZE {
        for column in 0..GRID_SIZE {
            print!("{}", board[row][column]);
        }
        println!();
    }
}

// recursive solve function

fn solve_board(mut board:[[i32; 9]; 9]) -> bool {
    for row in 0..GRID_SIZE {
        for column in 0..GRID_SIZE {
            if board[row][column] == 0 {
                for number_to_try in 1..GRID_SIZE + 1 {
                    if is_valid_placement(&board, number_to_try.try_into().unwrap(), row, column) {
                        board[row][column] = number_to_try as i32;
                        if solve_board(board) {
                            return true;
                        } else {
                            board[row][column] = 0;
                        }
                    }
                }
                return false;
            }
        }
    }
    print_board(&board);
    return true;
}

fn main() {
    // example board
    let mut board:[[i32;9 ]; 9] = [[7, 0, 2, 0, 5, 0, 6, 0, 0], [0, 0, 0, 0, 0, 3, 0, 0, 0], [1, 0, 0, 0, 0, 9, 5, 0, 0], [8, 0, 0, 0, 0, 0, 0, 9, 0], [0, 4, 3, 0, 0, 0, 7, 5, 0], [0, 9, 0, 0, 0, 0, 0, 0, 8], [0, 0, 9, 7, 0, 0, 0, 0, 5], [0, 0, 0, 2, 0, 0, 0, 0, 0], [0, 0, 7, 0, 4, 0, 2, 0, 3]];
    println!("Before solving: ");
    print_board(&board);
    println!();
    if solve_board(board) {
        println!("Solved!");
    } else {
        println!("This board is unsolvable!! :(");
    }
}
