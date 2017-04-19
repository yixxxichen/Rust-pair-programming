
use std::collections::HashMap;
use std::hash::Hash;
use std::string::String;
use std::clone;

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
// impl Clone for Result{
//     fn clone(&self) ->Self{
//         let mut result =Result::new();
//         {
//             result.value = self.value;
//             result.key = self.key.clone();
//         }
//         result
//     }
// }
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
    // fn find_edit(&self, path: &mut String,cur: &mut String, op: usize)-> Option<&mut Result>{
    //     match path.len() {
    //         0 => Some(&mut Result{
    //                     value :self.value,
    //                     key : *cur,
    //                     }),
    //         _ => Some({
    //             match self.children.get(&path.clone().chars().next().unwrap()) {
    //                 Some(trie) => Some({
    //                     let curchar = path.remove(0);
    //                     cur.push(curchar);
    //                  return   self.children.get(&curchar).unwrap()
    //                 .find_edit(path,cur,op);}),
    //                 None => Some(if op>0{
    //                     let mut max = Result
    //                     {
    //                         value:0,
    //                         key:"".to_string(),
    //                     };
    //                     let mut temp = Result{
    //                         value:0,
    //                         key:"".to_string(),
    //                     };
    //                     //insertion 
    //                         for key in self.children.keys(){
    //                             cur.push(*key);
    //                             temp = *self.children.get(&key)
    //                             .unwrap()
    //                             .find_edit(path,cur,op-1).unwrap();
    //                             if temp.value > max.value{
    //                                 max=temp;
    //                                 }
    //                         };
    //                         //deletion
    //                         let pathclone = path.clone();
    //                         pathclone.remove(0);
    //                         let curchar = pathclone.remove(0);
    //                         cur.push(curchar);
    //                         let mut temp = *self.children.get(&curchar).unwrap().find_edit(path,cur,op-1).unwrap();
    //                         if temp.value > max.value{
    //                                 max=temp;

    //                                 }
    //                         //transpose
    //                         pathclone = path.clone();
    //                         curchar = pathclone.remove(0);
    //                         pathclone.insert(1,curchar);
    //                         curchar = pathclone.remove(0);
    //                         cur.push(curchar);
    //                         temp = *self.children.get(&curchar).unwrap().find_edit(path,cur,op-1).unwrap();
    //                         if temp.value > max.value{
    //                                 max=temp;
    //                                 }
    //                         //replace
    //                         for key in self.children.keys(){
    //                             path.remove(0);
    //                             cur.push(*key);
    //                             temp = *self.children.get(&key)
    //                             .unwrap()
    //                             .find_edit(path,cur,op-1).unwrap();
    //                             if temp.value > max.value{
    //                                 max=temp;
    //                                 }
    //                                 return Some(&mut max);
    //                         }
    //                     })
    //             }
    //         })
    //     }
    // }
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
// #[test]
// fn find_edit_not_found(: ) -> RetType {
//     unimplemented!();
// }