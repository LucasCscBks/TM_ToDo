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
    fn options_todo(&mut self) -> Result<String, TerminalError> {
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

    fn ask_for_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        println!("{}", style("Bem vindo ao sistema de Todos! Escolha uma opção abaixo: ").green());
        println!(
            "[{}|{}|{}|{}|{}]",
            style("Adicionar").bold().blue(), 
            style("Listar").bold().yellow(),
            style("Atualizar").bold().green(),
            style("Deletar").magenta().bold(),
            style("Sair").bold().red()
        );        

        let options = self.options_todo()?;

        if options == "adicionar" {
            println!("{}", style("Digite o ToDo que deseja criar: ").cyan());
            let todo_res = self.input()?;
            print!("{}", style("TODO ADICIONADO 👍 : ").bold().magenta());
            
            Ok(Some(Todo::new(todo_res)))
        } else if options == "listar" {
            Ok(Some(Todo::new("listar".to_string() )))
        } else if options == "atualizar" {
            Ok(Some(Todo::new("atualizar".to_string() )))
        } else if options == "deletar" {
            Ok(Some(Todo::new("deletar".to_string() )))
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
    let mut todo_collection = Todos::new();
    
    loop {   
        let mut terminal = Terminal::new();
        let todo = terminal.ask_for_new_todo()?;
        match todo {
            Some(todo) => {
                if todo.message != "listar".to_string() && todo.message != "atualizar".to_string() && todo.message != "deletar".to_string() {
                    todo_collection.add_todo(todo.message.clone());
                    terminal.show_todo(&todo)?;
                } else if todo.message == "atualizar".to_string() {
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
                        Err(_) => println!("Erro")
                    }
                } else if todo.message == "listar".to_string() {
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
                } else if todo.message == "deletar".to_string() {
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
                        Err(_) => println!("Erro")
                    }
                }                
            },
            None => return Ok(())
        }
    }
}