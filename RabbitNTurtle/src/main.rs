// Implementation of simulation of the popular fable "the rabbit
// and the turtoise". We must create a function each for the rabbit
// and another for the turtoise. Each of the characters must
// suspend the execution for a second adn then evaluate what has
// happened according the probabilities.
//
// The probabilty is calculted by the random function, from 1 to 100
// and determine what have each animal done. 
//
// Consider that there are 70 spaces, the first one is the departure
// and the 70th is the ending one. If the anny of the charecters are
// on the starting position, they will stay at the same place.

use std::process;
use std::sync::{Arc, Mutex};
use std::thread;    // Importing thread lib
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use std::time::Duration;
// Main function 


fn main() {
    println!("Rabbit vs Turtoise");
    println!(">>>>> START <<<<<");
    let winnerT = Arc::new(Mutex::new(5));
    let winnerR = Arc::new(Mutex::new(5));

    let winnerT_clone = winnerT.clone();
    let winnerR_clone = winnerR.clone();

    let turtoise = thread::spawn(move|| { 
        let mut result = Turtoise();
        *winnerT.lock().unwrap()=1;
    });
    
    let rabitt = thread::spawn(move|| {
        let mut result = Rabbit();
        *winnerR.lock().unwrap()=1;
    });
    
    let mut resT;
    let mut resR;
    while true{
        resT = *winnerT_clone.lock().unwrap();
        resR = *winnerR_clone.lock().unwrap();
        if(resT==1)&&(resR==1){
            println!("Both won!");
            process::exit(0);
        }else if resT==1 {
            println!("Turtoise won!");
            process::exit(0);
        }else if resR==1 {
            println!("Rabbit won!");
            process::exit(0);
        }
    }
    
    // dont exit until both processes finish
    turtoise.join().unwrap();
    rabitt.join().unwrap();
}

// Turtoise function
fn Turtoise(){

    let choices = [6, 1, 3];
    let weights = [2, 3, 5];
    /*
     *  2 -> 20%
     *  3 -> 30%
     *  5 -> 50%
     *  2+3+5 -> 100%
     *  https://docs.rs/rand/0.8.5/rand/distributions/struct.WeightedIndex.html
     * */

    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    let mut position    = 1;   // initialize at starting position
    let mut prob;

    while position < 70{

        prob = choices[dist.sample(&mut rng)];
        
        if prob == 6{
            // 20% slip
            if (position-6)< 1{
                position = 1;
            } else {
                position -= 6;
            }
        } else if prob == 1{
            // 30% slow forward
            position += 1;
        } else if prob == 3{
            // 50% slow 
            position += 3;
        }else {
            println!("T:\t\t no movement");
            continue;
        }
        if position > 70{
            position = 70;
        }
        println!("T:\t\t position->{}", position);
        thread::sleep(Duration::from_millis(1000));
    }

}

// Turtoise function
fn Rabbit(){

    let choices = [0, 9, 12, 1, 2];
    let weights = [2, 2,  1, 3, 2];
    /*
     *  2 -> 20%
     *  2 -> 20%
     *  1 -> 10%
     *  3 -> 30%
     *  2 -> 20%
     *  2+2+1+3+2 -> 100%
     *  https://docs.rs/rand/0.8.5/rand/distributions/struct.WeightedIndex.html
     * */

    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    let mut position    = 1;   // initialize at starting position
    let mut prob;

    while position < 70{

        prob = choices[dist.sample(&mut rng)];
        
        if prob == 9{
            // 20% forward
            position += 9;
        } else if prob == 12{
            // 10% backwards
            if (position-12) < 1{
                position = 1;
            }else{
                position -= 12;
            }
        } else if prob == 1{
            // 30% forward
            position += 1;
        } else if prob == 2{
            // 20% backwards
            if (position-2) < 1{
                position = 1;
            }else{
                position -= 2;
            }
        }else {
            println!("R:\t\t no movement");
            continue;
        }
        if position > 70{
            position = 70;
        }
        println!("R:\t\t position->{}", position);
        thread::sleep(Duration::from_millis(1000));
    }    
}
