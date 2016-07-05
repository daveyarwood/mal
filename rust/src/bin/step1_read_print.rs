extern crate linenoise;
extern crate mal;

use mal::reader;
use mal::types::MalVal;

fn read(input: String) -> MalVal {
    match reader::read_str(input) {
        Ok(form) => form,
        Err(msg) => panic!(msg)
    }
}

fn eval(form: MalVal) -> String {
    "TODO: eval form".to_string()
}

fn print(result: String) -> String {
    result
}

fn rep(input: String) -> String {
    print(eval(read(input)))
}

fn main() {
    loop {
        match linenoise::input("user> ") {
            None => { break; }
            Some(input) => {
                linenoise::history_add(input.as_ref());
                println!("{}", rep(input));
            }
        }
    }
}
