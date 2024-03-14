use std::{
    fs,
    io::{self, IsTerminal, Write},
};

use mini_c::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

use std::env;

fn piped() {
    let stdin = io::stdin().lock();

    let input = io::read_to_string(stdin).unwrap();

    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan();
    let mut parser = Parser::new(&tokens);
    let ops = parser.parse();
    let mut interpreter = Interpreter::new(&ops);
    let result = interpreter.interpret();

    println!("{}", result);
}

fn repl() -> Result<(), io::Error> {
    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        let mut scanner = Scanner::new(buffer);
        let tokens = scanner.scan();
        let mut parser = Parser::new(&tokens);
        let ops = parser.parse();
        let mut interpreter = Interpreter::new(&ops);
        let result = interpreter.interpret();

        println!("{}", result);
    }
}

fn file(args: &[String]) -> Result<(), io::Error> {
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan();
    let mut parser = Parser::new(&tokens);
    let ops = parser.parse();
    let mut interpreter = Interpreter::new(&ops);
    let result = interpreter.interpret();

    println!("{}", result);
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();

    match args.len() {
        1 => {
            if std::io::stdin().is_terminal() {
                repl().unwrap();
            } else {
                piped();
            }
        }
        2 => file(&args).unwrap(),
        _ => {}
    }
}
