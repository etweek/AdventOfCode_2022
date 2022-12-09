use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
//use std::collections::VecDeque;
extern crate matrix;

use matrix::prelude::*;


fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let m_size:usize=input_lines.len();
    let mut matrix: Compressed<u32> = Compressed::zero((m_size, m_size));

    for (row,line) in input_lines.iter().enumerate(){
        println!("{row}:{line}");

        for (col, c) in line.chars().enumerate(){
            matrix.set((row,col), c.to_digit(10).unwrap());
        }
    }
    println!("{matrix:?}");
    // yeah, this is slow, but meh
    let mut best_score = 0;
    for r in 0..m_size{
        for c in 0..m_size{
            let score = scenic_score(&matrix, r, c);
            if score > best_score {
                let height = matrix.get((r,c));
                println!("Tree ({height}) at {r},{c} has better score of {score}");
                best_score = score;
            }
        }
    }
    println!("Done\n\n");
    println!("{best_score} is the best scenic score");
    

}

fn scenic_score(matrix: &Compressed<u32>, row:usize, col:usize) -> u32{
    // first check that row
    let mut result: u32 = 1;
    let height = matrix.get((row,col));
    let m_sz = matrix.rows();

    // first check to the north
    let mut multiplier = 0;
    for r in (0..row).rev() {
//        println!("{row},{col} of height {height} compared to {r},{col}");
        if matrix.get((r,col)) >= height{
            multiplier += 1;
            break;
        }
        else{
            multiplier += 1;
        }            
    }
    if multiplier == 0 {
        // nothing obscured from the north
        return 0;
    }
    else{
        println!("{row},{col} of height {height} multiplier {multiplier} above");
        result *= multiplier;
    }
    // then check to the south
    multiplier = 0;
    for r in row+1..m_sz{
        if matrix.get((r,col)) >= height{
            multiplier += 1;
            break;
        }
        else{
            multiplier += 1;
        }            
    }
    if multiplier == 0 {
        // nothing obscured from the south
        return 0;
    }
    else{
        println!("{row},{col} of height {height} multiplier {multiplier} below");
        result *= multiplier;
    }

    // west
    multiplier = 0;
    for c in (0..col).rev(){
        if matrix.get((row,c)) >= height{
            multiplier += 1;
            break;
        }
        else{
            multiplier += 1;
        }            
    }
    if multiplier == 0 {
        // nothing obscured from the west
        return 0;
    }
    else{
        println!("{row},{col} of height {height} multiplier {multiplier} left");
        result *= multiplier;
    }
    // then check to the east
    multiplier = 0;
    for c in col+1..m_sz{
        if matrix.get((row,c)) >= height{
            multiplier += 1;
            break;
        }
        else{
            multiplier += 1;
        }            
    }
    if multiplier == 0 {
        // nothing obscured from the east
        return 0;
    }
    else{
        println!("{row},{col} of height {height} multiplier {multiplier} right");
        result *= multiplier;
    }
    return result;
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
