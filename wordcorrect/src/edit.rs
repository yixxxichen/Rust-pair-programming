use std::collections::HashSet;
use std::iter::Iterator;

//static letters: &'static Vec<char> = &vec!['a','b','c'];

pub fn deletes(words_set: &mut HashSet<String>, input_word: String ) -> Vec<String> {
    let mut subword: String;
    let mut res: Vec<String> = Vec::new();
    let length = input_word.len();
    for i in 0..length {
        let l = (input_word[..i]).to_string();
        let r = &input_word[i+1..];
        subword = l + &r;
        words_set.insert(subword.clone());
        res.push(subword);
    }
    res
}
#[cfg(test)]
mod test_edit{
    use super::deletes;
    use super::transposes;
    use super::replaces;
    use super::inserts;
    use std::collections::HashSet;
    #[test]
    fn test_deletes() {
        let mut set: HashSet<String> = HashSet::new();
       // set.insert("ab");
        assert_eq!(deletes(&mut set, "ab".to_string()),["b", "a"]);
    }

    #[test]
    fn test_transposes_longlength() {
        let mut set: HashSet<String> = HashSet::new();
        assert_eq!(transposes(&mut set, "abcd".to_string()),["bacd", "acbd", "abdc"]);
    }
    #[test]
    fn test_transposes_shortlengt() {
        let mut set: HashSet<String> = HashSet::new();
        assert_eq!(transposes(&mut set, "ab".to_string()),["ba"]);
    }
    #[test]
    fn test_replaces_longlength() {
        let mut set: HashSet<String> = HashSet::new();
        assert_eq!(replaces(&mut set, "ab".to_string()),["ab", "bb", "cb", "aa", "ab", "ac"]);
    }
    #[test]
    fn test_inserts() {
        let mut set: HashSet<String> = HashSet::new();
        assert_eq!(inserts(&mut set, "ab".to_string()),["aab", "bab", "cab", "aab", "abb", "acb","aba" ,"abb", "abc"]);
    }

}

pub fn transposes(words_set: &mut HashSet<String>, input_word: String ) -> Vec<String>  {
    let mut subword: String;
    let mut res: Vec<String> = Vec::new();
    let length = input_word.len();
    if length == 0 {
        return res;
    }
    if length < 3 {
        subword = input_word.chars().rev().collect::<String>();
        res.push(subword);
        return res;
    }

    for i in 0..length - 1 {
        let l = (input_word[..i]).to_string();
        let r_0 = &(input_word[i..i+1]);
        let r_1 = &(input_word[i+1..i+2]); 
        let r = &input_word[i+2..];
        subword = l + r_1 + r_0 + &r;
        words_set.insert(subword.clone());
        res.push(subword);
    }
    res
}

pub fn replaces(words_set: &mut HashSet<String>, input_word: String) -> Vec<String> {
    //let letters = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let letters = ['a','b','c'];
    let mut subword: String;
    let mut res: Vec<String> = Vec::new();
    let length = input_word.len();
    if length < 2 {
        res.push(input_word);
        return res;
    }   
    for i in 0..length {
        for c in &letters{
            let l = (input_word[..i]).to_string();
            let r = &input_word[i+1..];
            subword = l + &c.to_string() + r;
            words_set.insert(subword.clone());
            res.push(subword); 
        }          
    }   
    res
}

pub fn inserts(words_set: &mut HashSet<String>, input_word: String) -> Vec<String> {
    let letters = ['a','b','c'];
    let mut subword: String;
    let mut res: Vec<String> = Vec::new();
    let length = input_word.len();
    for i in 0..length {
        for c in &letters{
            let l = (input_word[..i]).to_string();
            let r = &input_word[i..];
            subword = l + &c.to_string() + r;
            words_set.insert(subword.clone());
            res.push(subword);
        }
    }

    for c in &letters {
        subword = input_word.to_string() + &c.to_string();
        words_set.insert(subword.clone());
        res.push(subword);
    }
    res
}
