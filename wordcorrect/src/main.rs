
use std::io::{BufRead,BufReader,Read,stdin,Write,stdout};
use std::collections::HashSet;
fn main() {
    let mut possible_words: HashSet<String> = HashSet::new();
}
fn find(trie: & Trie, path: & mut String,pathclone: & mut String,cur: & mut String, op: usize)-> Option<Result>{
	if pathclone.len()==0 && trie.value>0{
		return Result{
			value: trie.value,
			key: cur.clone(),
		}
	else{
		let mut temppath = pathclone.clone();
        let mut currchar = temppath.remove(0);
        cur.push(currchar);
        let mut max = Result::new();
        let mut currtrie = trie.children.get(&curchar).unwrap();
        if currtrie {
        	max=find(currtrie,path,& mut temppath, cur, op);
        }
        else if op>0{
        	let mut temp = Result::new();
        	//insertion
        	for key in trie.children.keys(){
        		cur.push(*key);
        		currtrie = trie.children.get(&key).unwrap();
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
        	else{
        		temppath = pathclone.clone();
        		temppath.remove(0);
        		curchar = pathclone.remove(0);
        		cur.push(curchar);
        		currtrie = trie.children.get(&curchar).unwrap();
        		if currtrie{
        			temp = find(currtrie,path,pathclone,cur,op-1);
        			if temp.value>max.value{
        				max = temp;
        			}
        		}
        	//transpose
        	if path.len()>1{
        		temppath.pathclone.clone();
        		curchar = temppath.remove(0);
        		temppath.insert(1,curchar);
        		curchar = temppath.remove(0);
        		cur.push(curchar);
        		currtrie = trie.children.get(&curchar).unwrap();
        		if currtrie {
        			temp = find(currtrie,path,pathclone,cur,op-1);
        			if temp.value>max.value{
        				max = temp;
        			}
        		}
        		
        	}
        	}
        	}
        }
	}
	}
}
mod readinput;
mod edit;
mod trie;
#[test]
fn test_find_edit(){
	use super::{trie,Result};
    let mut t = Trie::new();
    t.insert(&mut "ac".to_string(), 4);
    t.insert(&mut "bc".to_string(), 5);
    assert_eq!(find(&mut "bc".to_string(),&mut "bc".to_string(),&mut "".to_string(),2).value, 5);
    assert_eq!(find(&mut "ac".to_string(),&mut "ac".to_string(),&mut "".to_string(),2).value, 4);
}