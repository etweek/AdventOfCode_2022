use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut window: VecDeque<char> = VecDeque::with_capacity(4);
    for line in input_lines.iter(){
        println!("{line}");
        for (i,cur_char) in line.chars().enumerate(){
            window.push_back(cur_char.clone());
            if window.len() > 4{
                window.pop_front();
            }
            if window.len() == 4{
                let test:HashSet<char> = window.clone().into_iter().collect();
                if test.len() == 4{
                    println!("Found unique 4 chars {window:?} ending at offset {i}, char {}",i+1);
                    break;
                }

            }
        }
    }
    
        
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
