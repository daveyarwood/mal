use types::MalVal;

fn list_to_str(list: &Vec<MalVal>) -> String {
    format!("({})", list.iter()
                        .map(|form| pr_str(form))
                        .collect::<Vec<String>>()
                        .join(" "))
}

pub fn pr_str(form: &MalVal) -> String {
    match *form {
        MalVal::Atom(ref atom) => atom.to_owned(),
        MalVal::Int(n)         => n.to_string(),
        MalVal::List(ref list) => list_to_str(list)
    }
}
