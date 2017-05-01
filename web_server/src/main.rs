extern crate time;
use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
use std::env;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc,Mutex};

mod request;

fn main() {
    // bind allows us to create a connection on the port
    // and gets it ready to accept connections.
    test_file_path();
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut log = File::create("log/log.txt");
    match log {
        Ok(_) => {
            let log_file = Arc::new(Mutex::new(log));
            println!("Create Log File!");  
            }      
        Err(_) => {
            panic!( "Can't create log file!");
        }
    }
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                        get_response(&mut stream);
                    });
         }
            Err(e) => { panic!("connection failed!") }
        }
}
}

// fn write_to_log(logs: &mut Arc<Mutex<File>>, req:&Vec<String>, code: &str ) {
//     if req.len() < 3 {
//         return Err(Error::ERROR400);
//     }
//     let check_GET = &req[0];
//     let get_path  = &req[1];
//     let protocol  = &req[2];


// }

//get current time 
fn get_time() -> String {
    let timespec = time::now();
    let res = time::strftime("%Y/%m/%d %H:%M:%S",&timespec).unwrap();
    return res;
}

fn get_response(stream: &mut TcpStream)  {
    let req = request::read_stream(stream);
    match request::check_request(&req) {
        Ok(mut res) => {
            let mut write_res = res.write_response();
            println!("{}",write_res);
            stream.write_all(write_res.as_bytes());          
        }
        Err(mut e) => {
            let mut write_err = e.write_error();
            stream.write_all(write_err.as_bytes());
        } 
    }
}


fn test_file_path() {
    let mut full_path = String::new();
    // We assume that we are in a valid directory.
    let local_path = env::current_dir().unwrap();
    let local_path_string = local_path.display().to_string();
    full_path.push_str(&local_path_string);
    let mypath = "/src";
    full_path.push_str(mypath);
    let path = Path::new(&full_path);
    if path.exists() {
        //try to open file
        let mut meta = match fs::metadata(&path) {
            Ok(m) => {
                let file_type = m.file_type();
                if file_type.is_file() {
                    panic!("FILE FOUND");
                }
                else {
                    panic!("WRONG");
                }
            }
            Err(_) => panic!("NOT FOUND"),
        };
    }
    else {
        println!("{}","NO PATH");
    }
}
