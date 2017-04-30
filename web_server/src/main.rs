use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
use std::env;
use std::fs::File;
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use std::thread;
mod request;
//use iron::prelude::*;
//extern crate iron;
fn main() {
    // bind allows us to create a connection on the port
    // and gets it ready to accept connections.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let stream = listener.accept().unwrap().0;
    handle_request(stream);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                        handle_request(stream);

                    });
                println!("success")
            }
            Err(e) => { println!("connection failed!") }
        }
}

}

fn handle_request(stream: TcpStream)  {
    let request = request::handle_stream(stream);
    for i in 0..3{
        println!("{}",request[i]);
    }
}
