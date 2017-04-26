#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(unused_mut)]
#![warn(unused_imports)]
use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
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
    let mut graph = Graph::new();
    graph = read_input(f);
    let newgraph = graph.change_map();
    find_path(stdin(),&mut stdout(),newgraph);
    // assert_eq!(newgraph.map.get(&"a".to_string()),Some(&res_a));
    // assert_eq!(newgraph.map.get(&"b".to_string()),Some(&res_b));
    
}
pub fn read_input<R: Read>(reader: R) -> Graph {
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
                panic!("Found empty line");
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

pub fn find_path<R: Read, W:Write>(reader: R, writer: &mut W, graph: Graph )  {
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        let mut input_nodes: Vec<&str> = line.split_whitespace().collect();
        if !input_nodes.len() == 2 {
            panic!("wrong start or end node");
        }
        else {
            let res = bfs::bfs(&graph.map,input_nodes[0].to_string(),input_nodes[1].to_string());
            if res.len() == 0 {
                write!(writer, "No path found.\n").unwrap();
            }
            else {
                for c in res{
                    write!(writer, "{} ",c.clone()).unwrap();
                }
                write!(writer, "\n").unwrap();
            }
        }
    }

}

// #[cfg(test)]
// mod test_find_path{
// 	use super::find_path;
// 	use std::io::Cursor;
//     //use super::word_sort;
//     use std::collections::HashMap;
// 	#[test]
// 	fn output_result(){
//         let mut map: HashMap<String, usize> = HashMap::new();
//         map.insert("hello".to_string(),1);
//         map.insert("world".to_string(),3);
//         map.insert("test".to_string(),4);
//         let count = word_sort(&map);
// 		assert_write("test: 4\nworld: 3\nhello: 1\n", count);
// 		}

//     fn assert_write(expected: &str, result: Vec<(&String,&usize)>) {
//         let mut writer = Cursor::new(vec![]);
//         find_path(&mut writer, result);
//         assert_eq!(expected.as_bytes(), &*writer.into_inner());
//     }
// }
