use types::MalVal;
use util;

fn seq_to_str(seq: &Vec<MalVal>, start_token: &str, end_token: &str) -> String {
    format!("{}{}{}",
            start_token,
            seq.iter()
               .map(|form| pr_str(form))
               .collect::<Vec<String>>()
               .join(" "),
            end_token)
}

fn list_to_str(list: &Vec<MalVal>) -> String {
    seq_to_str(list, "(", ")")
}

fn vector_to_str(vec: &Vec<MalVal>) -> String {
    seq_to_str(vec, "[", "]")
}

fn string_to_str(string: &str) -> String {
    format!("\"{}\"", util::escape(string))
}

fn bool_to_str(boolean: bool) -> String {
    if boolean {
        "true".to_string()
    } else {
        "false".to_string()
    }
}

pub fn pr_str(form: &MalVal) -> String {
    match *form {
        MalVal::Atom(ref atom)     => atom.to_owned(),
        MalVal::Boolean(boolean)   => bool_to_str(boolean),
        MalVal::Error(ref string)  => format!("ERROR: {}", string),
        MalVal::Int(n)             => n.to_string(),
        MalVal::Keyword(ref kw)    => format!(":{}", kw),
        MalVal::List(ref list)     => list_to_str(list),
        MalVal::Nil                => "nil".to_string(),
        MalVal::String(ref string) => string_to_str(string),
        MalVal::Vector(ref vec)    => vector_to_str(vec)
    }
}
