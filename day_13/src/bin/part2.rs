use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;
use std::iter::Peekable;
use std::cmp::Ordering;
//use std::hash::Hash;

#[derive(Clone)]
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

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        return compare(self,other) == Ordering::Equal;
    }
}

impl Eq for Item {}


// impl Hash for Item {
//     fn hash<H: Hasher>(&self, hasher: &mut H) {
//         match self {
//             &Val(ref x) => x.hash(hasher),
//             &List(ref x) => x.hash(hasher),
//         }            
//     }
// }

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(compare(self, other));
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        return compare(self, other);
    }
}

fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    // we want a vector of parsed lines that we sort using our comparison operator
    let mut packets: Vec<Item> = Vec::new();

    // loop through all the lines to parse them
    for line in input_lines {
        // ignore empty lines
        if line.len() > 1 {
            let parsed = parse_string(&line);
            println!("Parsed {parsed:?}");
            packets.push(parsed);
        }
    }

    // calculate part1 to make sure we haven't messed it up
    let mut part1 = 0;
    for pair in 1 ..=packets.len()/2 {
        let left = packets.get((pair-1)*2).unwrap();
        let right = packets.get((pair*2)-1).unwrap();
        let res = compare(left, right);
        println!("{pair}: {res:?}");
        if res == Ordering::Less {
            part1 += pair;
        }
    }

    //add in  divider packets
    let first_divider = parse_string(&String::from("[[2]]"));
    let fd = first_divider.clone();
    let second_divider = parse_string(&String::from("[[6]]"));
    let sd = second_divider.clone();
    packets.push(fd);
    packets.push(sd);
    // sort
    packets.sort_by(|a, b| compare(a,b));
    // find indices
    let i1: u32 = packets.binary_search(&first_divider)
        .expect("First Divider should be in the packets!")
        .try_into()
        .unwrap();
    let i2: u32 = packets.binary_search(&second_divider)
        .expect("Second Divider should be in the packets!")
        .try_into()
        .unwrap();
    let decoder_key = (i1 + 1)  * (i2 + 1);

    println!("\nSorted: {packets:#?}");
    println!("Done\n\n");
    println!("Part 1: {part1}\n");
    println!("Part 2: {decoder_key}");
    
    
}

fn compare(left: &Item, right: &Item) -> Ordering {
    println!("Comparing {left:?} with {right:?}");
    match left {
        Item::Val(ref x) => {
            match right {
                // Both integers
                Item::Val(ref y) => {
                    if x < y {
                        return Ordering::Less;
                    }
                    else if x > y {
                        return Ordering::Greater;
                    }
                    else if x == y {
                        return Ordering::Equal;
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
                                Ordering::Less     => {return Ordering::Less},
                                Ordering::Equal    => { },// keep going!
                                Ordering::Greater  => {return Ordering::Greater},
                            }
                        }
                        else{
                            // the left had more elements, bad!
                            println!("right ran out, wrong order");
                            return Ordering::Greater;
                        }
                    }
                    if x.len() < y.len() {
                        // the right had more elements, all good.
                        println!("left ran out, right order");
                        return Ordering::Less;
                    }
                    return Ordering::Equal;
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
