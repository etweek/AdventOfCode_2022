use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
//use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
//use std::collections::VecDeque;



fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut sizes: HashMap<String, i32> = HashMap::from([
        ("/".to_string(), 0),
    ]);
    let mut stack: Vec<&str> = vec![];
    //let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut path: String = "/".to_string();
    for line in input_lines.iter(){
        println!("{line}");
        let parts:Vec<&str> = line.split(" ").collect();
        if parts[0] == "$" {
            // it's a command, cd or ls?
            if parts[1] == "cd" {
                if parts[2] == "/"{
                    path  = "/".to_string();
                    stack = vec![];
                }
                else if parts[2] == ".."{
                    stack.pop();
                    println!("\t{path}$ cd up one");
                    path="/".to_string()+&stack.join("/");
                    
                }
                else{
                    let cwd = parts[2];
                    stack.push(&cwd);
                    println!("\t{path}$ cd to {cwd}");
                    let new_path="/".to_string()+&stack.join("/");
                    path=new_path.clone();
                    sizes.insert(new_path,0);
                }
            }
            else if parts[1] == "ls" {
                // ignore for now
            }
            else{
                panic!("unknown command");
            }
        }
        else{
            // ignore dir entries to start with
            if parts[0] == "dir" {
                //ignore
            }
            else{
                let size:i32 = i32::from_str_radix(parts[0],10).unwrap();
                // ignore the name
                // add the size to the path tally
                for i in 0..=stack.len(){
                    let s = &stack[0..i];
                    let temp_path = "/".to_string()+&s.join("/");
                    println!("\tAdding {size} to {temp_path}");
                    *sizes.get_mut(&temp_path).expect("Should have had an entry for {path}") += size;
                }
            }
        }
    }
    println!("Done\n\n");
    println!("{sizes:?}");
    let max_size = 100000;
    let mut total = 0;
    for (key, value) in sizes{
        if value <= max_size{
            println!("{key} has less than {max_size} with {value}");
            total += value;
        }
    }
    println!("total of all small dirs is {total}");
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
