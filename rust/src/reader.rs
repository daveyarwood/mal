extern crate regex;

use regex::Regex;

struct Reader {
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    fn next(&mut self) -> Option<String> {

    }

    fn peek(self) -> Option<String> {

    }
}

fn tokenizer(input: String) -> Vec<String> {
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#);
    let mut results = vec![];

    for cap in re.captures_iter(input) {
        let token = cap.at(1).unwrap_or("");
        if token == "" { break; }
        results.push(token);
    }

    results
}

fn read_str(input: String) {
    let tokens = tokenizer(input);
    let mut reader = Reader{tokens: tokens, position: 0};
    read_form(reader);
}
