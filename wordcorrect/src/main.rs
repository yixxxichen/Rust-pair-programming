
use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::collections::HashSet;
use std::env;
use std::collections::HashMap;
mod readinput;
mod edit;
//mod trie;
mod counter;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing input file");
    }
    let mut dict: HashMap<String, usize> = HashMap::new();
    let mut check_words: Vec<String> = Vec::new();
    dict = counter::word_count(&args[1]);
    check_words = readinput::read_input(stdin());

}

