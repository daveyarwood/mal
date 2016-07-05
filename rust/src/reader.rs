use regex::Regex;
use types::MalVal;

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

    fn peek(&mut self) -> Option<String> {
        if self.position < self.tokens.len() {
            let ref token = self.tokens[self.position];
            Some(token.to_owned())
        } else {
            None
        }
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

fn read_atom(reader: &mut Reader) -> MalVal {
    let token = reader.next().expect("Expected an atom, but got EOF.");
    if Regex::new(r"^-?\d+$").unwrap().is_match(&token) {
        let n = token.parse().expect("Error parsing integer.");
        MalVal::Int(n)
    } else {
        MalVal::Atom(token)
    }
}

fn read_list(reader: &mut Reader) -> MalVal {
    // Make sure the first token is "("
    let first_token = reader.next().expect("Expected '(', but got EOF.");
    assert!(first_token == "(", "A list must start with '('.");

    let mut list = Vec::<MalVal>::new();

    loop {
        // Throw an exception if there's no more input and the list hasn't been
        // closed.
        let token = reader.peek().expect("Expected ')', but got EOF.");

        if &token == ")" {
            reader.next();
            break;
        } else {
            let form = read_form(reader).unwrap();
            list.push(form);
        }
    }

    MalVal::List(list)
}

fn read_form(reader: &mut Reader) -> Option<MalVal> {
    let token = reader.peek().unwrap();
    match &token as &str {
        "(" => Some(read_list(reader)),
        _   => Some(read_atom(reader))
    }
}

pub fn read_str(input: String) -> Result<MalVal, String> {
    let tokens = tokenize(input);
    let reader = &mut Reader{tokens: tokens, position: 0};
    match read_form(reader) {
        Some(form) => Ok(form),
        None       => Err("Invalid input.".to_string())
    }
}
