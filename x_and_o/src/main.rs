use std::io;
use std::cmp::Ordering;

//this function will loop so long as a player's input is not valid
fn turn_loop_row() -> i32{
    let mut rowChoice: i32;
    loop{
        println!("Row: ");
        let mut row = String::new();
        io::stdin().read_line(&mut row).expect("Failed to read input.");
        rowChoice = row.parse().unwrap_or_default();

        if(rowChoice < 1 || rowChoice > 3){
            println!("Please choose a row between 1 and 3.");
        }
        else{
            break;
        }
    }
    return rowChoice;
}

//this function will loop so long as a player's input is not valid
fn turn_loop_column() -> i32{
    let mut colChoice: i32;
    loop{
        println!("Column: ");
        let mut col = String::new();
        io::stdin().read_line(&mut col).expect("Failed to read input.");
        colChoice = col.parse().unwrap_or_default();

        if(colChoice < 1 || colChoice > 3){
            println!("Please choose a column between 1 and 3.");
        }
        else{
            break;
        }
    }
    return colChoice;
}

fn main() {
    //and attempt at a simple game of xs and os
    //learning goal: try and make a turn based game
    println!("Tic Tac Toe!");
    println!("Rules: Both players must attempt to put three of their assigned symbol in a straight line. X goes first.");
    println!("_|_|_");
    println!("_|_|_");
    println!("_|_|_");

    let mut rowOneArray : [i32; 3] = [0, 0, 0];
    let mut rowTwoArray : [i32; 3] = [0, 0, 0];
    let mut rowThreeArray : [i32; 3] = [0, 0, 0];//keeps track of whats happening
    let mut winState = false;

    loop{
        let mut colChoiceOne: i32;
        let mut rowChoiceOne: i32;

        println!("Player one, please choose a row from 1 to 3.");
        rowChoiceOne = turn_loop_row();

        println!("Player one, please choose a column from 1 to 3.");
        colChoiceOne = turn_loop_column();

        //will loop until the player enters a correct input

        if(winState){
            println!("Player one wins!");
            break;
        }
        let mut colChoiceTwo: i32;
        let mut rowChoiceTwo: i32;

        println!("Player two, please choose a row from 1 to 3.");
        rowChoiceTwo = turn_loop_row();

        println!("Player two, please choose a column from 1 to 3.");
        colChoiceTwo = turn_loop_column();

        if(winState){
            println!("Player one wins!");
            break;
        }

    }

}
