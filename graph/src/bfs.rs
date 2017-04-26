use std::collections::HashMap;
use std::usize::MAX;
/*
* Input: map: the graph which is using hashmap to store the node information where keys are the node and the value is a vector consist of 
* neighbor node.
*  The visit is a hashmap which stores the node visited preventing the search stuck in a loop.
*/
pub fn bfs(map : &HashMap<String,Vec<String>>,a :String, b :String)-> Vec<String>{
	let mut queue = vec![a.clone()];
	let mut visit = HashMap::new();
	visit.insert(a.clone(),1);
	let mut path : HashMap<String, Vec<String>> = HashMap::new();
	path.insert(a.clone(),Vec::new());
	let mut ans = Vec::new();
	let mut min:usize = MAX;
	ans.push(a);
	while !queue.is_empty() {
		let now = queue.remove(0);
		let mut currentpath = path.get(&now.clone()).unwrap().clone();
		currentpath.push(now.clone());
		if now == b {
			if min>currentpath.len(){
				min = currentpath.len();
				ans = currentpath.clone();
			}
		}
		else {
			let node = map.get(&now.clone()).unwrap().clone();
			for i in node {
				if !(visit.contains_key(&i.clone())){
					queue.push(i.clone());
					visit.insert(i.clone(),1);
					path.insert(i.clone(),currentpath.clone());
				}
			}
		}
	}
	if min != MAX{
		return ans;
	}else {
		return Vec::new();
	}
	
}

#[test]
fn test_bfs(){
	let mut map = HashMap::new();
	map.insert("a".to_string(),vec!["b".to_string(),"d".to_string()]);
	map.insert("b".to_string(),vec!["a".to_string(),"d".to_string()]);
	map.insert("c".to_string(),vec!["d".to_string()]);
	map.insert("d".to_string(),vec!["a".to_string(),"b".to_string(),"c".to_string()]);
	map.insert("e".to_string(),vec!["e".to_string()]);
	assert_eq!(bfs(&map,"a".to_string(),"c".to_string()),vec!["a","d","c"]);
	assert_eq!(bfs(&map,"c".to_string(),"a".to_string()), vec!["c","d","a"]);
	assert_eq!(bfs(&map,"b".to_string(),"a".to_string()), vec!["b","a"]);
	assert_eq!(bfs(&map,"e".to_string(),"a".to_string()).is_empty(), true);
	}