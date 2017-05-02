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
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
    let mut log_f = File::create("log/log.txt"); 
    match log_f {
        Ok(_) => {
            println!("Create Log File!");  
            }      
        Err(_) => {
            panic!( "Can't create log file! Please create a folder /log");
        }
    }
    let unwrap_log = log_f.unwrap();
    let mut log_file = Arc::new(Mutex::new(unwrap_log));
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut logfile = log_file.clone();
                thread::spawn(move || {
                        let status = get_response(&mut stream,&mut logfile);
                    });
         }
            Err(e) => { panic!("connection failed!") }
        }
}
}

fn write_to_log(logs: &mut Arc<Mutex<File>>, req:&Vec<String>, code:&str ) {
    
    let mut log_lock = logs.lock().unwrap();
    let mut newcode = String::new();
    let mut all_log = String::new();
    let mut url = String::new();
    if req.len()< 2 {
        url = "".to_string();
    }
    else {
        url  = req[1].to_string();
    }
    let timestamp = get_time();
    all_log = format!("{} - {} - {}", timestamp, url, code);
    match log_lock.write_all(all_log.as_bytes()){
        Ok(_)   => {println!("Successful Log");}
        Err(_)  => {println!("Can't write into log");}
    }

}

//get current time 
fn get_time() -> String {
    let timespec = time::now();
    let res = time::strftime("%Y/%m/%d %H:%M:%S",&timespec).unwrap();
    return res;
}

fn get_response(stream: &mut TcpStream, logs: &mut Arc<Mutex<File>>)  {
    let req = request::get_request(stream);
    //let mut code_url:Vec<String> = vec![];  
    match request::check_request(&req) {
        Ok(mut res) => {
            let mut write_res = res.write_response();
            let code = res.get_res_code();
            let url = res.get_url();
            write_to_log(logs,&req,&code); 
            stream.write_all(write_res.as_bytes()).unwrap(); 
        }
        Err(mut e) => {
            let mut write_err = e.write_error();
            let code = write_err.clone();
            //let url = &req[1].to_string();
            write_to_log(logs,&req,&code);
            stream.write_all(write_err.as_bytes()).unwrap();
        } 
    }
}


fn test_file_path() {
    let mut full_path = String::new();
    // We assume that we are in a valid directory.
    let local_path = env::current_dir().unwrap();
    let local_path_string = local_path.display().to_string();
    full_path.push_str(&local_path_string);
    let mypath = "/src/main.txt";
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
        println!("{}","NO SUCH PATH/FILE");
    }
}
