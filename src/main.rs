#[derive(Debug, Clone)]
struct Todo {
    message: String
}

impl Todo {
    fn new_todo(message: String) -> Self{
        Todo::new_todo(message)
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
        println!("Olá deseja adicionar um novo ToDo?");
        println!("[sim/nao]");
        let res = self.input();
        if res == "nao" {
            println!("Encerrando ToDo! 💤");
            std::process::exit(0)
        };
        if res == "sim" {
            println!("Digite o ToDo que deseja criar: ");
            let todo_res = self.input();
            Todo {message: "Todo adicionado 👍 : ".to_string() + &todo_res}
        } else {
            Todo {message: "❌ Alternativa inexistente!! Digite sim ou nao. ❌".to_string()}
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