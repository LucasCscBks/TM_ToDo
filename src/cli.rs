use crate::terminal::{SystemOptions, UserInterface};
use crate::todos::TodoStorage;
use rand::prelude::random;
use crate::terminalerror::TerminalError;
use clearscreen::clear;
use console::style;

pub(crate) struct TodoCli {
    user_interface: Box<dyn UserInterface>,
    todo_storage: Box<dyn TodoStorage>,
}

impl TodoCli {
    pub fn new(user_interface: Box<dyn UserInterface>, todo_storage: Box<dyn TodoStorage>) -> Self {
        Self {
            user_interface,
            todo_storage
        }
    }

    pub fn run(&mut self) -> Result<(), TerminalError> {
        clear().expect("Falhou em limpar a tela");
        loop {   
            let options = self.user_interface.system_options()?;
    
            match options {
                SystemOptions::Add => {
                    let todo = self.user_interface.new_todo()?;
                    match todo {
                        Some(todo) => {
                            self.todo_storage.add_todo(todo.clone());
                            self.user_interface.show_todo(&todo)?;
                        },
                        None => self.user_interface.show_message(style("Não foi possível adicionar todo!".to_uppercase())),
                    } 
                },
                SystemOptions::List => {
                    let collection = self.todo_storage.get_todos();
                    if collection.is_empty() {
                        self.user_interface.show_message(style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red())
                    } else {
                        let mut count = 1;
                        self.user_interface.show_message(style("Minha lista de todos: ".to_string()).bold());
                        for i in collection {
                            let x: u8 = random();
                            self.user_interface.show_todos(count, style(&i.message.to_uppercase()).color256(x));
                            count += 1
                        }      
                    }
                },
                SystemOptions::Update => {
                    self.user_interface.show_message(style("Número do Todo :".to_string()).bold().green());
                    let number_todo = self.user_interface.input()?;
                    let number = number_todo.parse::<usize>();
                    match number {
                        Ok(number) => {
                            let todo = self.todo_storage.get_todo(number);
                            match todo {
                                Some(_todo) => {
                                    self.user_interface.show_message(style("Novo Todo :".to_string()).bold());
                                    let new_todo = self.user_interface.input()?;
                                    self.todo_storage.update_todo(number, new_todo);
                                    self.user_interface.show_message(style("Todo atualizado com Sucesso!!".to_string()).blue().bold())
                                },
                                None => self.user_interface.show_message(style("Número de Todo Inválido!".to_string()).red().bold())
                            }
                        },
                        Err(_) => self.user_interface.show_error_msg(style("[ERRO] Digite um número e não uma letra!".to_string().to_uppercase()).red())
                    }
                },
                SystemOptions::Delete => {
                    self.user_interface.show_message(style("Escolha o Todo que deseja deletar!".to_string()).bold().yellow());
                    let number_todo = self.user_interface.input()?;
                    let number = number_todo.parse::<usize>();
                    match number {
                        Ok(number) => {
                            let todo = self.todo_storage.get_todo(number);
                            match todo {
                                Some(_todo) => {
                                    self.todo_storage.remove_todo(number);
                                    self.user_interface.show_message(style("Todo removido com Sucesso!!".to_string()).white().bold())
                                },
                                None => self.user_interface.show_message(style("Número de Todo Inválido!".to_string()).red().bold())
                            }
                        },     
                        Err(_) => self.user_interface.show_error_msg(style("[ERRO] Digite somente números!".to_string().to_uppercase()).red())
                }
                },
                SystemOptions::Exit => {
                    self.user_interface.show_message(style("ToDo Encerrado! 💤".to_string()).underlined().bold());
                    return Ok(())
                },
                SystemOptions::Other => return Ok(())
            }
        }
    }
}