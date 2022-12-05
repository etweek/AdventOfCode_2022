use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
use std::str::FromStr;


fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut sum = 0;
    for line in input_lines.iter(){
        println!("{line}");
        let parts:Vec<&str> = line.split(",").collect();
        assert!(parts.len() == 2);
        // parse the first elfs jobs
        let num_pair_str = parts[0].split('-').collect::<Vec<_>>();
        assert!(num_pair_str.len() == 2);
        let first_elf:Vec<i32> = num_pair_str.iter().map(|s| i32::from_str(s).expect("failed to parse number"))
            .collect::<Vec<_>>();
        assert!(first_elf[0] <= first_elf[1]);
        // parse the second elfs jobs
        let num_pair_str2 = parts[1].split('-').collect::<Vec<_>>();
        assert!(num_pair_str.len() == 2);
        let second_elf:Vec<i32> = num_pair_str2.iter().map(|s| i32::from_str(s).expect("failed to parse number"))
            .collect::<Vec<_>>();
        assert!(second_elf[0] <= second_elf[1]);

        println!("First:  {} to {}\nSecond: {} to {}", first_elf[0], first_elf[1], second_elf[0], second_elf[1]);
        // do the checking
        if contains_range(first_elf[0], first_elf[1], second_elf[0], second_elf[1]) ||
            contains_range(second_elf[0], second_elf[1], first_elf[0], first_elf[1]){
                sum += 1;
            }
    }
    println!("\nAssignments with overlap: {sum}");
}

fn contains_range(start1:i32, end1:i32, start2:i32, end2:i32) -> bool{
    return contains_num(start1, end1, start2) && contains_num(start1, end1, end2);
}

fn contains_num(start: i32, end: i32, check:i32) -> bool{
    return check >= start && check <= end;
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
