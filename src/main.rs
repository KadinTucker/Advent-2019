use std::fs;
use std::num;

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

    // ==== DAY 3 ====

    let wire1 = generate_wire("wire1.txt");
    let wire2 = generate_wire("wire2.txt");

    let mut min_distance = -1; //All input values appear to be < 1000 in length

    for s1 in &wire1 {
        for s2 in &wire2 {
            let new_dist = match intersection(s1, s2) {
                Some(v) => v,
                None => min_distance
            };
            //println!("{}", new_dist);
            if new_dist < min_distance || min_distance < 0 {
                min_distance = new_dist;
            }
        }
    }

    println!("\tMinimum wire crossing distance: {}", min_distance);

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

/**
 * Generates a wire, as a vector of WirePiece structs,
 * loaded from a file with the given name
 */
fn generate_wire(filename: &str) -> Vec<WirePiece> {
    //File IO
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file"); //Returns a string
    let commands = contents.split(","); //Returns an iterable

    let mut currX = 0;
    let mut currY = 0;

    let mut pieces: Vec<WirePiece> = Vec::new();

    for c in commands {
        let direction = c.as_bytes()[0];
        let num: i32 = c[1..].parse().expect("Failed to parse string to number");
        //println!("{}", direction == 85 || direction == 76 || direction == 82 || direction == 68);
        match direction { 
            82 => {pieces.push(construct_wire_piece(true, currY, currX, currX + num)); currX += num}, //right
            76 => {pieces.push(construct_wire_piece(true, currY, currX - num, currX)); currX -= num}, //left
            85 => {pieces.push(construct_wire_piece(false, currX, currY, currY + num)); currY += num}, //up
            68 => {pieces.push(construct_wire_piece(false, currX, currY - num, currY)); currY -= num}, //down
            _ => {}
        }
    }
    pieces
}

/**
 * Struct defining a segment of a wire
 * Start should be always less than finish
 */
struct WirePiece {
    horizontal: bool, //Whether or not the wire is horizontal
    position: i32, //The static position along which the segment runs
    start: i32, //The starting position
    finish: i32 //The finishing position
}

/**
 * Constructor for the WirePiece struct
 */
fn construct_wire_piece(horizontal: bool, position: i32, start: i32, finish: i32) -> WirePiece {
    WirePiece {
        horizontal,
        position,
        start,
        finish
    }
}

/**
 * Determines an intersection between two wire segments
 * Returns an Option that contains a value if there is an intersection and the Manhattan distance of that intersection from the origin
 */
fn intersection(first: &WirePiece, second: &WirePiece) -> Option<i32> {
    let horizontal = first;
    let vertical = second;
    if first.horizontal && !second.horizontal {
        let horizontal = first;
        let vertical = second;
    } else if second.horizontal && !first.horizontal {
        let horizontal = second;
        let vertical = first;
    } else {
        return None;
    }
    //println!("first: ({}, {}, {}, {}); second: ({}, {}, {}, {})", first.horizontal, first.position, first.start, first.finish, second.horizontal, second.position, second.start, second.finish);
    if horizontal.position > vertical.start && horizontal.position < vertical.finish 
            && vertical.position > horizontal.start && vertical.position < horizontal.finish {
        //println!("{}", abs(horizontal.position) + abs(vertical.position));
        return Some(abs(horizontal.position) + abs(vertical.position));
    }
    None
}

fn abs(x: i32) -> i32{
    //println!("{}", x);
    if x < 0 {
        return -x;
    }
    x
}