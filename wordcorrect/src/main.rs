
#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::collections::HashSet;
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
    let mut f = File::open(&args[1]).expect("Error opening training file!");
    let mut dict: HashMap<String, usize> = HashMap::new();
    let mut check_words: Vec<String> = Vec::new();
    let mut t:Trie = Trie::new();
    dict = counter::word_count(&counter::read_input(f));

    for (key,value) in &dict {
        t.insert(&mut key.to_string(), *value);
    }

    check_words = readinput::read_input(stdin());
    //println!("{}",t.fetch(&mut "hello".to_string()));
    for word in check_words {
        if t.fetch(&mut word.to_string()) != 0 {
            println!("{}, {}", word, word);
        }
    }

}

