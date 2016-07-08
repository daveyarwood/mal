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

fn read_atom(reader: &mut Reader) -> Result<MalVal, String> {
    let token = reader.next().expect("Expected an atom, but got EOF.");
    if Regex::new(r"^-?\d+$").unwrap().is_match(&token) {
        let n = token.parse().expect("Error parsing integer.");
        Ok(MalVal::Int(n))
    } else if Regex::new(r#"^".*"$"#).unwrap().is_match(&token) {
        let string = util::unescape(&token[1..token.len()-1]);
        Ok(MalVal::String(string))
    } else if Regex::new(r#"^:.*$"#).unwrap().is_match(&token) {
        Ok(MalVal::Keyword(token[1..token.len()].to_string()))
    } else if token == "nil" {
        Ok(MalVal::Nil)
    } else if token == "true" {
        Ok(MalVal::Boolean(true))
    } else if token == "false" {
        Ok(MalVal::Boolean(false))
    } else {
        Ok(MalVal::Atom(token))
    }
}

fn read_seq(reader: &mut Reader, type_name: &str,
            start_token: &str, end_token: &str) -> Result<Vec<MalVal>, String> {
    // Make sure the first token is right
    let first_token = reader.next()
                            .expect(&format!("Expected '{}', but got EOF.",
                                             start_token));
    assert!(first_token == start_token,
            format!("A {} must start with '{}'.", type_name, start_token));

    let mut seq = Vec::<MalVal>::new();

    loop {
        let token = reader.peek();
        if token.is_none() {
            // Throw an exception if there's no more input and the list hasn't
            // been closed.
            return Err(format!("Expected '{}', but got EOF.", end_token));
        } else {
            if &token.unwrap() == end_token {
                reader.next();
                break;
            } else {
                match read_form(reader) {
                    Ok(hopefully_form) => seq.push(hopefully_form.unwrap()),
                    Err(msg)           => return Err(msg)
                }
            }
        }
    }

    Ok(seq)
}

fn read_list(reader: &mut Reader) -> Result<MalVal, String> {
    match read_seq(reader, "list", "(", ")") {
        Ok(list) => Ok(MalVal::List(list)),
        Err(msg) => Err(msg)
    }
}

fn read_vector(reader: &mut Reader) -> Result<MalVal, String> {
    match read_seq(reader, "vector", "[", "]") {
        Ok(vec)  => Ok(MalVal::Vector(vec)),
        Err(msg) => Err(msg)
    }
}

fn read_hashmap(reader: &mut Reader) -> Result<MalVal, String> {
    let contents = read_seq(reader, "hash-map", "{", "}");

    // TODO: store contents in an associative data structure;
    //       validate even number of forms;
    //       only allow certain kinds of keys?

    match contents {
        Ok(map)  => Ok(MalVal::HashMap(map)),
        Err(msg) => Err(msg)
    }
}

fn read_prefixed_form(reader: &mut Reader, type_name: &str, symbol_name: &str,
                      start_token: &str) -> Result<MalVal, String> {
    // Make sure the first token is right
    let first_token = reader.next()
                            .expect(&format!("Expected '{}', but got EOF.",
                                             start_token));
    assert!(first_token == start_token,
            format!("A(n) {} must start with '{}'.", type_name, start_token));

    match read_form(reader) {
        Ok(Some(form)) => Ok(MalVal::List(vec![MalVal::Atom(symbol_name.to_string()), form])),
        Ok(None)       => Err(format!("Invalid {}.", type_name)),
        Err(msg)       => Err(msg)
    }
}

fn read_form_with_metadata(reader: &mut Reader) -> Result<MalVal, String> {
    // Make sure the first token is right
    let first_token = reader.next().expect("Expected '^', but got EOF.");
    assert!(first_token == "^", "A form with metadata must start with '^'.");

    match read_form(reader) {
        Ok(Some(MalVal::HashMap(metadata))) => match read_form(reader) {
            Ok(Some(form)) => Ok(MalVal::List(vec![MalVal::Atom("with-meta".to_string()), form, MalVal::HashMap(metadata)])),
            Ok(None)       => return Err("Can't attach metadata to nothing.".to_string()),
            Err(msg)       => return Err(msg)
        },
        Ok(Some(_)) => return Err("Metadata must be a hash-map.".to_string()),
        Ok(None)              => return Err("Invalid use of '^'.".to_string()),
        Err(msg) => return Err(msg)
    }
}

fn read_form_starting_at(token: String,
                         reader: &mut Reader) -> Result<MalVal, String> {
    match &token as &str {
        "("  => read_list(reader),
        "["  => read_vector(reader),
        "{"  => read_hashmap(reader),
        ")"  => Err("Unexpected end of list ')'.".to_string()),
        "]"  => Err("Unexpected end of vector ']'.".to_string()),
        "}"  => Err("Unexpected end of hash-map '}'.".to_string()),
        "'"  => read_prefixed_form(reader, "quoted form", "quote", "'"),
        "`"  => read_prefixed_form(reader, "quasiquoted form", "quasiquote", "`"),
        "~"  => read_prefixed_form(reader, "unquoted form", "unquote", "~"),
        "~@" => read_prefixed_form(reader, "splice-unquoted form", "splice-unquote", "~@"),
        "@"  => read_prefixed_form(reader, "dereferenced form", "deref", "@"),
        "^"  => read_form_with_metadata(reader),
        _    => read_atom(reader)
    }
}

fn read_form(reader: &mut Reader) -> Result<Option<MalVal>, String> {
    match reader.peek() {
        Some(token) => match read_form_starting_at(token, reader) {
            Ok(value) => Ok(Some(value)),
            Err(msg)  => Err(msg)
        },
        None        => Ok(None)
    }
}

pub fn read_str(input: String) -> Result<Option<MalVal>, String> {
    let tokens = tokenize(input);
    let reader = &mut Reader{tokens: tokens, position: 0};
    read_form(reader)
}
