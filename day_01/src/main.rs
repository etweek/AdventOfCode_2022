use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    // Need to count the number of elves, start from 1
    let mut num_elves: i32 = 1;
    // remember the elf with the highest calories and their amount
    let mut high_elf: i32 = -1;
    let mut high_elf_amount: i32 = -1;
    let mut second_elf_amount: i32 = -1;
    let mut third_elf_amount: i32 = -1;
    
    // store how much the current elf is carrying
    let mut current_sum: i32 = 0;
    for line in input_lines.iter(){
        if line.len() > 0{
            println!("meal: {line}");
            current_sum += line.parse::<i32>().unwrap();
        }
        else{
            println!("Done with elf {num_elves}, they carry {current_sum}");
            if current_sum > high_elf_amount{
                third_elf_amount = second_elf_amount;
                second_elf_amount = high_elf_amount;
                high_elf_amount = current_sum;
                high_elf = num_elves;
            }
            else if current_sum > second_elf_amount{
                third_elf_amount = second_elf_amount;
                second_elf_amount = current_sum;
            }
            else if current_sum > third_elf_amount{
                third_elf_amount = current_sum;
            }

            num_elves += 1;
            current_sum=0;
        }
    }
    println!("Done with elf {num_elves}, they carry {current_sum}");
    if current_sum > high_elf_amount{
        third_elf_amount = second_elf_amount;
        second_elf_amount = high_elf_amount;
        high_elf_amount = current_sum;
        high_elf = num_elves;
    }
    else if current_sum > second_elf_amount{
        third_elf_amount = second_elf_amount;
        second_elf_amount = current_sum;
    }
    else if current_sum > third_elf_amount{
        third_elf_amount = current_sum;
    }

    println!("Done, the elf with the mostest is {high_elf}, they carry {high_elf_amount}");
    let total = high_elf_amount+second_elf_amount+third_elf_amount;
    println!("Done, the top three carry {total}");
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
