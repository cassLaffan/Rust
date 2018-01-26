use std::io;
use std::cmp::Ordering;

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
        println!("Player one, please choose a row from 1 to 3 and then a column from 1 to 3.");

        println!("Row: ");
        let mut rowOne = String::new();
        io::stdin().read_line(&mut rowOne).expect("Failed to read input.");
        let rowChoice = rowOne.parse::<i32>().unwrap();

        if(rowChoice == 1){

        }

        println!("Column: ");
        let mut colOne = String::new();
        io::stdin().read_line(&mut colOne).expect("Failed to read input.");
        let colChoice = colOne.parse::<i32>().unwrap();



        if(winState){
            println!("Player one wins!");
            break;
        }

        println!("Player two, please choose a row from 1 to 3 and then a column from 1 to 3.");

        println!("Row: ");
        let mut rowTwo = String::new();
        io::stdin().read_line(&mut rowTwo).expect("Failed to read input.");

        println!("Column: ");
        let mut colTwo = String::new();
        io::stdin().read_line(&mut colTwo).expect("Failed to read input.");

        if(winState){
            println!("Player one wins!");
            break;
        }
    }

}
