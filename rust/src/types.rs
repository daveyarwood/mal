pub enum MalVal {
    Atom(String),
    Error(String),
    Int(isize),
    List(Vec<MalVal>),
    String(String),
    Vector(Vec<MalVal>)
}
