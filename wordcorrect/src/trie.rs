
use std::collections::HashMap;
use std::hash::Hash;
use std::string::String;
use std::clone::Clone;

<<<<<<< HEAD
struct Trie {
    value: usize,
    children: HashMap<char, Trie>,
=======
pub struct Trie {
    pub value: usize,
    pub children: HashMap<char, Trie>,
>>>>>>> origin/master
}
struct Result{
    pub value: usize,
    pub key: String,
}
impl Result{
<<<<<<< HEAD
    fn new() -> Result {
=======
    pub fn new() -> Result {
>>>>>>> origin/master
        Result {
            value: 0,
            key: String::new(),
        }
    }
}
<<<<<<< HEAD
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
=======
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
>>>>>>> origin/master
impl Trie{
    fn new() -> Trie {
        Trie {
            value: 0,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, path: &mut String, v: usize) {
        if path.is_empty() {
            self.value = v;
            return;
        }

        self.children.entry(path.remove(0))
            .or_insert(Trie::new())
            .insert(path, v)
    }

    fn fetch(&self, path: &mut String) -> usize {
        match path.len() {
            0 => self.value,
            _ => self.children.get(&path.remove(0))
                    .unwrap()
                    .fetch(path)
        }
    }
<<<<<<< HEAD
    fn find_edit(&self, path: & mut String,pathclone: & mut String,cur: & mut String, op: usize)-> Option<Result>{
        match path.len() {
            0 =>Some(Result{
                        value :self.value,
                        key : cur.clone(),
                        }
                    ),
            _ => {
                        let mut temppath = pathclone.clone();
                        let mut curchar = temppath.remove(0);
                        cur.push(curchar);
                     let mut max = self.children.get(&curchar).unwrap().find_edit(path,& mut temppath,cur,op).unwrap();
                        if op>0 {
                        let mut temp = Result::new();
                        //insertion 
                            for key in self.children.keys(){
                                cur.push(*key);
                                temp = self.children.get(&key)
                                .unwrap()
                                .find_edit(path,pathclone,cur,op-1).unwrap().clone();
                                if temp.value > max.value{
                                    max=temp;

                                    }
                            };
                            //deletion
                            if path.len()>=1{
                            temppath = pathclone.clone();
                            temppath.remove(0);
                            curchar = pathclone.remove(0);
                            cur.push(curchar);
                            temp = self.children.get(&curchar).unwrap().find_edit(path,& mut temppath,cur,op-1).unwrap().clone();
                            if temp.value > max.value{
                                    max=temp;

                                    }
                                }
                            //transpose
                            if path.len()>=2{
                            temppath = pathclone.clone();
                            curchar = temppath.remove(0);
                            temppath.insert(1,curchar);
                            curchar = temppath.remove(0);
                            cur.push(curchar);
                            temp = self.children.get(&curchar).unwrap().find_edit(path,& mut temppath,cur,op-1).unwrap().clone();
                            if temp.value > max.value{
                                    max=temp;
                                    }
                                }
                            //replace
                            if path.len()>0{
                            for key in self.children.keys(){
                                temppath = pathclone.clone();
                                temppath.remove(0);
                                cur.push(*key);
                                temp = self.children.get(&key)
                                .unwrap()
                                .find_edit(path,& mut temppath,cur,op-1).unwrap().clone();
                                if temp.value > max.value{
                                    max=temp;
                                    }
                                }
                            }
                                    
                            }
                            return Some(max);
                

            }
        }
    }
=======
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
>>>>>>> origin/master
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
#[test]
fn test_find_edit(){
    let mut t = Trie::new();
    t.insert(&mut "ac".to_string(), 4);
    t.insert(&mut "bc".to_string(), 5);
    assert_eq!(t.find_edit(&mut "bc".to_string(),&mut "bc".to_string(),&mut "".to_string(),2).unwrap().value, 5);
    assert_eq!(t.find_edit(&mut "ac".to_string(),&mut "ac".to_string(),&mut "".to_string(),2).unwrap().value, 4);
}