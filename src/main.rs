use std::{
    env,
    io::{stdin, stdout, Write},
};

use nosh::parser::parser::CommandParser;
use whoami;
use colorize::{self, AnsiColor}; 

fn main() {
    shell_loop();
}

fn shell_loop() {
    loop {
        print!(
            "{}@{}$",
            format!("{}", whoami::username()).bold().green(),
            format!("{}", env::current_dir().unwrap().display()).bold().cyan()
        );
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("cannot read line.");
        line.remove(line.len() - 1);

        let mut parser = CommandParser::new();
        let res = parser.execute(line);

        dbg!("{:?}", res);
    }
}
