use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;
use std::iter::Peekable;

enum Item {
    Val(u32),
    List(Vec<Item>)
}
use std::fmt;
impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Item::*;
        match self {
            &Val(ref x) => write!(f, "{:?}", x),
            &List(ref x) => write!(f, "{:?}", x)
        }
    }
}


fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);
    let num_lines = input_lines.len();

    let num_pairs = (num_lines+1)/3;
    let mut pair_strings: Vec<(&String,&String)> = Vec::new();
    // grab the unparsed strings
    for pair_num in 1..=num_pairs {
        let first = &input_lines[(pair_num-1)*3];
        let second = &input_lines[(pair_num-1)*3+1];
        //println!("{first} & {second}");
        pair_strings.push((first,second));
    }
    //now parse them
    let mut pairs: Vec<(Item,Item)> = Vec::new();
    for pair_num in 1..=num_pairs {
        let element = pair_strings.get(pair_num-1).unwrap();
        let new_pair = (parse_string(element.0), parse_string(element.1));
        println!("Built pair: {new_pair:?}");
        pairs.push(new_pair);            
    }
    // Start processing, could have done this in the previous loop
    // but cba
    let mut index_sum = 0;
    for pair_num in 1..=num_pairs {
        let pair = pairs.get(pair_num-1).unwrap();
        let res = compare(&pair.0, &pair.1);

        if res < 0 {
            println!("{pair_num} in order\n");
            index_sum += pair_num;
        }
        else if res > 0 {
            println!("{pair_num} out of order\n");
        }
        else{
            panic!("{pair_num} same");
        }
    }
    println!("Done\n\n");
    println!("Sum of indexes in order is {index_sum}");
}

fn compare(left: &Item, right: &Item) -> i32 {
    println!("Comparing {left:?} with {right:?}");
    match left {
        Item::Val(ref x) => {
            match right {
                // Both integers
                Item::Val(ref y) => {
                    if x < y {
                        return -1;
                    }
                    else if x > y {
                        return 1;
                    }
                    else if x == y {
                        return 0;
                    }
                    else {
                        panic!("Failed to compare {left:?} with {right:?}");
                    }
                },
                Item::List(ref _y) => {
                    // Left is int, right is list
                    // convert the int to list
                    let left_list = Item::List(vec![Item::Val(*x)]);
                    return compare (&left_list, right);
                },
            } // end of matching right
        } // end of left is int
        Item::List(ref x) => {
            match right {

                Item::Val(ref y) => {
                    // Left is List, right is int
                    // convert the int to list
                    let right_list = Item::List(vec![Item::Val(*y)]);
                    return compare(left, &right_list);
                },
                Item::List(ref y) => {
                    // both lists, iterate and recurse
                    for i in 0 .. x.len(){
                        if let Some(val) = y.get(i) {
                            let res = compare(x.get(i).unwrap(), &val);
                            match res {
                                -1 => {return -1},
                                0  => { // keep going!
                                },
                                1  => {return 1},
                                e  => panic!("comparison returned weird value {e}"),
                            }
                        }
                        else{
                            // the left had more elements, bad!
                            println!("right ran out, wrong order");
                            return 1;
                        }
                    }
                    if x.len() < y.len() {
                        // the right had more elements, all good.
                        println!("left ran out, right order");
                        return -1;
                    }
                    return 0;
                },
            } // end of matching right
        } // end of left is list
    } // end of match left

}

fn parse_string(input: &String) -> Item {
//    print!("Parsing {input}...");
    let res:Item = parse_chars(&mut input.chars().peekable());
//    println!(" got {res:?}");
    return res;
}

fn parse_chars(input: &mut Peekable<Chars>) -> Item {
    //println!("\tParsing {input:?}");
    let character = input.next();
    match character {
        Some('[') => return build_list(input),
        Some(c)   => {
            let mut buf:String = String::from(c);
            if let Some(&test) = input.peek() {
                if test.is_digit(10) {
                    buf.push(test);
                }
            }
            //println!("\treturning {buf}");
            return Item::Val(u32::from_str_radix(&buf,10).unwrap())
        },
        None      => panic!("Ran out of chars")
    }
}

fn build_list(input: &mut Peekable<Chars>) -> Item {
    //println!("\t\t listing {input:?}");
    let mut result: Vec<Item> = Vec::new();
    loop {
        let character = input.peek();
        match character {
            Some(',') => {
                // all good, continue
                _ = input.next();
            },
            Some(']') => {
                _ = input.next();
                //println!("Built a list with {result:?}");
                return Item::List(result);
            }
            Some(_) => {
                result.push(parse_chars(input))
            },
            None => {
                panic!("Ran out of chars to make list");
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
