use std::io::{Stdin, Stdout, Write};
use crate::terminalerror::TerminalError;
use crate::todo::Todo;
use crate::todos::Todos;
use console::style;
use clearscreen::clear;
use rand::prelude::random;

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
    clear().expect("Falhou em limpar a tela");
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
                println!("{}" , style("Minha lista de todos: ").bold());
                for i in &todo_collection.todos {
                    let x: u8 = random();
                    println!("{} : {:?}", count, style(&i.message.to_uppercase()).color256(x));
                    count += 1
                }
            } else {
                println!("{}", style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red());
            }         
        } else if options == "atualizar" {
            println!("{}", style("Número do Todo :").bold().green());
            let number_todo = terminal.input()?;
            let number = number_todo.parse::<usize>();
            match number {
                Ok(number) => 
                    if number > todo_collection.todos.len() || number == 0 {
                        println!("{}", style("Número de Todo Inválido!").red().bold())
                    } else {
                        println!("{}" , style("Novo Todo :").bold());
                        let todo = terminal.input()?;
                        todo_collection.update_todo(number, todo);
                        println!("{}" , style("Todo atualizado com Sucesso!!").blue().bold())
                    }
                ,
                Err(_) => println!("[ERRO] Digite um número e não uma letra!")
            }
        } else if options == "deletar" {
            println!("{}", style("Escolha o Todo que deseja deletar!").bold().yellow());
            let number_todo = terminal.input()?;
            let number = number_todo.parse::<usize>();
            match number {
                Ok(number) => 
                    if number > todo_collection.todos.len() || number == 0 {
                        println!("{}", style("Número de Todo Inválido!").red().bold())
                    } else {
                        todo_collection.remove_todo(number);
                        println!("{}" , style("Todo removido com Sucesso!!").white().bold())
                    }
                ,
                Err(_) => println!("[ERRO] Digite somente números!")
            }
        } else {
            println!("{}", style("ToDo Encerrado! 💤").underlined().bold());
            return Ok(())
        }
    }
}