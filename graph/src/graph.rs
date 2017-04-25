use std::collections::HashMap;
pub struct Graph {
    pub vertex: HashMap<String,usize>,
    pub matrix: Vec<Vec<usize>>,
}

impl Graph{
    pub fn new() -> Graph {
        Graph {
            vertex: HashMap::new(),
            matrix: Vec::new(),
        }
    }
    pub fn set_index(&mut self, vertices: &Vec<&str>,mut count:usize) {
        //let mut count = 0;
        for s in vertices {
            if !self.vertex.contains_key(&s.to_string()) {
                self.vertex.insert(s.to_string(), count);
                count += 1;
            }
        }

    }
}

#[test]
fn set_index_work() {
    let mut t = Graph::new();
    let vector = vec!["aa","bb","cc","d"];
    t.set_index(&vector, 0 as usize);
    assert_eq!(t.vertex.get(&"aa".to_string()),Some(&0) );
    //assert_eq!(t.vertex.get(&'d'),Some(&3) );
    assert_eq!(t.vertex.get(&"zz".to_string()),None );
    //t.insert(&mut "bc".to_string(), 4);
}