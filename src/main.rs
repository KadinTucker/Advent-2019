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

    // ==== DAY 3 ====

    let wire1 = generate_wire("wire1.txt");
    let wire2 = generate_wire("wire2.txt");

    let mut min_distance = -1; //-1 is the default minimum; no lengths or distances should be negative, so this is "infinity"
    let mut min_length = -1;

    for s1 in &wire1 {
        for s2 in &wire2 {
            let new_dists = match intersection(s1, s2) {
                Some(v) => v,
                None => (min_distance, min_length)
            };
            //println!("{}", new_dist);
            if new_dists.0 < min_distance || min_distance < 0 {
                min_distance = new_dists.0;
            }
            if new_dists.1 < min_length || min_length < 0 {
                min_length = new_dists.1;
            }
        }
    }

    println!("\tMinimum wire crossing distance: {}", min_distance);
    println!("\tMinimum length along wire crossing: {}", min_length);

    // ==== DAY 4 ====

    println!("Number of possible passwords btwn 172930-683082: {}", count_passwords(172930, 683082));

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
    let mut prev_len = 0; //Previous combined length of all wires

    for c in commands {
        let direction = c.as_bytes()[0];
        let num: i32 = c[1..].parse().expect("Failed to parse string to number");
        //println!("{}", direction == 85 || direction == 76 || direction == 82 || direction == 68);
        match direction { 
            82 => {pieces.push(construct_wire_piece(true, false, currY, currX, currX + num, prev_len)); currX += num}, //right
            76 => {pieces.push(construct_wire_piece(true, true, currY, currX - num, currX, prev_len)); currX -= num}, //left
            85 => {pieces.push(construct_wire_piece(false, false, currX, currY, currY + num, prev_len)); currY += num}, //up
            68 => {pieces.push(construct_wire_piece(false, true, currX, currY - num, currY, prev_len)); currY -= num}, //down
            _ => {}
        }
        prev_len += num;
        //println!("{}", prev_len);
    }
    pieces
}

/**
 * Struct defining a segment of a wire
 * Start should be always less than finish
 */
struct WirePiece {
    horizontal: bool, //Whether or not the wire is horizontal
    negative: bool, //If the wire points down or left it is considered to be pointing negative; used to determine the length along the wire when finding an intersection
    position: i32, //The static position along which the segment runs
    start: i32, //The starting position
    finish: i32, //The finishing position
    total_len: i32 //The total length of all previous wires up until the start of this wire
}

/**
 * Constructor for the WirePiece struct
 */
fn construct_wire_piece(horizontal: bool, negative: bool, position: i32, start: i32, finish: i32, total_len: i32) -> WirePiece {
    WirePiece {
        horizontal,
        negative,
        position,
        start,
        finish,
        total_len
    }
}

/**
 * Determines an intersection between two wire segments
 * Returns an Option that contains a value if there is an intersection and the Manhattan distance of that intersection from the origin
 * the second item in the tuple is then the minimum of the two distances along the wires
 */
fn intersection(first: &WirePiece, second: &WirePiece) -> Option<(i32, i32)> {
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
        //If the wire segment is negative we take the total previous plus the difference between the coordinate and the finish
        //Else we use the start instead of the finish
        let hor_distance = horizontal.total_len + match horizontal.negative {
            true => abs(horizontal.finish - vertical.position),
            false => abs(horizontal.start - vertical.position)
        };
        let ver_distance = vertical.total_len + match vertical.negative {
            true => abs(vertical.finish - horizontal.position),
            false => abs(vertical.start - horizontal.position)
        };
        return Some((abs(horizontal.position) + abs(vertical.position), hor_distance + ver_distance));
    }
    None
}

fn abs(x: i32) -> i32 {
    //println!("{}", x);
    if x < 0 {
        return -x;
    }
    x
}

/**
 * Counts the number of valid passwords according to the specifications of the elves' memories
 */
fn count_passwords(left: i32, right: i32) -> i32 {
    let mut counter = left;
    if counter < 100000 {
        counter = 100000;
    }
    let mut numValid = 0;
    while counter <= right && counter <= 999999 {
        let mut valid = false;
        for i in 1..6 { //any valid password will be six digits long
            if !valid && get_digit_at(counter, i) == get_digit_at(counter, i + 1) {
                valid = true; //Valid if there exist doubled digits
            }
            if get_digit_at(counter, i) > get_digit_at(counter, i + 1) { //If the next digit is lower
                valid = false; //the password decreases and is therefore invalid; we will remove any validity that we may have gotten
                //println!("decreased");
                //counter += i32::pow(10, (5 - i) as u32);
                //println!("new num: {}", counter);
                break; 
            }
        }
        if valid { //Assuming everything else has gone well up to this point, the password is valid if there exist adjacents
            numValid += 1;
        } 
        counter += 1;
    }
    numValid
}

/**
 * Gets the digit at the given index of a number
 * Indexes start at 1, because that's the way the algorithm works
 */
fn get_digit_at(x: i32, index: i32) -> i32 {
    let mut length = 0;
    let mut num = x;
    while num > 0 {
        num /= 10;
        length += 1;
    }
    let mut digit = x;
    for i in 0..length - index {
        digit /= 10;
    }
    //println!("{}th digit of {}: {}", index, x, digit % 10);
    digit % 10
}