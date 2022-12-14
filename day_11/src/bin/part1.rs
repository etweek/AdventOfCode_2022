use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//usec std::collections::HashMap;
//use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
//use std::collections::VecDeque;
//extern crate matrix;
//use matrix::prelude::*;
use std::mem;

fn add (left: i32, right:i32) -> i32 {
    return left + right;
}
fn multiply  (left: i32, right:i32) -> i32 {
    return left * right;
}

type Binop = fn(i32, i32) -> i32;

struct Monkey {
    items: Vec<i32>,
    operation: Binop,
    constant: i32,
    test: i32,
    true_target: usize,
    false_target: usize,
    inspections:usize,
}

fn main() {
    let input_file_name = "sample_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let mut monkeys: Vec<Monkey> = Vec::new();
    let num_lines = input_lines.len();
    // add one because the last monkey doesn't get a newline after
    let num_monkeys = (num_lines+1) / 7;
    println!("Input has {num_monkeys} monkeys defined");
    
    for monkey_num in 1..=num_monkeys{
        let monkey_slice = &input_lines[(monkey_num-1)*7 .. (monkey_num*7)-1];
        println!(" Monkey number {monkey_num} defined by {:?}", monkey_slice);
        monkeys.push(build_monkey(monkey_slice));
        print_monkey(&monkey_num, &monkeys[monkey_num-1]);
    }
    // evaluate a round
    let num_rounds = 20;
    for round in 1..=num_rounds {
        println!("Round {round}");
        for monkey_num in 0..num_monkeys{
            println!("\tMonkey {monkey_num}:");
            let items: Vec<i32>;
            let operation: Binop;
            let constant: i32;
            let test: i32;
            let true_target: usize;
            let false_target: usize;
            {
                let monkey = &mut monkeys[monkey_num];
                // take ownership of items and replace with an empty vec
                items = mem::take(&mut monkey.items);
                // copy the other values so we can relinquish the borrow
                operation = monkey.operation;
                constant = monkey.constant;
                test = monkey.test;
                true_target = monkey.true_target;
                false_target = monkey.false_target;
                monkey.inspections += items.len();
            }
            for item in items {
                // perform operation
                let mut inner_constant = constant;
                if inner_constant == -1 {
                    inner_constant = item;
                }
                let new_worry = (operation)(item, inner_constant);
                // get bored
                let bored_worry = new_worry / 3;
                // perform test
                let remainder = bored_worry % test;
                if remainder == 0 {
                    println!("\t\tThrowing item of worry {item} -> {new_worry} -> {bored_worry} / {} to monkey {}",test, true_target);
                    let target = &mut monkeys[true_target];
                    target.items.push(bored_worry);
                }
                else{
                    println!("\t\tThrowing item of worry {item} -> {new_worry} -> {bored_worry} not / {} to monkey {}",test, false_target);
                    let target = &mut monkeys[false_target];
                    target.items.push(bored_worry);
                }
            }
        }
        for monkey_num in 0..num_monkeys{
            print_monkey(&monkey_num, &monkeys[monkey_num]);
        }
    }
    println!("Done\n\n");
    // find the two highest inspections
    let mut inspect_1 = 0;
    let mut inspect_2 = 0;
    for monkey_num in 0..num_monkeys{
        if monkeys[monkey_num].inspections > inspect_1 {
            inspect_2 = inspect_1;
            inspect_1 = monkeys[monkey_num].inspections;
        }
        else if monkeys[monkey_num].inspections > inspect_2 {
            inspect_2 = monkeys[monkey_num].inspections;
        }
    }
    let total = inspect_1 * inspect_2;
    println!("Total inspections by the top two rascals {total}");
    
}

fn print_monkey(num: &usize, monkey: &Monkey){
    println!("\t {num}: {:?} {} {} {} {}, Inspections: {} ",monkey.items, monkey.constant, monkey.test, monkey.true_target, monkey.false_target, monkey.inspections);

}

fn build_monkey(lines: &[String]) -> Monkey{
    let mut result = Monkey {
        items: vec![],
        operation: add,
        constant: 0,
        test: 0,
        true_target: 0,
        false_target: 0,
        inspections: 0
    };
    // ignore the first line, it's just the monkey number and we know/assume they're
    // monotonically increasing
    result.items = lines[1].split(": ") // we start with "Starting items: x, y, z"
        .skip(1)  // ignore the "starting items"
        .next().expect("malformed input, no starting items") // ensure there was a second half
        .split(", ") // now split into a futher list
        .filter_map(|portion| portion.parse::<i32>().ok()) //parse into ints
        .collect::<Vec<i32>>(); // and create a vec
    // now look at the operation
    let op_parts:Vec<&str> = lines[2]
        .split(" = ")
        .skip(1)
        .next().expect("malformed input, Operation doesn't have RHS")
        .split(" ") // now have an iterator over "old [op] [value]"
        .collect::<Vec<&str>>();
    if op_parts[1] == "+" {
        result.operation = add;
    }
    else if op_parts[1] == "*" {
        result.operation = multiply;
    }
    else{
        panic!("{} isn't a valid operation", op_parts[1])
    }
    let test = op_parts[2].parse::<i32>();

    match test {
        Ok(ok) => result.constant = ok,
        // dodgy hack alert
        // I'll mark -1 to mean "old"
        Err(_e) => result.constant = -1,
    }
    // next deal with the test
    // it's always divisible by something
    result.test =  lines[3]     // start with the whole line
        .find(" by ")           // find the separator
        .and_then(|i| lines[3][i+4..] // hop past the index
                  .parse()            // parse as a number
                  .ok())              // assume it went well
        .unwrap();              // grab the output
    // next the true target
    result.true_target = lines[4] //start with the whole line
        .find("monkey ") // locate
        .and_then(|i| lines[4][i+7..] //hop past
                  .parse()
                  .ok())
        .unwrap();
    // finally the false target
    result.false_target = lines[5] //start with the whole line
        .find("monkey ") // locate
        .and_then(|i| lines[5][i+7..] //hop past
                  .parse()
                  .ok())
        .unwrap();
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
