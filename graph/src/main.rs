#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::{BufRead,BufReader,stdin,Read};
use graph::Graph;
use std::env;
use std::collections::HashMap;
use std::fs::File;
mod graph;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing input file"); //check training file
    }
    let f = File::open(&args[1]).expect("Error opening graph file!");
    //let mut graph = Graph::new()
    
}
pub fn read_input<R: Read>(reader: R) -> Graph {
    //let mut input: Vec<String> = Vec::new();  
    let mut graph = Graph::new();
    let mut hash:HashMap<String,Vec<&str>> = HashMap::new();
    let mut lines = BufReader::new(reader).lines();
    let mut counter = 0;
    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}
            let temp = line.clone();
            let mut vertices: Vec<&str> = temp.split_whitespace().collect();
            if vertices.is_empty() {
                println!("Found empty line");
            }
            else {
                graph.set_index(&vertices,counter);
                // let other_nodes = &vertices[1..].to_vec();
                // let node = &vertices[0].to_string();
                // hash.insert(node.clone(),other_nodes.clone());
                hash.insert(vertices[0].to_string(),vertices[..].to_vec().clone());
            }
    }
    return graph;
}