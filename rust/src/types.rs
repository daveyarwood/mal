use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum MalVal {
    Atom(String),
    Boolean(bool),
    Error(String),
    HashMap(HashMap<MalVal, MalVal>),
    Int(isize),
    Keyword(String),
    List(Vec<MalVal>),
    Nil,
    String(String),
    Vector(Vec<MalVal>)
}
