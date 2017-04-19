//read from file and count words
use std::io::{BufRead,BufReader,Read,stdin};
//standard input and store the result in a string
pub fn read_input<R: Read>(reader: R) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        if let Ok(f) = line.to_lowercase().parse() {
            input.push(f); 
        }        
    }
    return input;
}
#[cfg(test)]
mod test_push_word{

    use super::read_input;
    use std::io::Cursor;
    //use std::collections::HashMap;
    #[test]
    fn read_one_line() {
        let input = "Hello world";
        assert_read(&["hello world"], input);
    }
    #[test]
    fn read_multi_lines() {
        let input = "Hello\nhello\nworld";
        assert_read(&["hello","hello","world"],input);
    }
    fn assert_read(expected: &[&str], input: &str) {
        let mock_read = Cursor::new(input);
        let words = read_input(mock_read);
        assert_eq!(expected.to_owned(), words);
    }
}
