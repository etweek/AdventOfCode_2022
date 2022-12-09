use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
//use std::collections::VecDeque;
//extern crate matrix;
//use matrix::prelude::*;


fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut pos: Vec<(i32,i32)> = vec![(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)];
    let mut seen:HashSet<(i32,i32)> = vec![(0,0)].into_iter().collect();
    
    for (row,line) in input_lines.iter().enumerate(){
        //println!("{row}:{line}");
        let parts:Vec<&str> = line.split(" ").collect();
        let dir = parts[0];
        let mut steps = i32::from_str_radix(parts[1],10).unwrap();
        println!("{row}: {steps} in direction {dir}");
        while steps > 0{
            pos[0] = move_grid(dir, pos[0]);
            for i in 1..10{
                pos[i] = follow(pos[i],pos[i-1]);
            }
            println!("{:?} --- {:?}", pos[0],pos[9]);
            seen.insert(pos[9]);
            steps -= 1;
        }
        
    }

    println!("Done\n\n");
    let unique = seen.len();
    println!("{unique} unique tail locations ({seen:?})");
}



fn move_grid (dir:&str, p: (i32, i32)) -> (i32, i32){
    match dir{
        "U"=>return (p.0  ,p.1+1),
        "D"=>return (p.0  ,p.1-1),
        "L"=>return (p.0-1,p.1),
        "R"=>return (p.0+1,p.1),
        _  => panic!("unknown direction {dir}"),
    }
}

fn follow (start:(i32,i32), head:(i32,i32)) -> (i32,i32){
    let diff_x = head.0-start.0;
    let diff_y = head.1-start.1;
//    println!("\t{diff_x},{diff_y} away");
    if diff_x.abs() > 1 && diff_y.abs() > 1 {
        // diagonal move
        let dir_x = diff_x / diff_x.abs();
        let dir_y = diff_y / diff_y.abs();
//        println!("\t{dir_x},{dir_y} move");
        return (start.0 + dir_x, start.1 + dir_y);
    }
    else if diff_x.abs() > 1 {
        // diagonal move
        let dir_x = if diff_x.abs() != 0 { diff_x / diff_x.abs()} else { 0 };
        let dir_y = if diff_y.abs() != 0 { diff_y / diff_y.abs()} else { 0 };
//        println!("\t{dir_x},{dir_y} move");
        return (start.0 + dir_x, start.1 + dir_y);
    }
    else if diff_y.abs() > 1 {
        // diagonal move
        let dir_x = if diff_x.abs() != 0 { diff_x / diff_x.abs()} else { 0 };
        let dir_y = if diff_y.abs() != 0 { diff_y / diff_y.abs()} else { 0 };
//        println!("\t{dir_x},{dir_y} move");
        return (start.0 + dir_x, start.1 + dir_y);
    }

    return start;
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
