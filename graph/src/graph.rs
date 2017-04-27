// graph
// Graph uses a hashmap as its stucture. Its key is a string of a node,
// and its values are a vector including all neighbors.

// Function - new()
// Create a new Graph

// Function - build_graph(vertices: &Vec<String>)

// This function helps to build the graph. 
// The input is a vector of strings. First element of the vector is the key(node) 
// and rest elements are the value(neighbors) of Graph. 

// Function - add_neighbor(node: String, new_node: String)
// Adding a neighbor to a node. Do not add if the neighbor is already exists.

// Function - change_map()
// This function is some kind of confusing. Since the graph is undirected, and 
// sometimes the input file doesn't inlude all two paths between two nodes.

// For example:
//     before:
//         a, b c
//         b, c
//         c

//     For node 'b', although 'a' and 'b' are connected, it didn't include 'a' as 'b's neighbor,
// so we add 'a' to 'b's neighbor.
//     It is similar for 'c'. The initial graph only shows two paths a->c and b->c, so we add
// c->a and c->b.

//     after:
//         a, b c
//         b, c a
//         c, a b

// Function - check_size()
// This original name "size()" is misleading, so I change it to check_size.
// This function checks if all nodes in the neighbors are listed，and it will panic if not.
// Actually we don't need to specificlly define the "size()" function. The size of map can be found by
// "Graph.map.len()"

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
    pub fn build_graph(&mut self, vertices: &Vec<String>) {
            if !self.map.contains_key(&vertices[0].to_string()) {
                self.map.insert(vertices[0].to_string(), vertices[1..].to_vec());
            }
        }
    pub fn add_neighbor(&mut self, node: String, new_node: String){
        let mut new_neighbor: Vec<String> = vec![];
        let mut check = 1;
        let mut empty = 0;
        match self.map.get(&node.to_string()){
            None => {
                                           
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

    pub fn change_map(&mut self) -> Graph{
        let mut newmap = self.clone();
        
        for (vertex,neighbors) in self.map.iter() {
            for s in neighbors {
                newmap.add_neighbor(s.to_string(),vertex.to_string());          
            }
        }
        newmap
    }
    pub fn check_size(&mut self) -> bool {
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
            true
        }else {
            panic!("Wrong number of nodes");
        }
    }
}
    
#[test]
fn build_graph_work() {
    let mut t = Graph::new();
    let vector = vec!["aa".to_string(),"bb".to_string(),"cc".to_string(),"d".to_string()];
    let res = vec!["bb".to_string(),"cc".to_string(),"d".to_string()];
    t.build_graph(&vector.to_vec());
    assert_eq!(t.map.get(&"aa".to_string()),Some(&res) );
    assert_eq!(t.map.get(&"zz".to_string()),None );

}

#[test]
fn test_add_neighbor() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
    let vector_b = vec!["b".to_string(),"c".to_string()];
    let vector_c = vec!["c".to_string()];
    t.build_graph(&vector_a.to_vec());
    t.build_graph(&vector_b.to_vec());
    t.build_graph(&vector_c.to_vec());
    let res = vec!["b".to_string(),"c".to_string(),"d".to_string()];
    t.add_neighbor("a".to_string(),"d".to_string());
    assert_eq!(t.map.get(&"a".to_string()),Some(&res));
}

#[test]
fn test_add_neighbor_toempty() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string()];
    t.build_graph(&vector_a.to_vec());
    let res = vec!["c".to_string()];
    t.add_neighbor("a".to_string(),"c".to_string());
    assert_eq!(t.map.get(&"a".to_string()),Some(&res));
}

#[test]
fn test_add_neighbor_nochange() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string()];
    t.build_graph(&vector_a.to_vec());
    let res = vec!["b".to_string()];
    t.add_neighbor("a".to_string(),"b".to_string());
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
    t.build_graph(&vector_a.to_vec());
    t.build_graph(&vector_b.to_vec());
    t.build_graph(&vector_c.to_vec());
    t.build_graph(&vector_d.to_vec());
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
    t.build_graph(&vector_a.to_vec());
    t.build_graph(&vector_b.to_vec());
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
    t.build_graph(&vector_a.to_vec());
    t.build_graph(&vector_b.to_vec());
    t.build_graph(&vector_c.to_vec());
    let newt = t.change_map();
    assert_eq!(newt.map.get(&"a".to_string()),Some(&res_a) );
    assert_eq!(newt.map.get(&"b".to_string()),Some(&res_b) );
    assert_eq!(newt.map.get(&"c".to_string()),Some(&res_c) );

}

// #[test]
// fn test_check_size_wrong() {
//     let mut t = Graph::new();
//     let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
//     let vector_b = vec!["b".to_string(),"c".to_string(),"a".to_string()];
//     t.build_graph(&vector_a.to_vec());
//     t.build_graph(&vector_b.to_vec());
//     let mut newt = t.change_map();
//     assert_eq!(newt.check_size(),2);
// }

#[test]
fn test_check_size_ok() {
    let mut t = Graph::new();
    let vector_a = vec!["a".to_string(),"b".to_string(),"c".to_string()];
    let vector_b = vec!["b".to_string(),"c".to_string(),"a".to_string()];
    let vector_c = vec!["c".to_string()];
    t.build_graph(&vector_a.to_vec());
    t.build_graph(&vector_b.to_vec());
    t.build_graph(&vector_c.to_vec());
    let mut newt = t.change_map();
    assert_eq!(newt.check_size(),true);
}
