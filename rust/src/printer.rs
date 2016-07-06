use types::MalVal;

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

pub fn pr_str(form: &MalVal) -> String {
    match *form {
        MalVal::Atom(ref atom)  => atom.to_owned(),
        MalVal::Int(n)          => n.to_string(),
        MalVal::List(ref list)  => list_to_str(list),
        MalVal::Vector(ref vec) => vector_to_str(vec)
    }
}
