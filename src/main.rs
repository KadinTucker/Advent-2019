use std::fs;

fn main() {

    println!("\n\n\\====/ OPERATION SAVE SANTA BEFORE CHRISTMAS \\====/");

    // ==== DAY 1 ====

    //File IO
    let contents = fs::read_to_string("mass_input.txt").expect("Something went wrong reading the file"); //Returns a string
    let lines = contents.lines(); //Returns an iterable

    let mut fuel_total = 0;
    for s in lines {
        let num: i32 = get_fuel(s.parse().expect("Failed to parse string to integer"));
        fuel_total += num;
    }
    println!("\tAmount of fuel needed: {}", fuel_total);

    // ==== DAY 2 ====

    let mut i = 0;
    while i < 100 {
        let mut j = 0;
        while j < 100 {
            if load_and_run_intcode(i, j) == 19690720 {
                println!("\tTo fix the computer: ({}, {}) gives 19690720 as the number in position zero for (noun, verb)", i, j);
                break;
            }
            j += 1;
        }
        i += 1;
    }

    // ==== END ====
    println!("\n");
}

/**
 * Gets the fuel required given an inputted mass
 * Calculates also the fuel needed to transport the fuel recursively
 */
fn get_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + get_fuel(fuel)
}

/**
 * Runs an intcode, entered in as a vector
 * Uses the rules as specified in the problem for Day 2
 * Returns the element at the 0th position at the end of iteration
 */
fn run_intcode(mut codes: Vec<usize>) -> i32 {
    let mut position: usize = 0;
    loop {
        let num = codes[position];
        let store_index = codes[position + 3];
        let next_index = codes[position + 1];
        let after_index = codes[position + 2]; //There must be a better way to put these into a one-liner, but how?
        if num == 1 {
            codes[store_index] = codes[next_index] + codes[after_index];
            position += 4;
        }
        else if num == 2 {
            codes[store_index] = codes[next_index] * codes[after_index];
            position += 4;
        }
        else if num == 99 {
            break;
        }
        else {
            break;
        }
    }
    codes[0] as i32
}

/**
 * Given a noun and a verb, run the intcode modified to give such an answer
 * as it exists to be loaded in from the file intcode.txt
 * Returns the value at position zero at the end of running
 */
fn load_and_run_intcode(noun: i32, verb: i32) -> i32 {
    //File IO
    let contents = fs::read_to_string("intcode.txt").expect("Something went wrong reading the file"); //Returns a string
    let codes = contents.split(","); //Returns an iterable
    
    let mut ints_vec: Vec<usize> = Vec::new();
    for c in codes {
        ints_vec.push(c.parse().expect("Failed to parse string to integer"));
    }
    ints_vec[1] = noun as usize;
    ints_vec[2] = verb as usize;
    run_intcode(ints_vec)
}