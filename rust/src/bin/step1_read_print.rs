extern crate linenoise;
extern crate mal;

use mal::printer;
use mal::reader;
use mal::types::MalVal;

fn read(input: String) -> Option<MalVal> {
    match reader::read_str(input) {
        Ok(form) => form,
        Err(msg) => Some(MalVal::Error(msg))
    }
}

fn eval(form: Option<MalVal>) -> Option<MalVal> {
    form
}

fn print(result: Option<MalVal>) -> String {
    match result {
        Some(form) => printer::pr_str(&form),
        None       => String::new()
    }
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
