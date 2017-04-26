// find path

// Read a graph from graph.dat. And select a start point and an end point,
// This program will find the shortest path and print out. If no path is found,
// it will print a message.

// Each line is a list of word, where the first word names some node 
// in the graph and the remaining words enumerate its neighbors.

// The user enters a start node and an end node on stdin, one at a time. 
// A query consists of two node names, a starting node and an ending node. 
// The program then prints out a path between the nodes, or a message - "No path found".

// INPUT & OUTPUT:
// $ cat graph.dat
//     a b b d
//     b d
//     c 
//     d


// $ cargo run graph.dat
// ->  a d
//     a d
// ->  a c
//     No path found.
// ->  a b c 
//     Input size is not 2
// ->  a z
//     START or END point not found
     
// ASSUMPTIONS:
// - The graph is an undirected graph. For node and neighbors: a b c,
// a is connected with b and with c.

// - Each node must take one line, no empty line is allowed in graph.dat, and 
// duplicated neighbors in one line are allowed.

// - Every node mentioned as a neighbor must start a line as well, 
// and no node may start more than one line. 

// - There must be one start node and one end node, if they are not in the graph,
// it will display a message. The input will only terminate with 999.

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
    graph.size();
    return graph;
    
}

pub fn find_path<R: Read, W:Write>(reader: R, writer: &mut W, graph: Graph )  {
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}
        let input_nodes: Vec<&str> = line.split_whitespace().collect();
        if input_nodes.len() != 2 {
            println!("Input size is not 2");
        }
        else {
            if graph.map.contains_key(&input_nodes[0].to_string()) && graph.map.contains_key(&input_nodes[1].to_string()) {
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
            else {
                println!("START or END point not found");
            }
        }
    }
}
