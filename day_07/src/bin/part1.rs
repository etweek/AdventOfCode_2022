use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::str::FromStr;
//use regex::Regex;
//use std::collections::VecDeque;

struct Inode<'a>{
    name: String,
    size: i32,
    parent: Option<&'a Self>,
    children: Vec<Self>,
}



fn main() {
    let input_file_name = "sample_input.txt";
    let input_lines: Vec<String> = read_vector(input_file_name);
    let mut root: Inode = Inode { name: String::from("/"), size: -1, parent: None, children: Vec::new() };
    let mut cwd: &str = "";
    let mut cInode: &mut Inode = &mut root;
    for line in input_lines.iter(){
        println!("{line}");
        let parts:Vec<&str> = line.split(" ").collect();
        if parts[0] == "$" {
            // it's a command, cd or ls?
            if parts[1] == "cd" {
                cwd = parts[2];
                println!("cd to {cwd}");
                // special case
                if cwd == "/" {
                    cInode = &mut root;
                }
                else {
                    // find the dir and make it current
                    println!("Looking for {cwd} in {}",debug_inode(cInode));
                    cInode = cInode.children.iter_mut().find(|c| c.name == cwd).unwrap();
                    // for &mut dir in cInode.children {
                    //     if dir.name == cwd {
                    //         cInode = dir;
                    //     }
                    // }
                    // check it found it
                    assert!(cwd == cInode.name);
                    
                }
            }
            else if parts[1] == "ls" {
                // ignore for now
            }
            else{
                panic!("unknown command");
            }
        }
        else{
            if parts[0] == "dir" {
                let new_inode = Inode { name: String::from(parts[1]), size: -1, parent: Some(cInode), children: Vec::new() };
                cInode.children.push(new_inode);
            }

        }
    }
    println!("Done\n\n");
    print_tree("", &root);
        
}

fn debug_inode(node: &Inode) -> String{
    let mut result: String = String::from(&node.name);
    result += "{";
    for dir in node.children.iter(){
        result += &dir.name;
    }
    result += "}";
    return result;
}

    

fn print_tree(prefix: &str, tree: &Inode) {
    println!("{prefix}{}", tree.name);
    for dir in &tree.children{
        print_tree(&(prefix.to_owned()+"\t"), &dir);
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
