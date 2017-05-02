use std::io::{BufRead,BufReader,Read};
use std::env;
use std::fs::File;
use std::path::Path;
use std::net::TcpStream;

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
        let res = format!("{} {}{}\nContent-type: {}\nContent-length: {}\n\n{}\n",
        self.protocal,self.status_code,self.server_name,self.file_type,self.file_length,self.file_content);
        return res;
    }
    pub fn get_res_code(&mut self) -> String {
        let code = self.status_code.to_owned();
        return code;
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
}

//Get stream content and return in Vec<String>
pub fn get_request(stream: &mut TcpStream) -> Vec<String> {
    let mut reader = BufReader::new(stream).lines();
    let mut res: Vec<String> = Vec::new();
    while let Some(Ok(line)) = reader.next(){
        //put requests into a vector
        let lines: Vec<&str> = line.split_whitespace().collect();
        for s in &lines{
            res.push(s.to_string());
        }
        //only read one line
        break
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
    let check_get = &req[0];
    let get_path  = &req[1];
    let protocol  = &req[2];
    if check_get == "GET" && protocol.starts_with("HTTP") {
        //check file exists - ERR404
        let mut full_path = String::new();
        // We assume that we are in a valid directory.
        let local_path = env::current_dir().unwrap();
        let local_path_string = local_path.display().to_string();
        full_path.push_str(&local_path_string);
        full_path.push_str(&get_path);
        let file_path = Path::new(&full_path);
        if file_path.exists() {
            //check if path is a dir, find index files in this path
            if file_path.is_dir() {
                let names = vec!["/index.html","/index.shtml","/index.txt"];
                return check_file(&names, req, &full_path);
            }
            else {
                //check if path is a file
                if file_path.is_file(){
                match File::open(&file_path){
                    Ok(_) => {
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
    let full_path = path;
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
//Get the file
pub fn create_response(req: &Vec<String>, path :&str) -> Response{
    let get_path = &path;
    let protocal_info  = &req[2];
    let file_path = Path::new(&get_path);
    let mut buf = String::new();
    // open file
    let mut f = File::open(&file_path).unwrap();
    let mut content_type = "text/plain".to_string();
    if get_path.contains(".html") {
         content_type = "text/html".to_string();
    }

    Response{
        protocal    : protocal_info.to_string(),
        status_code : "200 OK\n".to_string(),
        server_name : "web-server/0.1".to_string(),
        file_type   : content_type,
        file_length : f.read_to_string(&mut buf).unwrap(),
        file_content: buf,
    }
}
#[cfg(test)]
mod test{
    // use super::Response;
    use super::Error;
    use request::check_file;
    use request::check_request;
    #[test]
    fn test_error_write_error(){
        assert_eq!("400 Bad Request\n".to_string(), Error::ERROR400.write_error());
        assert_eq!("403 Forbidden\n".to_string(), Error::ERROR403.write_error());
        assert_eq!("404 Not Found\n".to_string(), Error::ERROR404.write_error());
    }
    #[test]
    fn test_error_get_error(){
        assert_eq!("400".to_string(), Error::ERROR400.get_error_code());
        assert_eq!("403".to_string(), Error::ERROR403.get_error_code());
        assert_eq!("404".to_string(), Error::ERROR404.get_error_code());
    }
    #[test]
    fn test_check_request_404(){
        let temp = vec!["GET".to_string(),"/src/wrong.rs".to_string(),"HTTP".to_string()];
        match check_request(&temp) {
            Ok(mut res) =>{
                assert_eq!(res.get_res_code(),"404".to_string());
            }
            Err(mut e) =>{
                assert_eq!(e.get_error_code(),"404".to_string());
            }
        }
    }
    #[test]
    fn test_check_request_400(){
        let temp = vec!["SET".to_string(),"/src/wrong.rs".to_string(),"HTTP".to_string()];
        match check_request(&temp) {
            Ok(mut res) =>{
                assert_eq!(res.get_res_code(),"400".to_string());
            }
            Err(mut e) =>{
                assert_eq!(e.get_error_code(),"400".to_string());
            }
        }
    }
    #[test]
    fn test_check_request_200(){
        let temp = vec!["GET".to_string(),"/src/main.rs".to_string(),"HTTP".to_string()];
        match check_request(&temp) {
            Ok(mut res) =>{
                assert_eq!(res.get_res_code(),"200 OK\n".to_string());
            }
            Err(mut e) =>{
                assert_eq!(e.get_error_code(),"404".to_string());
            }
        }
    }
    #[test]
    fn test_check_file_404(){
        let names = vec!["/index.html","/index.shtml","/index.txt"];
        let temp = vec!["GET".to_string(),"/src/main.rs".to_string(),"HTTP".to_string()];
        let path = "/src/main";
        match check_file(&names,&temp,&path) {
            Ok(mut res) => {
                assert_eq!(res.get_res_code(),"404".to_string());
            }
            Err(mut e) => {
                assert_eq!(e.get_error_code(),"404".to_string());
            },
        }
    }
    #[test]
    fn test_check_file_200(){
        let names = vec!["/index.html","/index.shtml","/index.txt"];
        let temp = vec!["GET".to_string(),"/src/main.rs".to_string(),"HTTP".to_string()];
        let path = "/src/main.rs";
        match check_file(&names,&temp,&path) {
            Ok(mut res) => {
                assert_eq!(res.get_res_code(),"200".to_string());
            }
            Err(mut e) => {
                assert_eq!(e.get_error_code(),"404".to_string());
            },
        }
    }
}
