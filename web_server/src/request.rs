use std::io::{BufRead,BufReader,stdin,Read, Write,stdout};
use std::env;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Response {
	protocal: String,
	status_code: String,
	server_name: String,
    file_url: String,
	file_type: String,
	file_length: usize,
	file_content: String,
}

impl Response {
    pub fn write_response(&mut self) -> String {
        let res = format!("{} {} {}\nContent-type: {}\nContent-length: {}\n\n{}\n",
        self.protocal,self.status_code,self.server_name,self.file_type,self.file_length,self.file_content);
        return res;
    }
    pub fn get_res_code(&mut self) -> String {
        let code = self.status_code.to_owned();
        return code;
    }

    pub fn get_url(&mut self) -> String {
        return self.file_url.to_owned();
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
            Error::ERROR400 => return "400 Bad Request\n".to_string(),
            Error::ERROR403 => return "403 Forbidden\n".to_string(),
            Error::ERROR404 => return "404 Not Found\n".to_string(), 
        }
    }
    pub fn get_error_code(&mut self) -> String {
        match *self {
            Error::ERROR400 => return "400".to_string(),
            Error::ERROR403 => return "403".to_string(),
            Error::ERROR404 => return "404".to_string(), 
        }
    }
}
pub enum Good {
    OK200,   
}
//Get stream content and return in Vec<String>
pub fn get_request(stream: &mut TcpStream) -> Vec<String> {
    let mut reader = BufReader::new(stream).lines();
    let mut input = String::new();
    let mut res: Vec<String> = Vec::new();
    while let Some(Ok(line)) = reader.next(){
        let oneline = line.clone();
        //put requests into a vector
        let lines: Vec<&str> = line.split_whitespace().collect();
        for s in &lines{
            res.push(s.to_string());
        }
        //check the end of input
        let byte = oneline.as_bytes();
        if byte.len()<256 {
            break
        }
    }
    return res;
}
// check the statuses of responses
// • 200 OK, which starts a reply that serves the specified file;
// • 400 Bad Request, which indicates that the command is not a properly formatted GET command;
// • 403 Forbidden, which rejects a command because it specifies a file that is off-limits; and
// • 404 Not Found, which informs the client that the specified file does not exist.
pub fn check_request(req: &Vec<String>) -> Result<Response,Error> {
    //check request format: - ERR400
    //GET
    //valid path
    //protocol (HTTP)
    if req.len() < 3 {
        return Err(Error::ERROR400);
    }
    let check_GET = &req[0];
    let get_path  = &req[1];
    let protocol  = &req[2];
    if check_GET == "GET" && protocol.contains("HTTP") {
        //check file exists - ERR404
        let mut full_path = String::new();
        // We assume that we are in a valid directory.
        let local_path = env::current_dir().unwrap();
        let local_path_string = local_path.display().to_string();
        full_path.push_str(&local_path_string);
        full_path.push_str(&get_path);
        let file_path = Path::new(&full_path);
        if file_path.exists() {
            match fs::metadata(&file_path) {
            Ok(meta) => {
                let file_type = meta.file_type();
                //check if path is a dir, find index files in this path
                if file_type.is_dir() {
                    let names = vec!["/index.html","/index.shtml","/index.txt"];
                    return check_file(&names, req, &full_path);
                }
                else {
                    //check if path is a file
                   if file_type.is_file(){
                    match File::open(&file_path){
                        Ok(get_file) => {
                            return Ok(create_response(req, &full_path));
                        }
                        Err(_) => {return Err(Error::ERROR403);}
                    }
                   }
                   //if not return ERROR 404
                   else {
                       return Err(Error::ERROR404);
                   }
                }
            }
            Err(_) => return Err(Error::ERROR404),
        }
        }
        else {
            // path does't exist
            return Err(Error::ERROR404);
        }
    }
    else {
        // wrong format
        return Err(Error::ERROR400);
    }
}

fn check_file(names: &Vec<&str>, req: &Vec<String>, path :&str) -> Result<Response,Error> {
    let mut full_path = path;
    for s in names{
        let mut file_path = full_path.clone().to_string();
        file_path.push_str(&s);
        ///check if file exist
        let full_file_path = Path::new(&file_path);
        if full_file_path.exists() {
            //try to open file
            match File::open(&full_file_path){
                Ok(_) => return Ok(create_response(req, &file_path)),
                Err(_) => return Err(Error::ERROR403),
            }
        }
    }
    //if no file found, return error 404
    return Err(Error::ERROR404);
}

pub fn create_response(req: &Vec<String>, path :&str) -> Response{
    let check_GET = &req[0];
    let get_path = &path;
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
        status_code : "200 OK\n".to_string(),
        server_name : "web-server/0.1".to_string(),
        file_url    : get_path.to_string(),
        file_type   : content_type,
        file_length : f.read_to_string(&mut buf).unwrap(),
        file_content: buf,
    }

}
