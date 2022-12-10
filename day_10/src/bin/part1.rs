use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
//use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
//use std::collections::VecDeque;
//extern crate matrix;
//use matrix::prelude::*;


fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut registers: HashMap<char,i32> = HashMap::from([
        ('x', 1),
        ('s', 0),
    ]);
    let mut cycle:u32 = 0;
    for (row,line) in input_lines.iter().enumerate(){
        println!("{row}:{line}");
        // get the current value
        let mut x = *registers.get_mut(&'x').unwrap();
        // grab the isntruction and optionally the value
        let parts:Vec<&str> = line.split(" ").collect();
        let instruction = parts[0];
        if instruction == "noop"{
            cycle += 1;
            registers = on_tick(cycle, registers);
        }
        else if instruction == "addx"{
            cycle += 1;
            registers = on_tick(cycle, registers);
            cycle += 1;
            registers = on_tick(cycle, registers);
            let value:i32 = i32::from_str_radix(parts[1],10).unwrap();
            x += value;
//            println!("\t x={x}");
        }
        else{
            panic!("Unknown instruction {instruction}");
        }
        registers.insert('x', x);// put it back
        
        
    }

    println!("Done\n\n");
    println!("{cycle} cycles completed, registers: {registers:?}");
}

fn on_tick(cycle: u32, mut registers: HashMap<char,i32>) -> HashMap<char,i32> {
    let x = *registers.get(&'x').unwrap();
    println!("\t\t{cycle}: {x}");
    // calculate signal strength
    if cycle == 20 {
        let strength = cycle as i32 * x;
        let mut s = *registers.get_mut(&'s').unwrap();
        s += strength;
        registers.insert('s', s);// put it back
        println!("\t\t\t{cycle}: signal strength: {strength}");
    }
    else if cycle > 20 && (cycle-20)%40 == 0 {
        let strength = cycle as i32 * x;
        let mut s = *registers.get_mut(&'s').unwrap();
        s += strength;
        registers.insert('s', s);// put it back
        println!("\t\t\t{cycle}: signal strength: {strength}");
    }
    return registers;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_vector<P>(filename:P) -> Vec<String>
where P: AsRef<Path>, {
    // build up the result
    let mut result: Vec<String> = vec![];
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(element) = line {
                result.push(element);
            }
        }
    }
    return result;
}    
