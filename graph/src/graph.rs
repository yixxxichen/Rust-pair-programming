use std::collections::HashMap;
pub struct Graph {
    pub map: HashMap<String,Vec<String>>
}

impl Graph{
    pub fn new() -> Graph {
        Graph {
            map: HashMap::new(),
        }
    }
    pub fn set_index(&mut self, vertices: &Vec<String>) {
            if !self.map.contains_key(&vertices[0].to_string()) {
                self.map.insert(vertices[0].to_string(), vertices[1..].to_vec());
            }
        }
    pub fn 
}

#[test]
fn set_index_work() {
    let mut t = Graph::new();
    let vector = vec!["aa".to_string(),"bb".to_string(),"cc".to_string(),"d".to_string()];
    let res = vec!["bb".to_string(),"cc".to_string(),"d".to_string()];
    t.set_index(&vector.to_vec());
    assert_eq!(t.map.get(&"aa".to_string()),Some(&res) );
    //assert_eq!(t.vertex.get(&'d'),Some(&3) );
    assert_eq!(t.map.get(&"zz".to_string()),None );
    //t.insert(&mut "bc".to_string(), 4);
}