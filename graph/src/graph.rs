#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
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
    //pub fn clone()
    //change_map() is to add neighbors to nodes 
    //ex:
    //  before:
    //      a, b c
    //      b, c
    //  after:
    //      a, b c
    //      b, c a
    pub fn change_map(&mut self) -> Graph{
        let mut newmap = self.clone();
        for (vertex,neighbors) in self.map.iter() {
            for s in neighbors {
                    //get neighbors of one in s
                    if let Some(mut n) = self.map.get(&s.to_string()) {
                        for i in 0..n.len(){
                            let mut value = &n[i].to_string();
                              for x in n.iter() {
                                    println!("elements in n: {} ",x);
                                }
                            //println!("{}",&value);
                            //println!("{}",);                            
                            if !value.eq(vertex){
                                println!("vertex {}:", vertex);
                                let mut new_vec = n.clone();
                                new_vec.push(vertex.to_string());
                                
                                 for x in new_vec.iter() {
                                    println!("elements in new_vec: {} ",x);
                                }
                                newmap.map.insert(s.to_string(),new_vec.to_vec());
                                //println!("{}",s);
                              
                                
                                //break
                            }                            
                        }
                    }                       
            }
        }
        newmap
    }
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

#[test]
fn change_map_work() {
    let mut t = Graph::new();
    let vector = vec!["a".to_string(),"b".to_string(),"c".to_string()];
    let vector_2 = vec!["b".to_string(),"c".to_string()];
    let res = vec!["b".to_string(),"c".to_string()];
    let res_2 = vec!["c".to_string(),"a".to_string()];
    //let res = vec!["bb".to_string(),"cc".to_string(),"d".to_string()];
    t.set_index(&vector.to_vec());
    t.set_index(&vector_2.to_vec());
    let newt = t.change_map();
    assert_eq!(newt.map.get(&"a".to_string()),Some(&res) );
    assert_eq!(newt.map.get(&"b".to_string()),Some(&res_2) );
}