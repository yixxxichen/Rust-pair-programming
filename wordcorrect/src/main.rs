/*
* word correction
*
* Reads text from the corpus text and 
* Count the frequency of each word, then correct the candidate word in input text, output with the most frequetly
* used one
* 
* Background
* 
* The purpose of correct is to find possible corrections for misspelled words. It consists of two phases: 
* The first phase is a training module, which consumes a corpus of correctly spelled words and counts the 
* number of occurrences of each word. The second phase uses the results of the first to check individual words. 
* Specifically, it checks whether each word is spelled correctly according to the training module and, if not, 
* whether “small edits” can reach a variant that is correctly spelled.
* 
* Given a word, an edit action is one of the following:
*   the deletion of one letter;
*
*   the transposition of two neighboring letters;
*
*   the replacement of one letter with another letter; and
*
*   the insertion of a letter at any position.
*
*   In this context, Norvig suggests that “small edits” means the application of one edit action possibly 
*   followed by the application of a second one to the result of the first.
*   Once the second part has generated all possible candidate for a potentially misspelled word, 
*   it picks the most frequently used one from the training corpus. If none of the candidates is a correct word, 
*   correct reports a failure.
*
* INPUT
*
* The input format is using two standard input consuming text. It could contain anything like words, numbers or some marks.
* writtten in ASCII.
* 
*    Hello world! Where are you now? 
*    www333
*    github.com/rust
*    !!!!!@@@@@@@
*
* Any non-alphabetic will be regarded as noise and will not be counted:
*
*    23232
*    ++--!!!@@
*    ...???''''""""
*
*
* The input terminates with end-of-file.
*
*
* OUTPUT
*
* The correct program consumes a training file on the command line and then reads words—one per line—from 
* standard input. For each word from standard in, correct prints one line. The line consists of just the word 
* if it is spelled correctly. If the word is not correctly spelled, correct prints the word and the best 
* improvement or “-” if there aren’t any improvements found.
*
*  hello
*
*  hell, hello
*
*  word
*
*  wordl, world
*
*  wor, world
*
*  wo, word
*
*  w, -
*
* ASSUMPTIONS
*
*  - Words are reading according to the language's reading routines,
*
*  - A word contained numbers will be count only the alphabetic character
*    and ignore the numbers.
*
*  - All the symbol, space and numbers will be considered as noise and ignored. 
*
*  - The input only terminate in the end-of-file, which is kind of unconvenient
*    if you want to use console to input your data.
*
*  - Once the word has been edited, we would pick the most frequently used one after
*    two editions.
* 
*  - Except fot the normal edition, we add the corner case handler to accelerate the algorithm
*
*  
*/
//#![allow(dead_code)]
//#![allow(unused_variables)]
use std::io::{stdin};
use std::env;
use std::collections::HashMap;
use trie::Trie;
use trie::Result;
use std::fs::File;
mod readinput;
mod trie;
mod counter;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing input file"); //check training file
    }
    let f = File::open(&args[1]).expect("Error opening training file!");
    //calculate the freq. of training file and stored in a hashmap
    let dict: HashMap<String, usize> = counter::word_count(&counter::read_input(f)); 
    //read input.txt and store in a vector
    let check_words: Vec<String> = readinput::read_input(stdin());
    //initialize the Trie
    let mut t:Trie = Trie::new();
    for (key,value) in &dict {
        t.insert(&mut key.to_string(), *value);
    }
    //check each word in input.txt
    for word in check_words {
        let mut changeword = word.clone();
        let mut changeword2 = word.clone();
        
        let temp = find(&t, &mut changeword,&mut changeword2,&mut "".to_string(), 2).key;
        if temp.is_empty(){
            println!("{}, -", word);
        }

        else if temp == word{
            println!("{}" , word);
        }

        else {
             println!("{}, {}", word, temp);
        }
    }
}
/*
* This is the main search function for this program. We implement the DFS(Depth-First-Search)+Regression 
* Algorithm to travel the whole trie and find all the candidate word, then pick the most frequently one.
* 
* Input : trie: The trie made from corpus.txt, contains all the word
*         path: The misspelled word
*         pathclone: The remained string need to match
*         cur: The trie path
*         op: The edit times left for current matching
*
* The stop condition is that current trie path consist a word and match the input/edited word.
* 
* We separate each character in the string and find the matched node in current trie. In the meantime,
* we also edit the word in case we can't find the original word in tries. But we set the original search
* with the highest priority.
* 
* Output is a Struct with key: String the most frequently used word
*                       value: maximum frequency.
*/
fn find(trie: & Trie, path: & String,pathclone: & mut String,cur: & mut String, op: i64)-> Result{
	if pathclone.len()==0 && trie.value>0 {
		return Result{
			value: trie.value,
			key: cur.clone(),
		}
    }
	else{
        let mut max= Result::new();
        let mut temp: Result;
        let mut temppath =pathclone.clone();
        let mut currtrie :&Trie;
        if pathclone.len()>0{
        
            let mut curchar = temppath.remove(0);
        
            if let Some(currtrie) = trie.children.get(&curchar) {
                cur.push(curchar);
        	   max = find(currtrie,path,& mut temppath, cur, op);
                if op==2 && max.key == *path{
                    return max;
                    }
                cur.pop();
                }
            //deletion
            //if we can get a word after deleting current character
            if op > 0{
                if pathclone.len()==1 && trie.value>0{
                temp= Result{
                    value: trie.value,
                    key: cur.clone(),
                   };
                   if temp.value>max.value{
                        max = temp;
                    }
                }
                if pathclone.len()==2 &&op==2&& trie.value>0{
                    temp= Result{
                        value: trie.value,
                        key: cur.clone(),
                   };
                   if temp.value>max.value{
                        max = temp;
                    }
                 }
                 if pathclone.len()>1{
                temppath = pathclone.clone();
                temppath.remove(0);
                curchar = temppath.remove(0);
                if let Some(currtrie) = trie.children.get(&curchar){
                    let counter = op-1;
                    cur.push(curchar);
                    temp = find(currtrie,path,&mut temppath,cur,counter);
                    if temp.value>max.value{
                        max = temp;
                    }
                    cur.pop();
                    if pathclone.len()>2 &&op==2{
                        temppath = pathclone.clone();
                        temppath.remove(0);
                        temppath.remove(0);
                        curchar = temppath.remove(0);
                        if let Some(currtrie) = trie.children.get(&curchar){
                            let counter = 0;
                            cur.push(curchar);
                            temp = find(currtrie,path,&mut temppath,cur,counter);
                            if temp.value>max.value{
                                max = temp;
                                }
                        cur.pop();
                        }
                    }
                }
            }
                //transpose
                if pathclone.len()>1{
                    temppath = pathclone.clone();
                    curchar = temppath.remove(0);
                    temppath.insert(1,curchar);
                    curchar = temppath.remove(0);
                    cur.push(curchar);
                    if let Some(currtrie) = trie.children.get(&curchar) {
                        let  counter = op-1;
                         temp = find(currtrie,path,&mut temppath,cur,counter);
                         if temp.value>max.value{
                            max = temp;
                         }
                      }
                    cur.pop();
                
                   }
               // replace
                for key in trie.children.keys(){
                    temppath = pathclone.clone();
                    if temppath.len()>1{
                        temppath.remove(0);
                    }
                    else{temppath="".to_string();}
                    currtrie = trie.children.get(&key).unwrap();
                    cur.push(*key);
                    let counter = op-1;
                    temp = find(&currtrie,path,&mut temppath,cur,counter);
                    if temp.value>max.value{
                        max = temp;
                         }
                         cur.pop();
                       }
            
                 }
            }        
        if op> 0{

        	//insertion
        	for key in trie.children.keys(){
        		cur.push(*key);
        		currtrie = trie.children.get(&key).unwrap();
                let counter = op-1;
        		temp = find(&currtrie,path,pathclone,cur,counter);
        		if temp.value>max.value{
        			max = temp;
        		}
                cur.pop();
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
    assert_eq!(find(&t, &mut "bce".to_string(),&mut "bce".to_string(),&mut "".to_string(), 2).value, 5);
    assert_eq!(find(&t, &mut "acd".to_string(),&mut "acd".to_string(),&mut "".to_string(), 2).value, 4);
    assert_eq!(find(&t, &mut "acd".to_string(),&mut "acd".to_string(),&mut "".to_string(), 2).key, "acd");
     assert_eq!(find(&t, &mut "".to_string(),&mut "".to_string(),&mut "".to_string(), 2).key, "gg");
    assert_eq!(find(&t, &mut "cbdca".to_string(),&mut "cbdca".to_string(),&mut "".to_string(), 2).value, 3);
}
#[test]
fn test_find_replace_value(){
    let mut t = Trie::new();
    t.insert(&mut "acd".to_string(), 4);
    t.insert(&mut "bce".to_string(), 5);
    t.insert(&mut "cbdca".to_string(),3);
    t.insert(&mut "gg".to_string(),100);
    assert_eq!(find(&t, &mut "bed".to_string(),&mut "bed".to_string(),&mut "".to_string(), 2).value, 5);
    assert_eq!(find(&t, &mut "b".to_string(),&mut "b".to_string(),&mut "".to_string(), 2).value, 100);
}
#[test]
fn test_find_delete_value(){
    let mut t = Trie::new();
    t.insert(&mut "acd".to_string(), 4);
    t.insert(&mut "bce".to_string(), 5);
    t.insert(&mut "cbdca".to_string(),3);
    t.insert(&mut "gg".to_string(),100);
    assert_eq!(find(&t, &mut "bcdea".to_string(),&mut "bed".to_string(),&mut "".to_string(), 2).value, 5);
    assert_eq!(find(&t, &mut "ggag".to_string(),&mut "b".to_string(),&mut "".to_string(), 2).value, 100);
}