pub enum MalVal {
    Atom(String),
    Boolean(bool),
    Error(String),
    HashMap(Vec<MalVal>), // TODO: use an associative data structure instead
    Int(isize),
    Keyword(String),
    List(Vec<MalVal>),
    Nil,
    String(String),
    Vector(Vec<MalVal>)
}
