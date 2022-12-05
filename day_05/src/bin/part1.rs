use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::str::FromStr;
use regex::Regex;

//1, 5, 9. 13. 17. 
fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut state = 0;
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut num_stacks = 0;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input_lines.iter(){
        println!("{state}:{line}");
        // perform init
        if 0 == state{
            let line_len = line.len()+1;
            num_stacks = line_len/4;
            println!("Need {num_stacks} stacks");
            let mut count = 0;
            loop{
                stacks.push(Vec::new());
                count += 1;
                if count == num_stacks{
                    break;
                }
            }
            state = 1;
        }
        // check if we're done with setting up
        if 1 == state && line.len() == 0{
            state = 2;
            print_stacks(&stacks);
            stacks = reverse_stacks(stacks);
            print_stacks(&stacks);

        }
        // set up the initial state
        else if 1 == state{
            let content: Vec<char> = line.chars().collect::<Vec<char>>();
            let mut cur_stack = 0;
            loop{
                let mark = content[1+(cur_stack*4)];
                if mark.is_digit(10){
                    // reached the e nd
                    break;
                }
                if ' ' != mark{
                    // push this on the front of the stack
                    stacks[cur_stack].push(mark);
                }
                cur_stack += 1;
                if cur_stack == num_stacks{
                    break;
                }
            }

        }
        else if 2 == state{
            assert!(move_re.is_match(line));
            for cap in move_re.captures_iter(line) {
                let mut crates_to_move:i32 = cap[1].parse::<i32>().unwrap();
                // take one off since we're 0-indexed
                let source:usize = cap[2].parse::<usize>().unwrap() -1;
                let target:usize = cap[3].parse::<usize>().unwrap() -1;
                println!("have to take {crates_to_move} crates off stack {} and move to stack {}", source+1, target+1);
                loop{
                    if 0 == crates_to_move{
                        break;
                    }
                    //println!("\t{}: {:?}",source,stacks[source]);
                    let cur_crate:char = stacks[source].pop().unwrap();
                    stacks[target].push(cur_crate);
                    crates_to_move -= 1;
                }
            }
            print_stacks(&stacks);
        }
    }
    
    let mut result:String = String::from("");
    let mut cur_stack = 0;
    let num_stacks = stacks.len();
    loop{
        result.push(stacks[cur_stack].pop().unwrap());
        
        cur_stack += 1;
        if cur_stack == num_stacks{
            break;
        }
    }
    println!("Final result: {result}");
        
}

fn print_stacks(stacks: &Vec<Vec<char>>){
    let mut cur_stack = 0;
    let num_stacks = stacks.len();
    loop{
        println!("\t{}: {:?}",cur_stack+1,stacks[cur_stack]);
        
        cur_stack += 1;
        if cur_stack == num_stacks{
            break;
        }
    }
}
            
fn reverse_stacks(mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut cur_stack = 0;
    let num_stacks = stacks.len();
    loop{
        stacks[cur_stack].reverse();
        cur_stack += 1;
        if cur_stack == num_stacks{
            break;
        }
    }
    return stacks;
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
