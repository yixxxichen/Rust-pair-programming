use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
use std::env;
use std::fs::File;
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use std::thread;
// pub fn handle_stream(stream: TcpStream) -> Vec<String> {
//     let mut reader = BufReader::new(stream);
//     let mut input = String::new();
//     while let Some(Ok(line)) = reader.next(){
//         // if let Ok(f) = line.parse(){
//         //     input.push_str(&f);
//         // }
//         let Ok(f) = line.parse();
//         input.push_str(&f);
//     }
//     println!("{}", input);
//     let res: Vec<String> = &input.split_whitespace().collect();
//     return res;
// }

pub fn handle_stream(stream: TcpStream) {

    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines() {
        if line.unwrap() == "" {
            break;
        }
    }

    send_response(reader.into_inner());
}
// New function to write back with!
pub fn send_response(mut stream: TcpStream) {
    // Write the header and the html body
    let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}