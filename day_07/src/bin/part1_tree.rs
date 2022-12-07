use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
//use std::collections::VecDeque;
//use trees::tr; 
use trees::Tree;

struct Inode{
    name: String,
    size: i32,
}


fn main() {
    let input_file_name = "sample_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut cwd: &str = "";
    let mut root= Tree::new(Inode { name: String::from("/"), size: 0 });
    let &mut cur_tree = &mut root;
    for line in input_lines.iter(){
        println!("{line}");
        let parts:Vec<&str> = line.split(" ").collect();
        if parts[0] == "$" {
            // it's a command, cd or ls?
            if parts[1] == "cd" {
                cwd = parts[2];
                println!("cd to {cwd}");
                
                // sizes.insert(cwd,0);
                // children.insert(cwd, Vec::new());
            }
            else if parts[1] == "ls" {
                // ignore for now
            }
            else{
                panic!("unknown command");
            }
        }
        else{
            // if parts[0] == "dir" {
            //     children.get_mut(cwd).expect("ICK").push(parts[0]);
            // }
            // else{
            //     *sizes.get_mut(cwd).expect("ICK AGAIN") += i32::from_str_radix(parts[0],10).unwrap();
            // }
        }
    }
    println!("Done\n\n");


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
