use regex::Regex;

struct Reader {
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    fn next(&mut self) -> Option<String> {
        if self.position < self.tokens.len() {
            let ref token = self.tokens[self.position];
            self.position += 1;
            Some(token.to_owned())
        } else {
            None
        }
    }

    fn peek(self) -> Option<String> {
        Some(self.tokens[self.position].to_owned())
    }
}

fn tokenize(input: String) -> Vec<String> {
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#).unwrap();
    let mut results = vec![];

    for cap in re.captures_iter(&input) {
        let token = cap.at(1).unwrap_or("");
        if token == "" { break; }
        results.push(token.to_string());
    }

    results
}

fn read_form(reader: Reader) -> String {
    let token = reader.peek().unwrap();
    match &token as &str {
        "(" => "left paren".to_string(),
        _   => "???".to_string()
    }
}

pub fn read_str(input: String) -> String {
    let tokens = tokenize(input);
    let mut reader = Reader{tokens: tokens, position: 0};
    read_form(reader)
}
