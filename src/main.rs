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
use std::io::{Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}
enum TerminalError {
    Stdout(io::Error),
    Stdin(io::Error),
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

        let res = self.input();
        let res2 = match res {
            Ok(res) => if res.to_lowercase() == "sim" {
                println!("Digite o ToDo que deseja criar: ");
                let todo_res = self.input();
                let todo_res2 = match todo_res {
                    Ok(todo_res) => {
                        print!("Todo adicionado 👍 : ");
                        Ok(Todo::new(todo_res))
                    }
                    Err(error) => Err(error)
                };
                todo_res2
            } else {
                println!("Você digitou: {}", res);
                println!("Encerrando ToDo! 💤");
                std::process::exit(0);
            },
            Err(error) => Err(error)
        };
        res2
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        let resolve = writeln!(self.stdout, "{}", todo.message);
        let resolve2 = match resolve {
            Ok(resolve) => Ok(resolve),
            Err(error) => Err(TerminalError::Stdout(error))
        };
        resolve2
    }
}

fn main() {
    loop {
        let mut terminal = Terminal::new();
        let todo = terminal.ask_for_new_todo();
        match todo {
            Ok(todo) => terminal.show_todo(&todo),
            Err(error) => Err(error)
        };
    }
}
