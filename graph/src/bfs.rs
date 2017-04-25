use std::collections::HashMap;
use std::usize::MAX;
pub fn bfs(map : &HashMap<String,Vec<String>>,a :String, b :String)-> Vec<String>{
	let mut queue = map.get(&a).unwrap().clone();
	let length = map.capacity();
	let mut visit = HashMap::new();
	visit.insert(a.clone(),1);
	let mut path : HashMap<String, Vec<String>> = HashMap::new();
	let mut ans = Vec::new();
	let mut min:usize = MAX;
	ans.push(a);
	while !queue.is_empty() {
		let mut now = queue.remove(0);
		let mut currentpath = path.get(&now.clone()).unwrap().clone();
		currentpath.push(now.clone());
		if now == b {
			if min>currentpath.len(){
				min = currentpath.len();
				ans = currentpath.clone();
			}
		else {
			let mut node = map.get(&now).unwrap();
			for i in node {
				if !(visit.contains_key(&i.clone())){
					queue.push(i.clone());
					visit.insert(i.clone(),1);
					path.insert(i.clone(),currentpath.clone());
				}
			}
		}
		}
 		
	}
	ans
}

#[test]
fn test_bfs(){
	let mut map = HashMap::new();
	map.insert("a",vec!["b","d","c"]);
	map.insert("b",vec!["a","d"]);
	map.insert("c",vec!["d"]);
	map.insert("d",vec!["a","b","c"]);
	assert_eq!(bfs(map,"a","c"),vec!["a","b","d","c"]);
	}
}