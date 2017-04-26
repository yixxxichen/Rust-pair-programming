// find path



use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
use graph::Graph;
use std::env;

use std::fs::File;
mod graph;
mod bfs;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing input file"); //check training file
    }
    let f = File::open(&args[1]).expect("Error opening graph file!");
    let mut graph = read_input(f);
    
    let newgraph = graph.change_map();
    find_path(stdin(),&mut stdout(),newgraph);

}
pub fn read_input<R: Read>(reader: R) -> Graph {
    let mut graph = Graph::new();

    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}
            let temp = line.clone();
            let vertices: Vec<&str> = temp.split_whitespace().collect();
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
        let input_nodes: Vec<&str> = line.split_whitespace().collect();
        if input_nodes.len() != 2 {
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
        break
    }

}
