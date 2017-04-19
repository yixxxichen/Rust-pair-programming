//read from file and count words
use std::io::{BufRead,BufReader,Read,stdin};
use std::collections::HashMap;
//standard input and store the result in a string
pub fn read_input<R: Read>(reader: R) -> String {
    let mut input = String::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        if line == "999" {break}
        let space = "";
        let res = [space, &line].join("\n");
        input.push_str(&res);  
    }
    return input;
}

pub fn word_count(input: &str) -> HashMap<String, usize> {
    let mut dict: HashMap<String, usize> = HashMap::new();
    let lower = input.to_lowercase(); //turn the input string into lowercase
    let slice: &str = lower.as_ref();
    //split the input string and search the word in hashmap, get value and plus 1
    for word in slice.split(|c: char| !c.is_alphabetic()).filter(|s| !s.is_empty()) {
        *dict.entry(word.to_string()).or_insert(0)+=1;
    }
    dict
}

