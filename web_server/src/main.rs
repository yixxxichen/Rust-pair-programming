extern crate time;
use std::io::{Write};
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc,Mutex};

mod request;

fn main() {
    // bind allows us to create a connection on the port
    // and gets it ready to accept connections.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    
    let log_f = File::create("log/log.txt"); 
    match log_f {
        Ok(_) => {
            println!("Create Log File!");  
            }      
        Err(_) => {
            panic!( "Can't create log file! Please create a folder /log");
        }
    }
    let unwrap_log = log_f.unwrap();
    let log_file = Arc::new(Mutex::new(unwrap_log));
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut logfile = log_file.clone();
                thread::spawn(move || {
                         get_response(&mut stream,&mut logfile);
                    });
         }
            Err(_) => { panic!("connection failed!") }
        }
}
}

fn write_to_log(logs: &mut Arc<Mutex<File>>, req:&Vec<String>, code:&str ) {
    
    let mut log_lock = logs.lock().unwrap();
    let url;
    if req.len()< 2 {
        url = "".to_string();
    }
    else {
        url  = req[1].to_string();
    }
    let timestamp = get_time();
    let all_log = format!("{} \"{}\" {}", timestamp, url, code);
    match log_lock.write_all(all_log.as_bytes()){
        Ok(_)   => {println!("Successful Log");}
        Err(_)  => {println!("Can't write into log");}
    }

}
//get current time 
fn get_time() -> String {
    let timespec = time::now();
    let res = time::strftime("%d/%m/%Y: %H:%M:%S",&timespec).unwrap();
    return res;
}

fn get_response(stream: &mut TcpStream, logs: &mut Arc<Mutex<File>>)  {
    let req = request::get_request(stream);
    match request::check_request(&req) {
        Ok(mut res) => {
            let write_res = res.write_response();
            let code = res.get_res_code();
            write_to_log(logs,&req,&code); 
            stream.write_all(write_res.as_bytes()).unwrap(); 
        }
        Err(mut e) => {
            let write_err = e.write_error();
            let code = write_err.clone();
            write_to_log(logs,&req,&code);
            stream.write_all(write_err.as_bytes()).unwrap();
        } 
    }
}
