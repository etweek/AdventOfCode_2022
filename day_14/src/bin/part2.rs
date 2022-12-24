// always need these
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// extra
extern crate matrix;
use matrix::prelude::*;
use std::cmp;
use std::{thread, time};

fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);
    let escape = '\u{001B}';
    let mut clear: String  = "".to_string();
    clear.push(escape);
    clear += "[2J";
    let pause = time::Duration::from_millis(100);

    // loop through all the lines to parse them
    // We'll first scan them into vectors, whilst finding the max and min
    // so we can build a matrix from them
    // initialize the ranges with where the sand comes from
    let mut x_range: (usize, usize) = (500, 500);
    let mut y_range: (usize, usize) = (0, 0);
    let mut rocks: Vec<Vec<(usize,usize)>> = Vec::new();
    
    for (num,line) in input_lines.iter().enumerate() {
        println!("{num}: {line}");
        let mut rock: Vec<(usize,usize)> = Vec::new();
        let vertices:Vec<&str> = line.split(" -> ").collect();
        for vertex in vertices {
            if let Some((x, y)) = vertex.split_once(',') {
                // Check if we're beyond the range already discovered
                let int_x = x.parse().unwrap();
                let int_y = y.parse().unwrap();
                if int_x < x_range.0 {
                    x_range.0 = int_x;
                }
                if int_x > x_range.1 {
                    x_range.1 = int_x;
                }
                if int_y < y_range.0 {
                    y_range.0 = int_y;
                }
                if int_y > y_range.1 {
                    y_range.1 = int_y;
                }
                rock.push((int_x,int_y));
            }
            else {
                panic!("Vertex {vertex} couldn't be split in twain");
            }
        }
        rocks.push(rock);
    }
    //println!("Found rocks: {rocks:#?}");
    println!("Ranges: x={x_range:?}, y={y_range:?}");
    // fudge the ranges
    // y will need 2 extra
    y_range.1 += 2;
    // x range will need to be twice the y range, centered around 500
    x_range.0 = 500 - (y_range.1 - y_range.0);
    x_range.1 = 500 + (y_range.1 - y_range.0);
    
    let x_size: usize = (x_range.1 - x_range.0 + 1).try_into().unwrap();
    let y_size: usize = (y_range.1 - y_range.0 + 1).try_into().unwrap();
    let origin = (0, 500 - x_range.0);
    let map_size = (y_size, x_size);
    let mut matrix: Compressed<usize> = Compressed::zero(map_size);
    // add in the origin, use 1 for that value
    matrix.set(origin, 1);
    // now populate the matrix/map
    // add the "floor" rock
    rocks.push(vec![(x_range.0,y_range.1),(x_range.1,y_range.1)]);
    for rock in rocks {
        // get all the pairs of vertices that form the lines
        // and fill in the map
        for i in 0.. (rock.len()-1) {
            let first  = &rock.get(i).unwrap();
            let second = &rock.get(i+1).unwrap();
            eprint!("Line from {first:?} to {second:?}: ");
            if first.0 == second.0 {
                // constant x, vary y
                let top:usize    = cmp::min(first.1, second.1);
                let bottom:usize = cmp::max(first.1, second.1);
                for y in top..=bottom {
                    eprint!("{y}..");
                    // imma use 8 for rock
                    matrix.set((y-y_range.0, first.0-x_range.0), 8);
                }
                eprintln!("✔");
            }
            else if first.1 == second.1 {
                // constant y, vary x
                let left:usize  = cmp::min(first.0, second.0);
                let right:usize = cmp::max(first.0, second.0);
                for x in left..=right {
                    eprint!("{x}..");
                    // imma use 8 for rock
                    matrix.set((first.1-y_range.0, x-x_range.0), 8);
                }
                eprintln!("✔");
            }
            else{
                panic!("To-Do: Diagonal line from {first:?} to {second:?}");
            }

        }


    }
    //println!("{matrix:#?}");
    println!("Starting state:");
    draw_map(&matrix);

    // start simulating the sand
    let mut units: u32 = 0;
    loop{
        let final_pos = add_sand(&matrix, &origin);
        if let Some(pos) = final_pos {
            eprintln!("Sand ended up in {pos:?}");
            // I'll use 6 for sand
            matrix.set(pos, 6);
            units += 1;
            if units % 100 == 0 {
                println!("{clear}");
                println!("Next state:");
                draw_map(&matrix);
            }
            //matrix.set(pos, 6);
            //thread::sleep(pause);
        }
        else{
            println!("Map full");
            // add one for that last bit
            units += 1;
            break;
        }

    }
    println!("Last state:");
    draw_map(&matrix);
    println!("Done\n\n");
    println!("Managed {units} units of sand before they fell off the map");
}

// return the ending position of the added sand
fn add_sand(map: &Compressed<usize>, origin: &(usize, usize)) -> Option<(usize, usize)> {
    let mut res = *origin;
    let mut previous = (0,0);
    while res != previous {
        while res != previous {
            while res != previous {
                previous = res;
                let check_pos = (res.0 + 1, res.1);
                let check = is_free(map, &check_pos);
                eprintln!("Checking down from {res:?} is free? {check:?}");
                match check {
                    Some(true) => {
                        // we can move down
                        res = check_pos;
                    }
                    Some(false) => {
                        // to-do, try to the side
                    }
                    None => {
                        // fell off the map
                        panic!("fell off the sides!");
                    }
                }
            }
            // that was all the straight downs, now try diagonal left
            if res.1 > 0 {
                let check_pos = (res.0 + 1, res.1 - 1);
                let check = is_free(map, &check_pos);
                eprintln!("Checking down-left from {res:?} is free? {check:?}");
                match check {
                    Some(true) => {
                        // we can move down-left
                        res = check_pos;
                    }
                    Some(false) => {
                        // to-do, try to the other side
                    }
                    None => {
                        // fell off the map
                        panic!("fell off the sides!");
                    }
                }
            }
            else{
                // fall off the map
                panic!("fell off the sides!");
            }
        }
        // now diagonal-right
        let check_pos = (res.0 + 1, res.1 + 1);
        let check = is_free(map, &check_pos);
        eprintln!("Checking down-right from {res:?} is free? {check:?}");
        match check {
            Some(true) => {
                // we can move down-left
                res = check_pos;
            }
            Some(false) => {
                // to-do, try to the side
            }
            None => {
                // fell off the map
                panic!("fell off the sides!");
            }
        }
    }
    if res == *origin {
        // we reached the top, no more!
        return None;
    }
    else{
        return Some(res);
    }
}


fn is_free (map: &Compressed<usize>, test: &(usize, usize)) -> Option<bool> {
    // bounds check, don't need to check negative values
    if  test.1 >= map.columns() ||
        test.0 >= map.rows() {
            return None;
        }
    let target = map.get(*test);
    //println!("\tmap element at {test:?} is {target}");
    match target {
        0 => return Some(true),
        _ => return Some(false)
    }
}

    
fn draw_map(map: &Compressed<usize>) {
    eprintln!("\tDrawing map: {} by {}", map.rows(), map.columns());
    for y in 0..map.rows() {
        for x in 0..map.columns() {
            //eprintln!("Drawing {x},{y}");
            match map.get((y,x)) {
                0 => print!("."),
                1 => print!("+"),
                6 => print!("o"),
                7 => print!("@"),
                8 => print!("#"),
                x => print!("{x}"),
            }
        }
        println!("");
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
