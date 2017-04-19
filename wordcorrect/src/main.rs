
//#![allow(dead_code)]
//#![allow(unused_variables)]
use std::io::{stdin};
//use std::collections::HashSet;
use std::env;
use std::collections::HashMap;
use trie::Trie;
use trie::Result;
use std::fs::File;
mod readinput;
mod edit;
mod trie;
mod counter;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing input file");
    }
    let f = File::open(&args[1]).expect("Error opening training file!");
    let dict: HashMap<String, usize> = counter::word_count(&counter::read_input(f));
    let check_words: Vec<String> = readinput::read_input(stdin());
    let mut t:Trie = Trie::new();
    for (key,value) in &dict {
        t.insert(&mut key.to_string(), *value);
    }

    for word in check_words {
        if t.fetch(&mut word.to_string()) != 0 { //change here
            println!("{}, {}", word, word);
        }
    }
}

fn find(trie: & Trie, path: & String,pathclone: & mut String,cur: & mut String, op: &mut usize)-> Result{
	if pathclone.len()==0 && trie.value>0 {
		return Result{
			value: trie.value,
			key: cur.clone(),
		}
    }
	else{
        let mut max = Result::new();
        let mut temp = Result::new();
        let mut temppath = pathclone.clone();
        let mut curchar : char='a';
        let mut currtrie = &Trie::new();
        if pathclone.len()>0{
        
        let mut curchar = temppath.remove(0);
        cur.push(curchar);
        
        if let Some(currtrie) = trie.children.get(&curchar) {
        	max = find(currtrie,path,& mut temppath, cur, op);
            if *op==2 && *cur == max.key{
                return max;
             }
            }
        cur.pop();
        }
        

         if *op> 0{

        	//insertion
        	for key in trie.children.keys(){
        		cur.push(*key);
        		currtrie = trie.children.get(&key).unwrap();
                let mut counter = *op-1;
        		temp = find(&currtrie,path,pathclone,cur,&mut counter);
        		if temp.value>max.value{
        			max = temp;
        		}
                cur.pop();
        	}
        	//deletion
        	//if we can get a word after deleting current character
            if pathclone.len()>0{
        	   if pathclone.len()==1 && trie.value>0{
            	return Result{
        		  	value: trie.value,
        	   		key: cur.clone(),
        	   	   }
                }
                if pathclone.len()==2 &&*op==2&& trie.value>0{
                    return Result{
                        value: trie.value,
                        key: cur.clone(),
                   }
                 }
        		temppath = pathclone.clone();
        		curchar = pathclone.remove(0);
        		cur.push(curchar);
        		if let Some(currtrie) = trie.children.get(&curchar){
                    let mut counter = *op-1;
        			temp = find(currtrie,path,&mut temppath,cur,&mut counter);
        			if temp.value>max.value{
        				max = temp;
        			}
        		}
                cur.pop();
            
        	//transpose
        	   // if pathclone.len()!=1{
        	   // 	    temppath = pathclone.clone();
        		  //   curchar = temppath.remove(0);
        	   // 	    temppath.insert(1,curchar);
        		  //   curchar = temppath.remove(0);
        		  //   cur.push(curchar);
        		  //   if let Some(currtrie) = trie.children.get(&curchar) {
            //             let mut counter = *op-1;
        			 //     temp = find(currtrie,path,&mut temppath,cur,&mut counter);
        			 //     if temp.value>max.value{
        				//     max = temp;
        			 //     }
        		  //     }
            //         cur.pop();
        		
        	   //     }
                //replace
                // for key in trie.children.keys(){
                //     temppath = pathclone.clone();
                //     if temppath.len()>1{temppath.remove(0);}
                //     else{temppath="".to_string();}
                //     cur.push(*key);
                //     currtrie = trie.children.get(&key).unwrap();
                //     let mut counter = *op-1;
                //     temp = find(&currtrie,path,&mut temppath,cur,&mut counter);
                //     if temp.value>max.value{
                //         max = temp;
                //          }
                //     cur.pop();
                //     }
            
                 }
            }
        return max;
    }
   
}
    
        	
 
#[test]
fn test_find_edit_value(){
    //use super::{trie,Result};
    let mut t = Trie::new();
    t.insert(&mut "acd".to_string(), 4);
    t.insert(&mut "bce".to_string(), 5);
    t.insert(&mut "cbdca".to_string(),3);
    t.insert(&mut "gg".to_string(),100);
    assert_eq!(find(&t, &mut "bce".to_string(),&mut "bce".to_string(),&mut "".to_string(),&mut 2).value, 5);
    assert_eq!(find(&t, &mut "acd".to_string(),&mut "acd".to_string(),&mut "".to_string(),&mut 2).value, 4);
    assert_eq!(find(&t, &mut "acd".to_string(),&mut "acd".to_string(),&mut "".to_string(),&mut 2).key, "acd");
     assert_eq!(find(&t, &mut "".to_string(),&mut "".to_string(),&mut "".to_string(),&mut 2).key, "gg");
    assert_eq!(find(&t, &mut "cbdca".to_string(),&mut "cbdca".to_string(),&mut "".to_string(),&mut 2).value, 3);
}
#[test]
fn test_find_replace_value(){
    let mut t = Trie::new();
    t.insert(&mut "acd".to_string(), 4);
    t.insert(&mut "bce".to_string(), 5);
    t.insert(&mut "cbdca".to_string(),3);
    t.insert(&mut "gg".to_string(),100);
    assert_eq!(find(&t, &mut "bed".to_string(),&mut "bed".to_string(),&mut "".to_string(),&mut 2).value, 5);
    assert_eq!(find(&t, &mut "b".to_string(),&mut "b".to_string(),&mut "".to_string(),&mut 2).value, 100);
}
#[test]
fn test_find_delete_value(){
    let mut t = Trie::new();
    t.insert(&mut "acd".to_string(), 4);
    t.insert(&mut "bce".to_string(), 5);
    t.insert(&mut "cbdca".to_string(),3);
    t.insert(&mut "gg".to_string(),100);
    assert_eq!(find(&t, &mut "bcdea".to_string(),&mut "bed".to_string(),&mut "".to_string(),&mut 2).value, 5);
    assert_eq!(find(&t, &mut "ggag".to_string(),&mut "b".to_string(),&mut "".to_string(),&mut 2).value, 100);
}