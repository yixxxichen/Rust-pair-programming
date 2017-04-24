use std::io::{BufRead,BufReader,Read};

fn main() {

    //println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing input file"); //check training file
    }
    let f = File::open(&args[1]).expect("Error opening graph file!");
}

pub fn read_input<R: Read>(reader: R) -> Vec<String> {
    //let mut input: Vec<String> = Vec::new();  
    //let mut graph = Graph::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}
        if let Ok(f) = line.to_lowercase().parse() {
            let vertices:Vec<Char> = f.split_whitespace().collect();

        }        
    }
    return graph;
}