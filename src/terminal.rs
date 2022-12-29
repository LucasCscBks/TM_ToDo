use std::io::{Stdin, Stdout, Write};
use crate::terminalerror::TerminalError;
use crate::todo::Todo;
use crate::todos::Todos;
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
    
    fn system_options(&mut self) -> Result<String, TerminalError> {
        println!("{}", style("Bem vindo ao sistema de Todos! Escolha uma opção abaixo: ").green());
        println!(
            "[{}|{}|{}|{}|{}]",
            style("Adicionar").bold().blue(), 
            style("Listar").bold().yellow(),
            style("Atualizar").bold().green(),
            style("Deletar").magenta().bold(),
            style("Sair").bold().red()
        );

        let mut res = self.input()?.to_lowercase();

        while !matches!(&*res, "adicionar" | "listar" | "atualizar" | "sair" | "deletar") {
            println!("{}", style("COMANDO ERRADO").red());
            println!(
                "Digite {},{}, {},{} ou {}", 
                style("Listar").blue().bold(), 
                style("Adicionar").yellow().bold(),
                style("Atualizar").green().bold(),
                style("Deletar").magenta().bold(),
                style("Sair").red().bold()
            );
            res = self.input()?.to_lowercase();
        }
        Ok(res)
    }

    fn new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {

        println!("{}", style("Digite o ToDo que deseja criar: ").cyan());
        let todo_res = self.input()?;
        print!("{}", style("TODO ADICIONADO 👍 : ").bold().magenta());
        
        Ok(Some(Todo::new(todo_res)))
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
    let mut todo_collection = Todos::new();
    
    loop {   
        let mut terminal = Terminal::new();
        let options = terminal.system_options()?;

        if options == "adicionar" {
            let todo = terminal.new_todo()?;
            match todo {
                Some(todo) => {
                    todo_collection.add_todo(todo.clone());
                    terminal.show_todo(&todo)?;
                },
                None => println!("Não foi possível adicionar todo!"),
            } 
        } else if options == "listar" {
            if todo_collection.todos.len() > 0 {
                let mut count = 1;
                println!("Minha lista de todos: ");
                for i in &todo_collection.todos {
                    println!("{} : {:?}", count, i.message);
                    count += 1
                }
            } else {
                println!("{}", style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red());
            }         
        } else if options == "atualizar" {
            println!("Número do Todo :");
            let number_todo = terminal.input()?;
            let number = number_todo.parse::<usize>();
            match number {
                Ok(number) => 
                    if number > todo_collection.todos.len() || number == 0 {
                        println!("{}", style("Número de Todo Inválido!").red().bold())
                    } else {
                        println!("Novo Todo :");
                        let todo = terminal.input()?;
                        todo_collection.todos[number - 1] = Todo::new(todo);
                        println!("{}" , style("Todo atualizado com Sucesso!!").blue().bold())
                    }
                ,
                Err(_) => println!("[ERRO] Digite um número e não uma letra!")
            }
        } else if options == "deletar" {
            println!("Escolha o Todo que deseja deletar!");
            let number_todo = terminal.input()?;
            let number = number_todo.parse::<usize>();
            match number {
                Ok(number) => 
                    if number > todo_collection.todos.len() || number == 0 {
                        println!("{}", style("Número de Todo Inválido!").red().bold())
                    } else {
                        todo_collection.todos.remove(number - 1);
                        println!("{}" , style("Todo removido com Sucesso!!").white().bold())
                    }
                ,
                Err(_) => println!("[ERRO] Digite somente números!")
            }
        } else {
            println!("{}", style("Encerrando ToDo! 💤").underlined().bold());
            return Ok(())
        }
    }
}