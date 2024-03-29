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

    pub async fn run(&mut self) -> Result<(), TerminalError> {
        clear().expect("Falhou em limpar a tela");
        self.todo_storage.read_storage().await;
        loop {
            let options = self.user_interface.system_options().await?;
    
            match options {
                SystemOptions::Add => {
                    let todo = self.user_interface.new_todo().await?;
                    match todo {
                        Some(todo) => {
                            self.todo_storage.add_todo(todo.clone()).await;
                            self.user_interface.show_todo(&todo).await?;
                        },
                        None => self.user_interface.show_message(style("Não foi possível adicionar todo!".to_uppercase())).await,
                    } 
                },
                SystemOptions::List => {
                    let collection = self.todo_storage.get_todos().await;
                    if collection.is_empty() {
                        self.user_interface.show_message(style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red()).await
                    } else {
                        let mut count = 1;
                        self.user_interface.show_message(style("Minha lista de todos: ".to_string()).bold()).await;
                        for i in collection {
                            let x: u8 = random();
                            self.user_interface.show_todos(count, style(&i.message.to_uppercase()).color256(x), i.resolved).await;
                            count += 1
                        }      
                    }
                },
                SystemOptions::Update => {
                    let collection = self.todo_storage.get_todos();
                    if collection.await.is_empty() {
                        self.user_interface.show_message(style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red()).await
                    } else {
                        self.user_interface.show_message(style("Número do Todo :".to_string()).bold().green()).await;
                        let number_todo = self.user_interface.input().await?;
                        let number = number_todo.parse::<usize>();
                        match number {
                            Ok(number) => {
                                let todo = self.todo_storage.get_todo(number);
                                match todo.await {
                                    Some(todo) => {
                                        if todo.resolved == true {
                                            self.user_interface.show_message(style("Esse todo já foi resolvido e não é possível atualizar!!".to_uppercase().to_string()).red().bold()).await
                                        } else {
                                            self.user_interface.show_message(style("Novo Todo :".to_string()).bold()).await;
                                            let new_todo = self.user_interface.input().await?;
                                            self.todo_storage.update_todo(number, new_todo).await;
                                            self.user_interface.show_message(style("Todo atualizado com Sucesso!!".to_string()).blue().bold()).await
                                        }
                                    },
                                    None => self.user_interface.show_message(style("Número de Todo Inválido!".to_string()).red().bold()).await
                                }
                            },
                            Err(_) => self.user_interface.show_error_msg(style("[ERRO] Digite um número e não uma letra!".to_string().to_uppercase()).red()).await
                        }
                    }
                },
                SystemOptions::Resolve => {
                    let collection = self.todo_storage.get_todos();
                    if collection.await.is_empty() {
                        self.user_interface.show_message(style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red()).await
                    } else {
                        self.user_interface.show_message(style("Escolha o Todo que deseja resolver!".to_string()).bold().yellow()).await;
                        let number_todo = self.user_interface.input().await?;
                        let number = number_todo.parse::<usize>();
                        match number {
                            Ok(number) => {
                                let todo = self.todo_storage.get_todo(number);
                                match todo.await {
                                    Some(todo) => {
                                        if todo.resolved == true {
                                            self.user_interface.show_message(style("Esse todo já está resolvido!!".to_string()).red().bold()).await
                                        } else {
                                            self.todo_storage.resolve_todo(number).await;
                                            self.user_interface.show_message(style("Todo resolvido com Sucesso!!".to_string()).white().bold()).await
                                        }
                                    },
                                    None => self.user_interface.show_message(style("Número de Todo Inválido!".to_string()).red().bold()).await
                                }
                            },     
                            Err(_) => self.user_interface.show_error_msg(style("[ERRO] Digite somente números!".to_string().to_uppercase()).red()).await
                        }
                    }
                },
                SystemOptions::Delete => {
                    let collection = self.todo_storage.get_todos();
                    if collection.await.is_empty() {
                        self.user_interface.show_message(style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red()).await
                    } else {
                        self.user_interface.show_message(style("Escolha o Todo que deseja deletar!".to_string()).bold().yellow()).await;
                        let number_todo = self.user_interface.input().await?;
                        let number = number_todo.parse::<usize>();
                        match number {
                            Ok(number) => {
                                let todo = self.todo_storage.get_todo(number);
                                match todo.await {
                                    Some(_todo) => {
                                        self.todo_storage.remove_todo(number).await;
                                        self.user_interface.show_message(style("Todo removido com Sucesso!!".to_string()).white().bold()).await
                                    },
                                    None => self.user_interface.show_message(style("Número de Todo Inválido!".to_string()).red().bold()).await
                                }
                            },     
                            Err(_) => self.user_interface.show_error_msg(style("[ERRO] Digite somente números!".to_string().to_uppercase()).red()).await
                        }
                    }
                },
                SystemOptions::Exit => {
                    self.user_interface.show_message(style("ToDo Encerrado! 💤".to_string()).underlined().bold()).await;
                    self.todo_storage.write_storage().await?;
                    return Ok(())
                },
                SystemOptions::Other => return Ok(())
            }
        }
    }
}