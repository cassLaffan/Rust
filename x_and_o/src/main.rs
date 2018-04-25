use std::io;
use std::cmp::Ordering;

//this function will loop so long as a player's input is not valid
fn turn_loop_row(reader: stdin) -> i32{
    let mut row_choice: i32;
    loop{
        println!("Row: ");
        let mut row = String::new();
        reader.read_line(&mut row).expect("Failed to read input.");
        row_choice = row.trim().parse::<i32>();

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
fn turn_loop_column(reader: stdin) -> i32{
    let mut col_choice: i32;
    loop{
        println!("Column: ");
        let mut col = String::new();
        reader.read_line(&mut col).expect("Failed to read input.");
        col_choice = col.parse().unwrap_or_default();

        if(col_choice < 1 || col_choice > 3){
            println!("Please choose a column between 1 and 3.");
        }
        else{
            break;
        }
    }
    return col_choice;
}

fn main() {
    let reader = io::stdin();
    //and attempt at a simple game of xs and os
    //learning goal: try and make a turn based game
    println!("Tic Tac Toe!");
    println!("Rules: Both players must attempt to put three of their assigned symbol in a straight line. X goes first.");
    println!("_|_|_");
    println!("_|_|_");
    println!("_|_|_");

    let mut row_one_array : [i32; 3] = [0, 0, 0];
    let mut row_two_rray : [i32; 3] = [0, 0, 0];
    let mut row_three_array : [i32; 3] = [0, 0, 0];//keeps track of whats happening
    let mut win_state = false;

    loop{
        let mut col_choice_one: i32;
        let mut row_choice_one: i32;

        println!("Player one, please choose a row from 1 to 3.");
        row_choice_one = turn_loop_row(reader);

        println!("Player one, please choose a column from 1 to 3.");
        col_choice_one = turn_loop_column(reader);

        //will loop until the player enters a correct input

        if(win_state){
            println!("Player one wins!");
            break;
        }
        let mut col_choice_two: i32;
        let mut row_choice_two: i32;

        println!("Player two, please choose a row from 1 to 3.");
        row_choice_two = turn_loop_row(reader);

        println!("Player two, please choose a column from 1 to 3.");
        col_choice_two = turn_loop_column(reader);

        if(win_state){
            println!("Player one wins!");
            break;
        }

    }

}
