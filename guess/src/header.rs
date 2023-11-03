use std::io::stdin;

#[derive(Debug)]
struct HttpHeader {
    name: String,
    value: String
}

impl HttpHeader {
    fn new(n: &str, v:&str) -> Self {
        HttpHeader {
            name: String::from(n),
            value: String::from(v)
        }
    }

    fn parse(line: &String) -> Option<Self> {
        line.trim().split_once(":")
            .map(|p| Self::new(p.0.trim(), p.1.trim()))
    }
}

fn main() {
    loop {
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                if let Some(header) = HttpHeader::parse(&input) {
                    println!("header is {:?}", header)
                }
            }
            Err(_) => break
        }
    }
}
