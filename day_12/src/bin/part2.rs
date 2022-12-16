use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
//use std::collections::HashSet;
extern crate matrix;

use matrix::prelude::*;


fn main() {
    let input_file_name = "puzzle_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);

    let rows:usize=input_lines.len();
    let cols:usize=input_lines[0].len();
    let mut map: Compressed<u32> = Compressed::zero((rows, cols));
    let mut distance: Compressed<u32> = Compressed::zero((rows, cols));
    // init to max so it's obvious if they're wrong later
    let mut start:(usize,usize)=(usize::MAX,usize::MAX);
    let mut end:(usize,usize)=(usize::MAX,usize::MAX);

    let mut unvisited:HashMap<(usize,usize),u32> = HashMap::new();
    let mut visited:HashMap<(usize,usize),u32> = HashMap::new();
    let mut a_locs:Vec<(usize,usize)> = Vec::new();
    // compute all possible row/cols and add to unvisited
    for row in 0..rows {
        for col in 0..cols {
            unvisited.insert((row,col),u32::MAX);
        }
    }
    // load the map
    for (row,line) in input_lines.iter().enumerate(){
        //println!("{row}:{line}");

        for (col, c) in line.chars().enumerate(){
            let pos = (row,col);
            let height =
                match c {
                    'S' =>  0,
                    'E' => 25,
                     _  => c as u32 - 'a' as u32
                };
            let dist =
                match c {
                    'E' =>  0,
                     _  => u32::MAX,
                };
            if c == 'a' {
                a_locs.push(pos);

            }
            else if c == 'S' {
                a_locs.push(pos);
                start = (row, col);
            }
            else if c == 'E' {
                end = (row, col);
                unvisited.insert((row,col),0);
            }
            //println!("{c} -> {height}");
            map.set((row,col), height);
            distance.set((row,col), dist);
        }
    }
    println!("{map:?}, {distance:?}");
    // start walking from 'S' using Dijkstra
    // find all unvisited neighbours
    let mut next:Option<(usize,usize)> = Some(end);
    loop{
        check_neighbours(next.unwrap(), &map, &mut unvisited, &mut visited);
        next = find_least(&unvisited);
        println!("Next to check: {next:?}");
        if next == None {
            break;
        }
    }
    println!("Done in {:?} steps\n\n", visited.get(&start));
    let mut smallest: u32 = u32::MAX;
    for pos in a_locs {
        let test = visited.get(&pos);
        match test  {
            Some(dist) => {
                println!("Distance to {pos:?} of height 0: {dist}");
                if *dist < smallest {
                    smallest = *dist;
                }
            }
            None => {//don't care
            }
        }
    }
    println!("Shortest distance to height 0: {smallest}");
    //println!("Map: {map:?}\n\n Visited: {visited:?}\n\n Unvisited: {unvisited:?}");
    

}
fn find_least(unvisited: &HashMap<(usize,usize),u32>) -> Option<(usize,usize)> {
    let mut smallest:u32 = u32::MAX;
    let mut smallest_key:Option<(usize,usize)> = None;
    for element in unvisited.iter() {
        if element.1 < &smallest {
            smallest = *element.1;
            smallest_key = Some(*element.0);
        }
    }
    return smallest_key;


}
fn check_neighbours(cur_pos:(usize,usize),
                    map: &Compressed<u32>,
                    unvisited: &mut HashMap<(usize,usize),u32>,
                    visited: &mut HashMap<(usize,usize),u32>) {
    let cur_height = map.get(cur_pos);
    let cur_distance = *unvisited.get(&cur_pos).unwrap();
    unvisited.remove(&cur_pos);
    println!("Checking neighbours of {cur_pos:?} (height {cur_height}) {cur_distance} from the start");
    if cur_pos.0 > 0 && unvisited.contains_key(&(cur_pos.0-1,cur_pos.1)){
        // block above
        let pos = (cur_pos.0 -1, cur_pos.1);
        check_set(cur_height, cur_distance, pos, &map, unvisited);
    }
    if cur_pos.1 > 0 && unvisited.contains_key(&(cur_pos.0,cur_pos.1-1)){
        // block above
        let pos = (cur_pos.0 , cur_pos.1 -1);
        check_set(cur_height, cur_distance, pos, &map, unvisited);
    }
    if cur_pos.0 < map.rows() && unvisited.contains_key(&(cur_pos.0+1,cur_pos.1)){
        // block above
        let pos = (cur_pos.0 +1, cur_pos.1);
        check_set(cur_height, cur_distance, pos, &map, unvisited);
    }
    if cur_pos.1 < map.columns() && unvisited.contains_key(&(cur_pos.0,cur_pos.1 +1)){
        // block above
        let pos = (cur_pos.0 , cur_pos.1 +1);
        check_set(cur_height, cur_distance, pos, &map, unvisited);
    }
    visited.insert(cur_pos, cur_distance);
}

fn check_set(cur_height:u32, cur_distance:u32, pos:(usize,usize), map: &Compressed<u32>, unvisited: &mut HashMap<(usize,usize),u32>){
    let target_height = map.get(pos);
    let mut height = cur_height;
    if height > 0 {
        height -= 1;
    }
    if target_height >= height {
        // it's reachable
        if cur_distance +1 <= *unvisited.get(&pos).unwrap() {
            // we're quicker
            unvisited.insert(pos, cur_distance+1);
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
