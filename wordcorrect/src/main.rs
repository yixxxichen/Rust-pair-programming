
#![allow(dead_code)]
//#![allow(unused_variables)]
use std::io::{stdin};
//use std::collections::HashSet;
use std::env;
use std::collections::HashMap;
use trie::Trie;
use std::fs::File;
mod readinput;
mod edit;
mod trie;
mod counter;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing input file");
    }
    let f = File::open(&args[1]).expect("Error opening training file!");
    let dict: HashMap<String, usize> = counter::word_count(&counter::read_input(f));
    let check_words: Vec<String> = readinput::read_input(stdin());
    let mut t:Trie = Trie::new();
    for (key,value) in &dict {
        t.insert(&mut key.to_string(), *value);
    }

    for word in check_words {
        if t.fetch(&mut word.to_string()) != 0 {
            println!("{}, {}", word, word);
        }
    }

}

