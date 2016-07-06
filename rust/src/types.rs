pub enum MalVal {
    Atom(String),
    Int(isize),
    List(Vec<MalVal>),
    String(String),
    Vector(Vec<MalVal>)
}
