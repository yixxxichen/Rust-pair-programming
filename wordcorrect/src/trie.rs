
use std::collections::HashMap;
use std::hash::Hash;
use std::string::String;
use std::clone::Clone;

pub struct Trie {
    pub value: usize,
    pub children: HashMap<char, Trie>,
}
pub struct Result{
    pub value: usize,
    pub key: String,
}
impl Result{
    pub fn new() -> Result {
        Result {
            value: 0,
            key: String::new(),
        }
    }
}
impl Clone for Result{
    fn clone(&self) ->Self{
        let mut result =Result::new();
        {
            result.value = self.value;
            result.key = self.key.clone();
        }
        result
    }
}
impl Trie{
    pub fn new() -> Trie {
        Trie {
            value: 0,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: &mut String, v: usize) {
        if path.is_empty() {
            self.value = v;
            return;
        }

        self.children.entry(path.remove(0))
            .or_insert(Trie::new())
            .insert(path, v)
    }

    pub fn fetch(&self, path: &mut String) -> usize {
        match path.len() {
            0 => self.value,
            _ => self.children.get(&path.remove(0))
                    .unwrap()
                    .fetch(path)
        }
    }

   
}


#[test]
fn fetch_works() {
    let mut t = Trie::new();
    t.insert(&mut "vec1".to_string(), 3);
    let f = t.fetch(&mut "vec1".to_string());
    assert_eq!(f, 3);
}

#[test]
fn insert_panics_if_exists() {
    let mut t = Trie::new();
    t.insert(&mut "abc".to_string(), 3);
    t.insert(&mut "bc".to_string(), 4);
}

#[test]
fn insert_works_if_none() {
    let mut t = Trie::new();
    t.insert(&mut "ac".to_string(), 4);
    t.insert(&mut "bc".to_string(), 5);

    assert_eq!(t.fetch(&mut "a".to_string()), 0);
    assert_eq!(t.fetch(&mut "bc".to_string()), 5);
    assert_eq!(t.fetch(&mut "ac".to_string()), 4);
}
