use std::io::stdin;

#[derive(Debug, PartialEq)]
pub struct HttpHeader {
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

#[cfg(test)]
mod tests {
    use crate::HttpHeader;

    #[test]
    fn parse_headers_successfully() {
        let headers = [
            ("Content-Type: application/json", Some(HttpHeader{name: "Content-Type".to_string(), value: "application/json".to_string()})),
            ("content-type : application/json", Some(HttpHeader{name: "content-type".to_string(), value: "application/json".to_string()})),
            (" content-Type : application/json ", Some(HttpHeader{name: "content-Type".to_string(), value: "application/json".to_string()})),
        ];

        let mods = headers.map(|h| (HttpHeader::parse(&String::from(h.0)), h.1));

        for case in mods {
            assert_eq!(case.0, case.1)
        }
    }
}
