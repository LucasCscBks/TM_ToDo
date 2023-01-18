use std::io::{Stdin, Stdout, Write};
use crate::terminalerror::TerminalError;
use crate::todo::Todo;
use crate::todos::Todos;
use console::{style, StyledObject};
use clearscreen::clear;
use rand::prelude::random;

pub struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

pub enum SystemOptions {
    Add,
    List,
    Update,
    Delete,
    Exit,
    Other
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
    
    fn system_options(&mut self) -> Result<SystemOptions, TerminalError> {
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
        clear().expect("Falhou em limpar a tela");

        while !matches!(&*res, "adicionar" | "listar" | "atualizar" | "sair" | "deletar") {
            println!("{}", style("COMANDO ERRADO").red());
            println!(
                "Digite {},{}, {},{} ou {}", 
                style("Adicionar").blue().bold(), 
                style("Listar").yellow().bold(),
                style("Atualizar").green().bold(),
                style("Deletar").magenta().bold(),
                style("Sair").red().bold()
            );
            res = self.input()?.to_lowercase();
        }
        match res.trim() {
            "adicionar" => Ok(SystemOptions::Add),
            "listar" => Ok(SystemOptions::List),
            "atualizar" => Ok(SystemOptions::Update),
            "deletar" => Ok(SystemOptions::Delete),
            "sair" => Ok(SystemOptions::Exit),
            _ => Ok(SystemOptions::Other)
        }
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

    fn show_error_msg(&mut self, message: StyledObject<String>) {
        println!("{}", message)
    }

    fn show_message(&mut self, message: StyledObject<String>) {
        println!("{}", message);
    }

    fn show_todos(&mut self, index: i32, message: StyledObject<&String>) {
        println!("{} : {}", index, message);
    }

}

pub fn loop_todo() -> Result<(), TerminalError> {
    clear().expect("Falhou em limpar a tela");
    let mut todo_collection = Todos::new();
    
    loop {   
        let mut terminal = Terminal::new();
        let options = terminal.system_options()?;

        match options {
            SystemOptions::Add => {
                let todo = terminal.new_todo()?;
                match todo {
                    Some(todo) => {
                        todo_collection.add_todo(todo.clone());
                        terminal.show_todo(&todo)?;
                    },
                    None => terminal.show_message(style("Não foi possível adicionar todo!".to_uppercase())),
                } 
            },
            SystemOptions::List => {
                let collection = todo_collection.get_todos();
                if collection.is_empty() {
                    terminal.show_message(style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red())
                } else {
                    let mut count = 1;
                    terminal.show_message(style("Minha lista de todos: ".to_string()).bold());
                    for i in collection {
                        let x: u8 = random();
                        terminal.show_todos(count, style(&i.message.to_uppercase()).color256(x));
                        count += 1
                    }      
                }
            },
            SystemOptions::Update => {
                terminal.show_message(style("Número do Todo :".to_string()).bold().green());
                let number_todo = terminal.input()?;
                let number = number_todo.parse::<usize>();
                match number {
                    Ok(number) => {
                        let todo = todo_collection.get_todo(number);
                        match todo {
                            Some(_todo) => {
                                terminal.show_message(style("Novo Todo :".to_string()).bold());
                                let new_todo = terminal.input()?;
                                todo_collection.update_todo(number, new_todo);
                                terminal.show_message(style("Todo atualizado com Sucesso!!".to_string()).blue().bold())
                            },
                            None => terminal.show_message(style("Número de Todo Inválido!".to_string()).red().bold())
                        }
                    },
                    Err(_) => terminal.show_error_msg(style("[ERRO] Digite um número e não uma letra!".to_string().to_uppercase()).red())
                }
            },
            SystemOptions::Delete => {
                terminal.show_message(style("Escolha o Todo que deseja deletar!".to_string()).bold().yellow());
                let number_todo = terminal.input()?;
                let number = number_todo.parse::<usize>();
                match number {
                    Ok(number) => {
                        let todo = todo_collection.get_todo(number);
                        match todo {
                            Some(_todo) => {
                                todo_collection.remove_todo(number);
                                terminal.show_message(style("Todo removido com Sucesso!!".to_string()).white().bold())
                            },
                            None => terminal.show_message(style("Número de Todo Inválido!".to_string()).red().bold())
                        }
                    },     
                    Err(_) => terminal.show_error_msg(style("[ERRO] Digite somente números!".to_string().to_uppercase()).red())
            }
            },
            SystemOptions::Exit => {
                terminal.show_message(style("ToDo Encerrado! 💤".to_string()).underlined().bold());
                return Ok(())
            },
            SystemOptions::Other => return Ok(())
        }
    }
}