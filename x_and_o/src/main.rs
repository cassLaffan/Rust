use std::io;
use std::cmp::Ordering;

//this function will loop so long as a player's input is not valid
fn turn_loop() -> bool{
    loop{

    }
    return true;
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
        println!("Player one, please choose a row from 1 to 3 and then a column from 1 to 3.");
        let mut colChoiceOne: i32;
        let mut rowChoiceOne: i32;

        loop{
            println!("Row: ");
            let mut rowOne = String::new();
            io::stdin().read_line(&mut rowOne).expect("Failed to read input.");
            rowChoiceOne = rowOne.parse().unwrap_or_default();

            if(rowChoiceOne < 1 || rowChoiceOne > 3){
                println!("Please choose a row between 1 and 3.");
            }
            else{
                break;
            }
        }


        //will loop until the player enters a correct input
        loop{
            println!("Column: ");
            let mut colOne = String::new();
            io::stdin().read_line(&mut colOne).expect("Failed to read input.");
            colChoiceOne = colOne.parse().unwrap_or_default();

            if(colChoiceOne < 1 || colChoiceOne > 3){
                println!("Please choose a column between 1 and 3.");
            }
            else{
                break;
            }
        }

        if(winState){
            println!("Player one wins!");
            break;
        }

        println!("Player two, please choose a row from 1 to 3 and then a column from 1 to 3.");
        let mut colChoiceTwo: i32;
        let mut rowChoiceTwo: i32;

        loop{
            println!("Row: ");
            let mut rowTwo = String::new();
            io::stdin().read_line(&mut rowTwo).expect("Failed to read input.");
            rowChoiceTwo = rowTwo.parse().unwrap_or_default();

            if(rowChoiceTwo < 1 || rowChoiceTwo > 3){
                println!("Please choose a row between 1 and 3.");
            }
            else{
                break;
            }
        }

        loop{
            println!("Column: ");
            let mut colTwo = String::new();
            io::stdin().read_line(&mut colTwo).expect("Failed to read input.");
            colChoiceTwo = colTwo.parse().unwrap_or_default();

            if(colChoiceTwo < 1 || colChoiceTwo > 3){
                println!("Please choose a column between 1 and 3.");
            }
            else{
                break;
            }

        }
        if(winState){
            println!("Player one wins!");
            break;
        }

    }

}
