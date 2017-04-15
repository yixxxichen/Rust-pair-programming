use std::collections::HashSet;

fn deletes(words_set: &mut HashSet<String>, input_word: String ) -> Vec<String> {
    let subword: String;
    let mut res: Vec<String> = Vec::new();
    let length = input_word.len();
    for i in 0..length {
        subword = &input_word[..i].to_string() + &input_word[i+1..].to_string();
        words_set.insert(subword.clone());
        res.push(subword);
    }
    res
}
#[cfg(test)]
mod test_edit{
    use super::deletes;
    use std::collections::HashSet;
    #[test]
    fn test_deletes() {
        let mut set: HashSet<String> = HashSet::new();
       // set.insert("ab");
        assert_eq!(deletes(&set, "ab"),["a", "b"]);
    }

}
