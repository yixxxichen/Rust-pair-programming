use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
use std::env;
use std::fs::File;
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Response {
	protocal: String,
	status_code: String,
	server_name: String,
	file_type: String,
	file_length: usize,
	file_content: String,
}

impl Response {
    pub fn write_response(&mut self) -> String {
        // let pro = self.protocal.to_string();
        // let sta = self.status_code.to_string();
        // let ser = self.server_name.to_string();
        // let typ = self.file_type.to_string();
        // let leng = self.file_length;
        // let cont = self.file_content.to_string();
        let mut res = String::new();
        let leng = self.file_length.clone().to_string();
        res.push_str(&self.protocal);
        res.push_str(&self.status_code);
        res.push_str(&self.server_name);
        res.push_str(&self.file_type);
        res.push_str(&leng);
        res.push_str(&self.file_content);
        //res = self.protocal + &self.status_code.clone() +&self.server_name.clone() + &self.file_type.clone() + &leng.clone() + &self.file_content.clone();
        return res;
    }
}

pub enum Error {
	ERROR400,
	ERROR403,
	ERROR404,
}
impl Error {
    pub fn write_error(&mut self) -> String {
        match *self {
            Error::ERROR400 => return "400 Bad Request".to_string(),
            Error::ERROR403 => return "403 Forbidden".to_string(),
            Error::ERROR404 => return "404 Not Found".to_string(), 
        }
    }
}

pub enum Good {
    OK200,
    
}

//Get stream content and return in Vec<String>
pub fn handle_stream(stream: &mut TcpStream) -> Vec<String> {
    let mut reader = BufReader::new(stream).lines();
    let mut input = String::new();
    let mut res: Vec<String> = Vec::new();
    while let Some(Ok(line)) = reader.next(){
        if line == "999" {
            break
        }
        if line == "\n".to_owned() || line == "\r\n".to_owned(){   
            break						
        }
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
pub fn check_request(req: &Vec<String>) -> Result<Good,Error> {
    //check request format: - ERR400
    //GET
    //PATH
    //protocol
    if req.len() < 3 {
        return Err(Error::ERROR400);
    }
    let check_GET = &req[0];
    let get_path  = &req[1];
    let protocol  = &req[2];
    if check_GET == "GET" && protocol.contains("HTTP") {
        //check file exists - ERR404
        //let path_string = file_path;
        let file_path = Path::new(&get_path);
        if file_path.exists() {
            //let get_file = File::open(&file_path);
            match File::open(&file_path) {
                Ok(some_file) => return Ok(Good::OK200),
                Err(_) => return Err(Error::ERROR403),
            }
        }
        else {
            return Err(Error::ERROR404)
        }
    }
    else {
        return Err(Error::ERROR400);
    }
}

pub fn create_response(req: &Vec<String>) -> Response{
    let check_GET = &req[0];
    let get_path = &req[1];
    let protocal_info  = &req[2];
    let file_path = Path::new(&get_path);
    let mut buf = String::new();
    let mut f = File::open(&file_path).unwrap();
    let mut content_type = String::new();
    if get_path.contains("html") {
        content_type = "text/html".to_string();
    }
    else {
        content_type = "text/plain".to_string();
    }

    Response{
        protocal    : protocal_info.to_string(),
        status_code : "200".to_string(),
        server_name : "gan-yixi-web-server/0.1".to_string(),
        file_type   : content_type,
        file_length : f.read_to_string(&mut buf).unwrap(),
        file_content: buf,
    }

}
// New function to write back with!
pub fn send_response(mut stream: TcpStream) {
    // Write the header and the html body
    let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, World!</body></html>";
    stream.write_all(response.as_bytes()).unwrap();
}