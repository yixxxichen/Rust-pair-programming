use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Eq, PartialEq, Debug, Clone)]
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
    pub fn add_node(&mut self, node: String, new_node: String){
        let mut new_neighbor: Vec<String> = vec![];
        let mut check = 1;
        let mut empty = 0;
        match self.map.get(&node.to_string()){
            None => {
                println!("missing node");                              
            }
            Some(n) => {
                if n.is_empty() {
                    new_neighbor.push(new_node);
                    empty = 1;
                }     
                else {
                      for i in 0..n.len(){
                        let value = &n[i].to_string();               
                        if !value.eq(&new_node.to_string()) {
                            check = 0;
                        }
                        else {
                            check = 1;
                            break
                        }                      
                    }
                    if check == 0{
                        new_neighbor = n.clone();
                        new_neighbor.push(new_node.to_string());
                    }
                }           
            }
        }
        if empty == 1 && check == 1 {
            self.map.insert(node.to_string(), new_neighbor.to_vec());
        }
        if empty == 0 && check == 0 {
            self.map.insert(node.to_string(),new_neighbor.to_vec());
        }
    }
    //pub fn clone()
    //change_map() is to add neighbors to nodes 
    //ex:
    //  before:
    //      a, b c
    //      b, c
    //      c
    //  after:
    //      a, b c
    //      b, c a
    //      c, a b
    pub fn change_map(&mut self) -> Graph{
        let mut newmap = self.clone();
        
        for (vertex,neighbors) in self.map.iter() {
            for s in neighbors {
                newmap.add_node(s.to_string(),vertex.to_string());          
            }
        }
        newmap
    }
    pub fn size(&mut self) -> usize {
        let mut hash:HashSet<&str> = HashSet::new();
        for (vertex,neighbors) in self.map.iter(){
            for s in neighbors {
               hash.insert(s);
            }
            hash.insert(vertex);
        }
        let input_length = hash.len();
        let map_length = self.map.len();
        if input_length == map_length {
            input_length
        }else {
            panic!("wrong number of nodes");
        }
    }
}
    
#[test]
fn set_index_work() {
    let mut t = Graph::new();
    let vector = vec!["aa".to_string(),"bb".to_string(),"cc".to_string(),"d".to_string()];
    let res = vec!["bb".to_string(),"cc".to_string(),"d".to_string()];
    t.set_index(&vector.to_vec());
    assert_eq!(t.map.get(&"aa".to_string()),Some(&res) );
    assert_eq!(t.map.get(&"zz".to_string()),None );

}

#[test]
fn test_add_node() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
    let vector_b = vec!["b".to_string(),"c".to_string()];
    let vector_c = vec!["c".to_string()];
    t.set_index(&vector_a.to_vec());
    t.set_index(&vector_b.to_vec());
    t.set_index(&vector_c.to_vec());
    let res = vec!["b".to_string(),"c".to_string(),"d".to_string()];
    t.add_node("a".to_string(),"d".to_string());
    assert_eq!(t.map.get(&"a".to_string()),Some(&res));
}

#[test]
fn test_add_node_toempty() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string()];
    t.set_index(&vector_a.to_vec());
    let res = vec!["c".to_string()];
    t.add_node("a".to_string(),"c".to_string());
    assert_eq!(t.map.get(&"a".to_string()),Some(&res));
}

#[test]
fn test_add_node_nochange() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string()];
    t.set_index(&vector_a.to_vec());
    let res = vec!["b".to_string()];
    t.add_node("a".to_string(),"b".to_string());
    assert_eq!(t.map.get(&"a".to_string()),Some(&res));
}

#[test]
fn change_map_change_value() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string(),"d".to_string()];
    let vector_b = vec!["b".to_string()];
    let vector_c = vec!["c".to_string()];
    let vector_d = vec!["d".to_string()];
    let res_a = vec!["b".to_string(),"c".to_string(),"d".to_string()];
    let res_b = vec!["a".to_string()];
    let res_c = vec!["a".to_string()];
    let res_d = vec!["a".to_string()];
    t.set_index(&vector_a.to_vec());
    t.set_index(&vector_b.to_vec());
    t.set_index(&vector_c.to_vec());
    t.set_index(&vector_d.to_vec());
    let newt = t.change_map();
    assert_eq!(newt.map.get(&"a".to_string()),Some(&res_a));
    assert_eq!(newt.map.get(&"b".to_string()),Some(&res_b));
    assert_eq!(newt.map.get(&"c".to_string()),Some(&res_c));
    assert_eq!(newt.map.get(&"d".to_string()),Some(&res_d));
}

#[test]
fn change_map_change_one(){
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
    let vector_b = vec!["b".to_string()];
    let res_b = vec!["a".to_string()];
    t.set_index(&vector_a.to_vec());
    t.set_index(&vector_b.to_vec());
    let newt = t.change_map();
    assert_eq!(newt.map.get(&"b".to_string()),Some(&res_b) );
}

#[test]
fn change_map_not_change_value() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
    let vector_b = vec!["b".to_string(),"c".to_string(),"a".to_string()];
    let vector_c = vec!["c".to_string(),"a".to_string(),"b".to_string()];
    let res_a = vec!["b".to_string(),"c".to_string()];
    let res_b = vec!["c".to_string(),"a".to_string()];
    let res_c = vec!["a".to_string(),"b".to_string()];
    t.set_index(&vector_a.to_vec());
    t.set_index(&vector_b.to_vec());
    t.set_index(&vector_c.to_vec());
    let newt = t.change_map();
    assert_eq!(newt.map.get(&"a".to_string()),Some(&res_a) );
    assert_eq!(newt.map.get(&"b".to_string()),Some(&res_b) );
    assert_eq!(newt.map.get(&"c".to_string()),Some(&res_c) );

}

// #[test]
// fn test_size_wrong() {
//     let mut t = Graph::new();
//     let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
//     let vector_b = vec!["b".to_string(),"c".to_string(),"a".to_string()];
//     t.set_index(&vector_a.to_vec());
//     t.set_index(&vector_b.to_vec());
//     let mut newt = t.change_map();
//     assert_eq!(newt.size(),2);
// }

#[test]
fn test_size_ok() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
    let vector_b = vec!["b".to_string(),"c".to_string(),"a".to_string()];
    let vector_c = vec!["c".to_string()];
    t.set_index(&vector_a.to_vec());
    t.set_index(&vector_b.to_vec());
    t.set_index(&vector_c.to_vec());
    let mut newt = t.change_map();
    assert_eq!(newt.size(),3);
}
