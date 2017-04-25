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

// pub fn set_index(index_map: &HashMap<String,usize>, vertices: &Vec<&str>,mut count:usize) {
//     //let mut count = 0;
//     for s in vertices {
//         if !index_map.contains_key(&s.to_string()) {
//             index_map.insert(s.to_string(), count);
//             count += 1;
//         }
//     }

// }
// #[test]
// fn set_index_work() {
//     let mut t: HashMap<String,usize> = HashMap::new();
//     let vector = vec!["aa","bb","cc","d"];
//     set_index(&t,&vector, 0 as usize);
//     assert_eq!(t.get(&"aa".to_string()),Some(&0) );
//     //assert_eq!(t.vertex.get(&'d'),Some(&3) );
//     assert_eq!(t.get(&"zz".to_string()),None );
//     //t.insert(&mut "bc".to_string(), 4);
// }