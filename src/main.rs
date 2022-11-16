#[derive(Debug, Clone)]
struct Todo {
    message: String
}

impl Todo {
    fn new(message: String) -> Self{
        Todo {
            message
        }
    }
}
use std::io::{Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout()
        }
    }

    fn input(&mut self) -> String {
        let mut buf: String = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }

    fn ask_for_new_todo(&mut self) -> Todo {
        println!("Olá deseja adicionar um novo ToDo? (Digite 'sim' para adicionar) ");
        
        let res = self.input();
        if res.to_lowercase() == "sim" {
            println!("Digite o ToDo que deseja criar: ");
            let todo_res = self.input();
            print!("Todo adicionado 👍 : ");
            let todo: Todo = Todo::new(todo_res);
            return todo
        } else {
            println!("Você digitou: {}" , res);
            println!("Encerrando ToDo! 💤");
            std::process::exit(0)
        }
        
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "{}", todo.message).unwrap();
    }
}

fn main() {
    loop {
        let mut terminal = Terminal::new();
        let todo = terminal.ask_for_new_todo();
        terminal.show_todo(&todo);
    }
}