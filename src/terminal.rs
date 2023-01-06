use std::io::{Stdin, Stdout, Write};
use crate::terminalerror::TerminalError;
use crate::todo::Todo;
use crate::todos::Todos;
use console::style;
use clearscreen::clear;

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
                    None => println!("Não foi possível adicionar todo!"),
                } 
            },
            SystemOptions::List => {
                todo_collection.show_todos()
            },
            SystemOptions::Update => {
                println!("{}", style("Número do Todo :").bold().green());
                let number_todo = terminal.input()?;
                let number = number_todo.parse::<usize>();
                match number {
                    Ok(number) => {
                        let todo = todo_collection.get_todo(number);
                        match todo {
                            Some(_todo) => {
                                println!("{}" , style("Novo Todo :").bold());
                                let new_todo = terminal.input()?;
                                todo_collection.update_todo(number, new_todo);
                                println!("{}" , style("Todo atualizado com Sucesso!!").blue().bold())
                            },
                            None => println!("{}", style("Número de Todo Inválido!").red().bold())
                        }
                    },
                    Err(_) => println!("[ERRO] Digite um número e não uma letra!")
                }
            },
            SystemOptions::Delete => {
                println!("{}", style("Escolha o Todo que deseja deletar!").bold().yellow());
                let number_todo = terminal.input()?;
                let number = number_todo.parse::<usize>();
                match number {
                    Ok(number) => 
                        todo_collection.remove_todo(number)
                    ,
                    Err(_) => println!("[ERRO] Digite somente números!")
            }
            },
            SystemOptions::Exit => {
                println!("{}", style("ToDo Encerrado! 💤").underlined().bold());
                return Ok(())
            },
            SystemOptions::Other => return Ok(())
        }
    }
}