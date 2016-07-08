pub enum MalVal {
    Atom(String),
    Boolean(bool),
    Error(String),
    Int(isize),
    List(Vec<MalVal>),
    Nil,
    String(String),
    Vector(Vec<MalVal>)
}
