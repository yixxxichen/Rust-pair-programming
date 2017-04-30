use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
use std::env;
use std::fs::File;
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Response {
	status_info: String,
	status_code: String,
	server_name: String,
	file_type: String,
	file_length: usize,
	file_content: String,
}

pub enum Error {
	
	ERROR400,
	ERROR403,
	ERROR404,
}

#[derive(Debug)]
pub enum Good {
    OK200,
    
}

//Get stream content and return in Vec<String>
pub fn handle_stream(stream: TcpStream) -> Vec<String> {
    let mut reader = BufReader::new(stream).lines();
    let mut input = String::new();
    let mut res: Vec<String> = Vec::new();
    while let Some(Ok(line)) = reader.next(){
        //println!("{}",line);
        let lines: Vec<&str> = line.split_whitespace().collect();
        //let mut temp = line.split_whitespace();
        for s in &lines{
            res.push(s.to_string());
            }       
    }
    return res;
}
// check the statuses of responses
// • 200 OK, which starts a reply that serves the specified file;
// • 400 Bad Request, which indicates that the command is not a properly formatted GET command;
// • 403 Forbidden, which rejects a command because it specifies a file that is off-limits; and
// • 404 Not Found, which informs the client that the specified file does not exist.
pub fn check_request(req: &Vec<&String>) -> Result<Good,Error> {
    //check request format: - ERR400
    //GET
    //PATH
    //protocol
    if req.len() < 3 {
        return Err(Error::ERROR400);
    }
    let check_GET = req[0];
    let get_path = req[1];
    let protocol  = req[2];
    if check_GET == "GET" && protocol.contains("HTTP") {
        //check file exists - ERR404
        //let path_string = file_path;
        let file_path = Path::new(get_path);
        if file_path.exists() {
            //let get_file = File::open(&file_path);
            match File::open(&file_path) {
                Ok(some_file) => return Ok(Good::OK200),
                Err(_) => return Err(Error::ERROR403),
            }
        }
        else {
            return Err(Error::ERROR403)
        }
    }
    else {
        return Err(Error::ERROR400);
    }
}


// New function to write back with!
pub fn send_response(mut stream: TcpStream) {
    // Write the header and the html body
    let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}