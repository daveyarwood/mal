pub enum MalVal {
    Atom(String),
    Int(isize),
    List(Vec<MalVal>),
    Vector(Vec<MalVal>)
}
