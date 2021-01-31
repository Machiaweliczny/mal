use std::io;

// https://github.com/kanaka/mal#mal---make-a-lisp

fn main() {
    println!("Hello in MAL sandbox!");
    let mut line : String = String::from("");
    while line != "exit\n" {
        line = rep()
    }
}

fn read() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            input
        }
        Err(_error) =>  "".to_string()
    }
}

fn eval(line: &str) -> String {
  return line.to_string();
}

fn print(ast: &str){
    println!("{}", ast);
}

fn rep() -> String {
    let line = read();
    let ast = eval(&line);
    print(&ast);
    return line;
}
    