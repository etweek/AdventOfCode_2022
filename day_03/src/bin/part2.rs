use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut sum = 0;
    let mut three: Vec<HashSet<char>> = Vec::new();
    for line in input_lines.iter(){
        println!("{line}");
        let set: HashSet<char> = line.chars().collect::<HashSet<char>>();
        three.push(set);
        if three.len() == 3{
            println!("gathered three");
            let inter: HashSet<char> = three[0].intersection(&three[1]).copied().collect::<HashSet<char>>();
            let answer: Vec<&char> = three[2].intersection(&inter).collect::<Vec<&char>>();
            println!("\tIntersection: {answer:?}");
            assert!(answer.len() == 1);
            let mut val = (*answer[0] as u32 - 'A' as u32)+1;
            if val > 26 {
                val -= 32;
            }
            else{
                val += 26;
            }
            println!("\tValue: {val}");
            sum += val;
            // Clear out the vector
            three.clear();
        }
        
    }
    assert!(three.is_empty());
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
