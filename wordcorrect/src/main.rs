
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
        if t.fetch(&mut word.to_string()) != 0 {
            println!("{}, {}", word, word);
        }
    }

}
fn find(trie: & Trie, path: & mut String,pathclone: & mut String,cur: & mut String, op: usize)-> Result{
	if pathclone.len()==0 && trie.value>0{
		return Result{
			value: trie.value,
			key: cur.clone(),
		}
    }
	else{
		let mut temppath = pathclone.clone();
        let mut curchar = temppath.remove(0);
        cur.push(curchar);
        let mut max = Result::new();
        let mut currtrie = trie.children.get(&curchar).expect("first error");
        if !currtrie.children.is_empty() {
        	max = find(currtrie,path,& mut temppath, cur, op);
            
        }
        else if op>0{
        	let mut temp = Result::new();
        	//insertion
        	for key in trie.children.keys(){
        		cur.push(*key);
        		currtrie = trie.children.get(&key).expect("insert error");
        		temp = find(currtrie,path,pathclone,cur,op-1);
        		if temp.value>max.value{
        			max = temp;
        		}
        	}
        	//deletion
        	//if we can get a word after deleting current character
        	if pathclone.len()==1 && trie.value>0{
        		return Result{
        			value: trie.value,
        			key: cur.clone(),
        		}
            }
        	else{
        		temppath = pathclone.clone();
        		temppath.remove(0);
        		curchar = pathclone.remove(0);
        		cur.push(curchar);
        		currtrie = trie.children.get(&curchar).expect("delete error");
        		if !currtrie.children.is_empty(){
        			temp = find(currtrie,path,pathclone,cur,op-1);
        			if temp.value>max.value{
        				max = temp;
        			}
        		}
            }
        	//transpose
        	if path.len()>1{
        		temppath = pathclone.clone();
        		curchar = temppath.remove(0);
        		temppath.insert(1,curchar);
        		curchar = temppath.remove(0);
        		cur.push(curchar);
        		currtrie = trie.children.get(&curchar).expect("transpose error");
        		if !currtrie.children.is_empty() {
        			temp = find(currtrie,path,pathclone,cur,op-1);
        			if temp.value>max.value{
        				max = temp;
        			}
        		}
        		
        	}
                //replace
            for key in trie.children.keys(){
            temppath = pathclone.clone();
            temppath.remove(0);
            cur.push(*key);
            currtrie = trie.children.get(&key).expect("replace error");
            if !currtrie.children.is_empty(){
            temp = find(currtrie,path,pathclone,cur,op-1);
            if temp.value>max.value{
                max = temp;
            }
            }
            
            }
            
        }
       return max; 
    }
}
    
        	
 
#[test]
fn test_find_edit(){
    //use super::{trie,Result};
    let mut t = Trie::new();
    t.insert(&mut "ac".to_string(), 4);
    t.insert(&mut "bc".to_string(), 5);
    assert_eq!(find(&t, &mut "bc".to_string(),&mut "bc".to_string(),&mut "".to_string(),2).value, 5);
    assert_eq!(find(&t, &mut "ac".to_string(),&mut "ac".to_string(),&mut "".to_string(),2).value, 4);
}
