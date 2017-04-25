#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::{BufRead,BufReader,stdin,Read};
use graph::Graph;
use std::env;
use std::collections::HashMap;
use std::fs::File;
mod graph;
mod bfs;
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
    let mut index_map : HashMap<String,usize> = HashMap::new();
    let mut counter = 0;
    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}
            let temp = line.clone();
            let mut vertices: Vec<&str> = temp.split_whitespace().collect();
            if vertices.is_empty() {
                println!("Found empty line");
            }
            else {
                let mut nodes:Vec<String> = vec![];
                for s in &vertices {
                    nodes.push(s.to_string());
                }
                graph.set_index(&nodes);
            }
    }
    return graph;
}
