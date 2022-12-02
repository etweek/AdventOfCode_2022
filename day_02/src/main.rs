use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let part = 2;
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    // A == Rock      = 1
    // B == Paper     = 2
    // C == Scissors  = 3

    let starting_score: HashMap<&str, i32> = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    // Loss = 0
    // Draw = 3
    // Win  = 6
    if part == 1{
        // X == Rock      = 1
        // Y == Paper     = 2
        // Z == Scissors  = 3
        let mut score = 0;
        for line in input_lines.iter(){
            // split the line into two
            let parts:Vec<&str> = line.split(" ").collect();
            let opponent = parts[0];
            let mine = parts[1];
            //println!("Theirs: {opponent} vs mine: {mine}");
            score += match opponent{
                "A" => rock(&starting_score, mine),
                "B" => paper(&starting_score, mine),
                "C" => scissors(&starting_score, mine),
                _   => panic!("I don't understand {opponent}")
            };
        }
        println!("Total score = {score}");
    }
    else if part == 2{
        // X == Lose      = 0
        // Y == Draw      = 3
        // Z == Win       = 6
        let mut score = 0;
        for line in input_lines.iter(){
            // split the line into two
            let parts:Vec<&str> = line.split(" ").collect();
            let opponent = parts[0];
            let outcome = parts[1];
            //println!("Theirs: {opponent} vs mine: {mine}");
            score += match outcome{
                "X" => lose(opponent),
                "Y" => draw(opponent),
                "Z" => win(opponent),
                _   => panic!("I don't understand {outcome}")
            };
        }
        println!("Total score = {score}");




    }
}
fn lose(opponent: &str) -> i32{
    let mut result =0;
    result += match opponent{
        "A" => 3, // they're rock, so I need to be scissors - 3
        "B" => 1,
        "C" => 2,
        _   => panic!("Lose: I don't understand {opponent}"),
    };
    return result;
}

fn draw(opponent: &str) -> i32{
    let mut result = 3;
    result += match opponent{
        "A" => 1, // they're rock, so I need to be rock - 1
        "B" => 2,
        "C" => 3,
        _   => panic!("Draw: I don't understand {opponent}"),
    };
    return result;
}

fn win(opponent: &str) -> i32{
    let mut result = 6;
    result += match opponent{
        "A" => 2, // they're rock, so I need to be paper - 2
        "B" => 3,
        "C" => 1,
        _   => panic!("Win: I don't understand {opponent}"),
    };
    return result;
}

fn rock(scoring: &HashMap<&str,i32>, mine: &str) -> i32{
    let mut result = scoring[mine];
    println!("Rock: {mine} scored {result:?}");
    result += match mine{
        "X" => 3,
        "Y" => 6,
        "Z" => 0,
        _   => panic!("Rock: I don't understand {mine}"),
    };
    
    return result;
}

fn paper(scoring: &HashMap<&str,i32>, mine: &str) -> i32{
    let mut result = scoring[mine];
    println!("Paper: {mine} scored {result:?}");
    result += match mine{
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _   => panic!("Paper: I don't understand {mine}"),
    };
    
    return result;
}


fn scissors(scoring: &HashMap<&str,i32>, mine: &str) -> i32{
    let mut result = scoring[mine];
    println!("Scissors: {mine} scored {result:?}");
    result += match mine{
        "X" => 6,
        "Y" => 0,
        "Z" => 3,
        _   => panic!("Scissors: I don't understand {mine}"),
    };
    
    return result;
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
