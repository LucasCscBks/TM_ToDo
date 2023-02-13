use tokio::io::{Stdin, Stdout, BufReader, AsyncBufReadExt};
use async_trait::async_trait;
use crate::terminalerror::TerminalError;
use crate::todo::Todo;
use console::{style, StyledObject};
use clearscreen::clear;

pub struct Terminal {
    stdin: BufReader<Stdin>,
    _stdout: Stdout,
}

pub enum SystemOptions {
    Add,
    List,
    Update,
    Resolve,
    Delete,
    Exit,
    Other
}

#[async_trait]
pub trait UserInterface {
    async fn input(&mut self) -> Result<String, TerminalError>;
    async fn system_options(&mut self) -> Result<SystemOptions, TerminalError>;
    async fn new_todo(&mut self) -> Result<Option<Todo>, TerminalError>;
    async fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError>;
    async fn show_error(&mut self, error: TerminalError);
    async fn show_error_msg(&mut self, message: StyledObject<String>);
    async fn show_message(&mut self, message: StyledObject<String>);
    async fn show_todos(&mut self, index: i32, message: StyledObject<&String>, resolved: bool);
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdin: BufReader::new(tokio::io::stdin()),
            _stdout: tokio::io::stdout(),
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserInterface for Terminal {
    async fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf: String = String::new();
        
        match self.stdin.read_line(&mut buf).await {
            Ok(_) => Ok(buf.trim().to_string()),
            Err(error) => Err(TerminalError::Stdin(error))
        }
    }
    
    async fn system_options(&mut self) -> Result<SystemOptions, TerminalError> {
        println!("{}", style("Bem vindo ao sistema de Todos! Escolha uma opção abaixo: ").green());
        println!(
            "[{}|{}|{}|{}|{}|{}]",
            style("Adicionar").bold().blue(), 
            style("Listar").bold().yellow(),
            style("Atualizar").bold().green(),
            style("Resolver").bold().white(),
            style("Deletar").magenta().bold(),
            style("Sair").bold().red()
        );

        let mut res = self.input().await?.to_lowercase();
        clear().expect("Falhou em limpar a tela");

        while !matches!(&*res, "adicionar" | "listar" | "atualizar" | "resolver" | "sair" | "deletar") {
            println!("{}", style("COMANDO ERRADO").red());
            println!(
                "Digite {},{}, {},{}, {} ou {}", 
                style("Adicionar").blue().bold(), 
                style("Listar").yellow().bold(),
                style("Atualizar").green().bold(),
                style("Resolver").bold().white(),
                style("Deletar").magenta().bold(),
                style("Sair").red().bold()
            );
            res = self.input().await?.to_lowercase();
        }
        match res.trim() {
            "adicionar" => Ok(SystemOptions::Add),
            "listar" => Ok(SystemOptions::List),
            "atualizar" => Ok(SystemOptions::Update),
            "resolver" => Ok(SystemOptions::Resolve),
            "deletar" => Ok(SystemOptions::Delete),
            "sair" => Ok(SystemOptions::Exit),
            _ => Ok(SystemOptions::Other)
        }
    }

    async fn new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {

        println!("{}", style("Digite o ToDo que deseja criar: ").cyan());
        let todo_res = self.input().await?;
        print!("{}", style("TODO ADICIONADO 👍 : ").bold().magenta());
        
        Ok(Some(Todo::new(todo_res, false)))
    }

    async fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        println!("{}", style(&todo.message).italic().underlined().color256(105));
        Ok(())
    }

    async fn show_error(&mut self, error: TerminalError) {
        eprintln!("{}", error.error_type());
    }

    async fn show_error_msg(&mut self, message: StyledObject<String>) {
        println!("{}", message)
    }

    async fn show_message(&mut self, message: StyledObject<String>) {
        println!("{}", message);
    }

    async fn show_todos(&mut self, index: i32, message: StyledObject<&String>, resolved: bool) {
        if !resolved {
            println!("{} : {} - 🎯", index, message);
        } else {
            println!("{} : {} - ✅", index, message);
        }
    }
}