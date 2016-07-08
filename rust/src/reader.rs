use regex::Regex;
use types::MalVal;
use util;

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
    } else if Regex::new(r#"^".*"$"#).unwrap().is_match(&token) {
        let string = util::unescape(&token[1..token.len()-1]);
        MalVal::String(string)
    } else {
        MalVal::Atom(token)
    }
}

fn read_seq(reader: &mut Reader, type_name: &str,
            start_token: &str, end_token: &str) -> Vec<MalVal> {
    // Make sure the first token is right
    let first_token = reader.next()
                            .expect(&format!("Expected '{}', but got EOF.",
                                             start_token));
    assert!(first_token == start_token,
            format!("A {} must start with '{}'.", type_name, start_token));

    let mut seq = Vec::<MalVal>::new();

    loop {
        // Throw an exception if there's no more input and the list hasn't been
        // closed.
        let token = reader.peek()
                          .expect(&format!("Expected '{}', but got EOF.",
                                           end_token));

        if &token == end_token {
            reader.next();
            break;
        } else {
            let form = read_form(reader).unwrap();
            seq.push(form);
        }
    }

    seq
}

fn read_list(reader: &mut Reader) -> MalVal {
    let list = read_seq(reader, "list", "(", ")");
    MalVal::List(list)
}

fn read_vector(reader: &mut Reader) -> MalVal {
    let vec = read_seq(reader, "vector", "[", "]");
    MalVal::Vector(vec)
}

fn read_starting_at(token: String, reader: &mut Reader) -> MalVal {
    match &token as &str {
        "(" => read_list(reader),
        "[" => read_vector(reader),
        _   => read_atom(reader)
    }
}

fn read_form(reader: &mut Reader) -> Option<MalVal> {
    reader.peek().and_then(|token| Some(read_starting_at(token, reader)))
}

pub fn read_str(input: String) -> Result<Option<MalVal>, String> {
    let tokens = tokenize(input);
    let reader = &mut Reader{tokens: tokens, position: 0};
    match read_form(reader) {
        Some(form) => Ok(Some(form)),
        None       => Ok(None)
    }
}
