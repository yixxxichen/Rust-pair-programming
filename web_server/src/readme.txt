web_server
This is a rudimentary web server, which can respond to the single command of HTTP 0.9 GET method.
In return to a valid GET request, the web server spawns a thread that retrieves the request, 
records it to a log file, and generates a response. 

INPUT:
    A HTTP request

OUTPUT:
    - The request url is not a path.
        return Error 400
    - The request url doesn't exist in the system.
        return Error 404
    - The request url is pointing to a directory. 
        It is interpreted as pointing to one of these files: index.html, index.shtml, and index.txt. 
    - The request url is pointing to a file.
        - The file exists and it could be opened
            return 200 OK
        - File not Found
            return Error 404 
        - File can not open
            return Error 403
    
    All the requests and responses are recorded to a file log.txt. The format is "Time path response-code".
        - If can't create log file, return "Can't create log file! Please create a folder /log".
        - If can't write to log file, return "Can't write into log".

ASSUMPTION:
    - The http request should be in one line, starting with "GET" and following by a file path and protocal.
    - The file path should be a Unix-style path to a file, and the program will add current path before the input path.
    - The program will write logs to /log/log.txt. It will return an error if the folder doesn't exist.
    - When the log file is being written, it is locked until the finishment of the operation.
    - If the request is a valid path, the program will try to search index file before returning an error.

REFERENCE:
Let's Build a Web Server in Rust:
    https://dfockler.github.io/2016/05/20/web-server.html