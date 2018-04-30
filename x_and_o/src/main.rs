use std::io;
use std::cmp::Ordering;

fn set_playing_board(board: &[mut [mut i32, ..3], ..3], turn: i32) -> void{

    if(turn == 0){
        println!("_|_|_");
        println!("_|_|_");
        println!("_|_|_");
    }
    else{
        
    }

}

//this function will loop so long as a player's input is not valid
fn turn_loop_row(rdr: &io::Stdin) -> i32{
    let mut row_choice: i32;
    row_choice = 0;
    loop{
        println!("Row: ");
        let mut row = String::new();
        rdr.read_line(&mut row).expect("Failed to read input.");

        let row_choice: u32 = match row.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if(row_choice < 1 || row_choice > 3){
            println!("Please choose a row between 1 and 3.");
            println!("{}", row_choice);
        }
        else{
            break;
        }
    }
    return row_choice;
}

//this function will loop so long as a player's input is not valid
fn turn_loop_column(rdr: &io::Stdin) -> i32{
    let mut col_choice: i32;
    col_choice = 0;
    loop{
        println!("Column: ");
        let mut col = String::new();
        rdr.read_line(&mut col).expect("Failed to read input.");

        let col_choice: u32 = match col.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if(col_choice < 1 || col_choice > 3){
            println!("Please choose a column between 1 and 3.");
        }
        else{
            break;
        }
    }
    return col_choice;
}

fn find_win_state() -> bool{

}

fn main() {
    let reader = io::stdin();
    //and attempt at a simple game of xs and os
    //learning goal: try and make a turn based game
    println!("Tic Tac Toe!");
    println!("Rules: Both players must attempt to put three of their assigned symbol in a straight line. X goes first.");
    let row_one_array : [mut i32; 3] = [0, 0, 0];
    let row_two_array : [mut i32; 3] = [0, 0, 0];
    let row_three_array : [mut i32; 3] = [0, 0, 0];

    let state : [mut [mut i32, ..3], ..3] = [row_one_array, row_two_array, row_three_array];

    let mut win_state = false;

    loop{
        let mut col_choice_one: i32;
        let mut row_choice_one: i32;

        println!("Player one, please choose a row from 1 to 3.");
        row_choice_one = turn_loop_row(&reader);

        println!("Player one, please choose a column from 1 to 3.");
        col_choice_one = turn_loop_column(&reader);

        //will loop until the player enters a correct input

        if(win_state){
            println!("Player one wins!");
            break;
        }
        let mut col_choice_two: i32;
        let mut row_choice_two: i32;

        println!("Player two, please choose a row from 1 to 3.");
        row_choice_two = turn_loop_row(&reader);

        println!("Player two, please choose a column from 1 to 3.");
        col_choice_two = turn_loop_column(&reader);

        if(win_state){
            println!("Player one wins!");
            break;
        }

    }

}
