use regex::Regex;
use std::collections::HashMap;
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
        match reader.peek() {
            // Throw an exception if there's no more input and the list hasn't
            // been closed.
            None => {
                return Err(format!("Expected '{}', but got EOF.", end_token))
            },
            // If the next token is the token that ends the seq, move ahead and
            // break.
            Some(ref token) if token == end_token => {
                reader.next();
                break;
            },
            // If it's anything else, read the next form and add it to the seq.
            Some(_) => {
                match read_form(reader) {
                    Err(msg)       => { return Err(msg); },
                    Ok(None)       => { reader.next(); },
                    Ok(Some(form)) => { seq.push(form); }
                };
            }
        };
    };

    Ok(seq)
}

fn read_list(reader: &mut Reader) -> Result<MalVal, String> {
    read_seq(reader, "list", "(", ")")
        .map(|list| MalVal::List(list))
}

fn read_vector(reader: &mut Reader) -> Result<MalVal, String> {
    read_seq(reader, "vector", "[", "]")
        .map(|vec| MalVal::Vector(vec))
}

fn read_hashmap(reader: &mut Reader) -> Result<MalVal, String> {
    match read_seq(reader, "hash-map", "{", "}") {
        Err(msg) => Err(msg),
        Ok(contents) => {
            if contents.len() % 2 == 0 {
                let map = HashMap::new();
                Ok(MalVal::HashMap(map))
            } else {
                Err("A hash-map must contain an even number of forms.".to_string())
            }
        }
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
        Ok(Some(form)) => {
            let symbol = MalVal::Atom(symbol_name.to_string());
            Ok(MalVal::List(vec![symbol, form]))
        },
        Ok(None)       => Err(format!("Invalid {}.", type_name)),
        Err(msg)       => Err(msg)
    }
}

fn read_form_with_metadata(reader: &mut Reader) -> Result<MalVal, String> {
    // Make sure the first token is right
    let first_token = reader.next().expect("Expected '^', but got EOF.");
    assert!(first_token == "^", "A form with metadata must start with '^'.");

    match read_form(reader) {
        Ok(Some(MalVal::HashMap(metadata))) => {
            match read_form(reader) {
                Ok(Some(form)) => {
                    let with_meta = MalVal::Atom("with-meta".to_string());
                    let meta      = MalVal::HashMap(metadata);
                    Ok(MalVal::List(vec![with_meta, form, meta]))
                },
                Ok(None)       => return Err("Can't attach metadata to nothing.".to_string()),
                Err(msg)       => return Err(msg)
            }
        },
        Ok(Some(_)) => return Err("Metadata must be a hash-map.".to_string()),
        Ok(None)    => return Err("Invalid use of '^'.".to_string()),
        Err(msg)    => return Err(msg)
    }
}

fn read_form_starting_at(token: String,
                         reader: &mut Reader) -> Result<Option<MalVal>, String> {
    // If the current token is a comment, return None
    if Regex::new(r#"^;.*$"#).unwrap().is_match(&token) {
        Ok(None)
    // Otherwise, read a form starting from the token
    } else {
        let form = match &token as &str {
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
        };

        form.map(|value| Some(value))
    }
}

fn read_form(reader: &mut Reader) -> Result<Option<MalVal>, String> {
    match reader.peek() {
        Some(token) => read_form_starting_at(token, reader),
        None        => Ok(None)
    }
}

pub fn read_str(input: String) -> Result<Option<MalVal>, String> {
    let tokens = tokenize(input);
    let reader = &mut Reader{tokens: tokens, position: 0};
    read_form(reader)
}
