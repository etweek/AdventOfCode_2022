use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut sum = 0;
    for line in input_lines.iter(){
        println!("{line}");
        let length:usize = line.len();
        let complen:usize = length/2;
        let comp1:&str = &line[0..complen];
        let comp2:&str = &line[complen..length];
        if comp1.len() != comp2.len(){
            panic!("The two halves are of different size!");
        }
        println!("\t{comp1}\n\t{comp2}\n");
        let set1: HashSet<char> = comp1.chars().collect();
        let set2: HashSet<char> = comp2.chars().collect();
        println!("\t{set1:?}\n\t{set2:?}\n");
        let answer: Vec<&char> = set1.intersection(&set2).collect::<Vec<&char>>();
        println!("\tIntersection: {answer:?}");
        if answer.len() != 1{
            panic!("intersection wasn't 1 char");
        }
        let mut val = (*answer[0] as u32 - 'A' as u32)+1;
        if val > 26 {
            val -= 32;
        }
        else{
            val += 26;
        }
        println!("\tValue: {val}");
        sum += val;
        
    }
    println!("\nTotal sum = {sum}");
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
