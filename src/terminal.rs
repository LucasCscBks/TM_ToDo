use std::io::{Stdin, Stdout, Write};
use crate::terminalerror::TerminalError;
use crate::todo::Todo;

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
    fn receive_option(&mut self) -> Result<String, TerminalError> {
        let mut res = self.input()?;
        if res.to_lowercase() == "sim" {
            return Ok(res)
        } else if res.to_lowercase() == "nao" {
            return Ok(res)
        } else {
            loop {
                println!("Comando errado, digite Sim ou Nao!");
                res = self.input()?;
                if res.to_lowercase() == "sim" || res.to_lowercase() == "nao" {
                    break;
                }
            }
        }
        Ok(res)
    }

    fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {
        println!("Olá, deseja adicionar um novo ToDo? ");
        println!("[Sim/Nao]");

        let res = self.receive_option()?;
        if res.to_lowercase() == "sim" {
            println!("Digite o ToDo que deseja criar: ");
            let todo_res = self.input()?;
            print!("Todo adicionado 👍 : ");
            Ok(Todo::new(todo_res))
        } else {
            println!("Você digitou: {}", res);
            println!("Encerrando ToDo! 💤");
            std::process::exit(0);
        }
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        let resolve = writeln!(self.stdout, "{}", todo.message);
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
        terminal.show_todo(&todo)?;
    }
}