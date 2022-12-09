use std::io::{Stdin, Stdout, Write};
use crate::terminalerror::TerminalError;
use crate::todo::Todo;
use console::style;

pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf: String = String::new();

        match self.stdin.read_line(&mut buf) {
            Ok(_) => Ok(buf.trim().to_string()),
            Err(error) => Err(TerminalError::Stdin(error))
        }
    }
    fn should_ask_for_new_todo(&mut self) -> Result<bool, TerminalError> {
        let mut res = self.input()?;
        while res.to_lowercase() != "sim" && res.to_lowercase() != "nao" {
            println!("{}", style("COMANDO ERRADO").red());
            println!("Digite {} ou {}", style("Sim").blue().bold(), style("Nao").yellow().bold());
            res = self.input()?;
        }
        if res.to_lowercase() == "sim" {
            Ok(true)
        } else {
            Ok(false)
        }
        
    }

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        println!("{}", style("Olá, deseja adicionar um novo ToDo? ").green());
        println!("[{}|{}]", style("Sim").bold().blue(), style("Nao").bold().yellow());

        let res = self.should_ask_for_new_todo()?;
        if res {
            println!("{}", style("Digite o ToDo que deseja criar: ").cyan());
            let todo_res = self.input()?;
            print!("{}", style("TODO ADICIONADO 👍 : ").bold().magenta());
            Ok(Some(Todo::new(todo_res)))
        } else {
            println!("{}", style("Encerrando ToDo! 💤").underlined().bold());
            Ok(None)
        }
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        let resolve = writeln!(self.stdout, "{}", style(&todo.message).italic().underlined().color256(105));
        resolve.map_err(TerminalError::Stdout)
    }

    pub fn show_error(&mut self, error: TerminalError) {
        eprintln!("{}", error.error_type());
    }
}

pub fn loop_todo() -> Result<(), TerminalError> {
    loop {   
        let mut terminal = Terminal::new();
        let todo = terminal.ask_for_new_todo()?;
        match todo {
            Some(todo) => terminal.show_todo(&todo)?,
            None => break
        }
    }
    Ok(())
}