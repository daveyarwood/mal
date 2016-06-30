use regex::Regex;

struct Reader {
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    fn next(&mut self) -> Option<String> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(self) -> Option<String> {
        Some(self.tokens[self.position])
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

fn read_form(reader: Reader) {
    let token = reader.peek().unwrap();
    match token {
        "(" => "left paren",
        _   => "???"
    }
}

pub fn read_str(input: String) {
    let tokens = tokenizer(input);
    let mut reader = Reader{tokens: tokens, position: 0};
    read_form(reader);
}
