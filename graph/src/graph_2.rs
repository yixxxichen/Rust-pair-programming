use std::collections::HashMap;
pub struct Graph {
    pub vertex: Vec<char>,
    pub matrix: Vec<Vec<usize>>,
}

impl Graph{
    pub fn new() -> Graph {
        Graph {
            vertex: Vec::new(),
            matrix: Vec::new(),
        }
    }
    pub fn set_index(&mut self, vertices: &Vec<char>,mut count:usize) {
        //let mut count = 0;
        for ch in vertices {
            if self.vertex. {
                unimplemented!();
            }
            if !self.vertex.contains_key(&ch) {
                self.vertex.insert(ch.clone(), count);
                count += 1;
            }
        }

    }
}

#[test]
fn set_index_work() {
    let mut t = Graph::new();
    let vector = vec!['a','b','c','d'];
    t.set_index(&vector, 0 as usize);
    assert_eq!(t.vertex.get(&'a'),Some(&0) );
    assert_eq!(t.vertex.get(&'d'),Some(&3) );
    assert_eq!(t.vertex.get(&'z'),None );
    //t.insert(&mut "bc".to_string(), 4);
}