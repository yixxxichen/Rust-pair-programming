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
    //handle_request(stream);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                        get_response(&mut stream);

                    });
                println!("success")
            }
            Err(e) => { println!("connection failed!") }
        }
}

}

fn get_response(stream: &mut TcpStream)  {
    let req = request::handle_stream(stream);
    match request::check_request(&req) {
        Ok(_) => {
            let mut create_req = request::create_response(&req);
            let write_res = create_req.write_response();
            &stream.write(write_res.as_bytes());          
        }
        Err(e) => {} 
        //&stream.write(e.to_string().as_bytes().unwrap())
    }
}
