use std::io::{Stdin, Stdout, Write};
use std::io;
#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Self {
        Todo { message }
    }
}

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

#[derive(Debug)]
enum TerminalError {
    Stdout(io::Error),
    Stdin(io::Error),
}

impl TerminalError {
    fn error_type(self) -> String {
        match self {
            Self::Stdin(err) => format!("Erro {}", err),
            Self::Stdout(err) => format!("Erro {}", err)
        }
    }
}

impl Terminal {
    fn new() -> Self {
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

    fn ask_for_new_todo(&mut self) -> Result<Todo, TerminalError> {
        println!("Olá, deseja adicionar um novo ToDo? ");
        println!("[Sim/Nao]");

        let res = self.input()?;
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

    fn show_error(&mut self, error: TerminalError) {
        eprintln!("{}", error.error_type());
    }
}

fn loop_todo() -> Result<(), TerminalError> {
    loop {   
        let mut terminal = Terminal::new();
        let todo = terminal.ask_for_new_todo()?;
        terminal.show_todo(&todo)?;
    }
}

fn main() {
    let mut terminal = Terminal::new();
    if let Err(error) = loop_todo() {
        terminal.show_error(error)
    }
}
