mod lib;
use lib::{get_input_from_user, get_random_number};

// s použitím funkcií get_input_from_user a get_random_number vytvorte program, ktorý
// 1. vytvorí náhodné čislo s pomocou funkcie get_random_number
// 2. vypýta si číslo od užívateľa pomocou get_input_from_user
// 3. Ak užívateľ číslo uhádne, program mu zagratuluje a ukončí sa
// Doplňujúce úlohy:
// a) program vyžiada maximálne aj minimálne hodnoty pre náhodné číslo od užívateľa
// b) program nedovolí užívateľovi dať väčšie minimálne číslo ako maximálne a znovu ho požiada o nové čísla
// c) program umožní užívateľovi hádať viac krát
// d) program povie užívateľovi, či je náhodné číslo menšie alebo väčšie, aby mu uľahčil hádanie
// e) vytvorte si vlastné verzie funkcií get_input_from_user a get_random_number

fn valid_guess(rand_min: i32, rand_max: i32, user_guess: i32) -> bool {
    if user_guess <= rand_max && user_guess >= rand_min {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("Please enter minimal and maximal boundary for random number to be generated");
    let rand_min: i32 = get_input_from_user("min: ");
    let rand_max: i32 = get_input_from_user("max: ");
    let rand_num = get_random_number(rand_min, rand_max).unwrap();

    loop {
        let guess: i32 = get_input_from_user("Enter your guess: ");

        if valid_guess(rand_min, rand_max, guess) {
            if guess == rand_num {
                println!("Match!, you won!");
                break;
            } else if guess < rand_num {
                println!("A little more!")
            } else {
                println!("A little less!");
            }
        } else {
            println!("Your guess is not within random generated number range.");
            println!("Please enter number within range <{}, {}>", rand_min, rand_max);
        }
    }
}
