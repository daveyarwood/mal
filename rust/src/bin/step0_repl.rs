use std::io;
use std::io::Write;

fn read(input: String) -> String {
  input
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
    print!("user> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let input = input.trim();

    println!("{}", rep(input.to_string()));
    // io::stdout().flush().unwrap();
  }
}
