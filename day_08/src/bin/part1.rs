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
    let mut num_visible = 0;
    for r in 0..m_size{
        for c in 0..m_size{
            if is_visible(&matrix, r, c){
                num_visible += 1;
                let height = matrix.get((r,c));
                println!("Tree ({height}) at {r},{c} can be seen");
            }



        }
    }
    println!("Done\n\n");
    println!("{num_visible} trees are visible");
    

}

fn is_visible(matrix: &Compressed<u32>, row:usize, col:usize) -> bool{
    // first check that row
    let mut result: bool = true;
    let height = matrix.get((row,col));
    let m_sz = matrix.rows();

    // first check to the north
    for r in 0..row{
        if matrix.get((r,col)) >= height{
            println!("{row},{col} of height {height} obscured by {r},{col} above");
            result = false;
            break;
        }
    }
    if result {
        // nothing obscured from the north
        return result;
    }
    // then check to the south
    result = true;
    for r in row+1..m_sz{
        if matrix.get((r,col)) >= height{
            println!("{row},{col} of height {height} obscured by {r},{col} below");
            result = false;
            break;
        }
    }
    if result {
        // nothing obscured from the south
        return result;
    }
    // west
    result = true;
    for c in 0..col{
        if matrix.get((row,c)) >= height{
            println!("{row},{col} of height {height} obscured by {row},{c} left");
            result = false;
            break;
        }
    }
    if result {
        // nothing obscured from the west
        return result;
    }
    // east
    result = true;
    for c in col+1..m_sz{
        if matrix.get((row,c)) >= height{
            println!("{row},{col} of height {height} obscured by {row},{c} right");
            result = false;
            break;
        }
    }
    if result {
        // nothing obscured from the east
        return result;
    }
    // nope, can't see it.

    return false;
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
