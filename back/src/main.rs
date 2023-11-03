use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::io::Error;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        handle_connection(_stream);
    }
}

enum HttpMethod {
    GET,
    POST,
}

struct HttpHeader {
    name: String,
    value: String,
}

struct HttpRequest {
    method: HttpMethod,
    uri: String,
    headers: Vec<HttpHeader>,
}


impl HttpRequest {
    fn empty() -> Self {
        HttpRequest {
            method: HttpMethod::GET,
            uri: "".parse().unwrap(),
            headers: vec![],
        }
    }

    fn addHeader(mut self, line: String) {
        match line.split(":") {
            [name, value] => self.headers.push(HttpHeader{ name, value })
        }
    }

    fn addFirstLine(&mut self, line: String) {
        let splited: Vec<&str> = line.split(" ").collect();
        self.method = match splited.get(0) {
            Some(&"GET") => HttpMethod::GET,
            Some(&"POST") => HttpMethod::POST,
            _ => HttpMethod::POST,
        };
        self.uri = match splited.get(1) {
            Some(&val) => val
            _ => "_"
        }.parse().unwrap()
    }

    fn parseHeader(&mut self, line: std::io::Result<String>) {
        match line {
            Ok(l) => self.addHeader(l)
            Err(_) => {}
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.lines()
        .fold(HttpRequest::empty(), |acc, line| acc.parseHeader(line) )
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("html/index.html").unwrap();
    let content_length = content.len();

    let response =
        format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
