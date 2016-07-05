extern crate linenoise;
extern crate mal;

use mal::printer;
use mal::reader;
use mal::types::MalVal;

fn read(input: String) -> MalVal {
    match reader::read_str(input) {
        Ok(form) => form,
        Err(msg) => panic!(msg)
    }
}

fn eval(form: MalVal) -> MalVal {
    form
}

fn print(result: MalVal) -> String {
    printer::pr_str(&result)
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
