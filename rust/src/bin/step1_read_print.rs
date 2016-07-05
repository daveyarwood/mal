extern crate linenoise;
extern crate mal;

use mal::reader;

fn read(input: String) -> String {
  reader::read_str(input)
}

fn eval(code: String) -> String {
  code
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
