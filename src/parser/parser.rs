use super::{redirect::Redirect, cursor::Cursor};

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    command: String,
    sub_command: String,
    options: Vec<String>,
    pipe: Option<Box<ParsedCommand>>,
    redirect: Option<Redirect>,
    path: String,
}

impl ParsedCommand {
    pub fn new() -> Self {
       Self { 
            command: String::new(),
            sub_command: String::new(),
            options: Vec::new(),
            pipe: None,
            redirect: None,
            path: String::new(),
       }
    }
}

pub struct CommandParser {
    parsed_command: ParsedCommand,
    cursor: Cursor,
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            parsed_command: ParsedCommand::new(),
            cursor: Cursor::new()
        }
    }

    /// Parsing input to Command.
    /// * 'line' - Line input.
    pub fn execute(&mut self, line: String) -> ParsedCommand {
        let mut elements: Vec<&str> = line.split(" ").collect();
        self.parse_loop(&mut elements);
        self.parsed_command.clone()
    }

    pub fn get_parsed_command(&self) -> ParsedCommand {
        self.parsed_command.clone()
    }

    fn parse_loop(&mut self, mut elements: &mut Vec<&str>) {
        self.cursor.next();
        self.parsed_command.command = elements[0].to_string();

        loop {
            if elements.len() <= self.cursor.current() {
                break;
            }

            self.parse(&mut elements);
            self.cursor.next();
        }
    }

    fn parse(&mut self, args: &mut Vec<&str>) {
        let arg: &str = args[self.cursor.current()];

        // Redirection
        if arg.chars().nth(0).unwrap_or(' ') == '>' {
            self.cursor.next();
            let arg: &str = args[self.cursor.current()];

            // When over
            if arg.len() == 2 && arg.chars().nth(1).unwrap() == '>' {
                self.parsed_command.redirect = Some(Redirect::new(arg, true));
                return;
            }
            self.parsed_command.redirect = Some(Redirect::new(arg, false));
            return;
        }
        if arg.chars().nth(0).unwrap_or(' ') == '-' {
            self.parsed_command.options.push(arg.to_string());
            return;
        }
        if arg.contains("/") || arg.contains(".") {
            self.parsed_command.path = arg.to_string();
            return;
        }
        if arg == "|" {
            let mut piped_to: CommandParser = CommandParser::new();
            let mut arg_split: Vec<_> = Vec::new();

            for index in self.cursor.current() + 1..args.len() {
                arg_split.push(args[index]);
            }
            dbg!("{:?}", &arg_split);
            piped_to.parse_loop(&mut arg_split);
            self.parsed_command.pipe = Some(Box::new(piped_to.get_parsed_command()));
            self.cursor.forward(args.len() - self.cursor.current());
            return;
        }

        self.parsed_command.sub_command = args[1].to_string();
    }
}
