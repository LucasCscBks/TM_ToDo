#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Self {
        Todo { message }
    }
}
use std::io::{Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}
enum TerminalError {
    Stdout(String),
    Stdin(String),
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
            Err(error) => Err(TerminalError::Stdin(format!("Erro {:?}", error)))
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
        match resolve {
            Ok(resolve) => Ok(resolve),
            error => Err(TerminalError::Stdout(format!("Erro {:?}", error)))
        }
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
    loop_todo();
}
