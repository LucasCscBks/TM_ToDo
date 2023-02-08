use crate::{todo::Todo, terminalerror::TerminalError};
use async_trait::async_trait;
use tokio::fs::{ read_to_string, write};

#[derive(Debug)]
pub struct Todos {
    todos: Vec<Todo>,
}

#[async_trait]
pub trait TodoStorage {
    async fn read_storage(&mut self);
    async fn add_todo(&mut self, todo: Todo);
    async fn update_todo(&mut self, index: usize, message: String);
    async fn remove_todo(&mut self, index:usize);
    async fn resolve_todo(&mut self, index: usize);
    async fn get_todo(&mut self, index:usize) -> Option<&Todo>;
    async fn get_todos(&mut self) -> Vec<Todo>;
    async fn write_storage(&mut self)-> Result<(), TerminalError>;
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            todos: Vec::new()
        }
    }
}

#[async_trait]
impl TodoStorage for Todos {
    async fn read_storage(&mut self) {
        let file = read_to_string("session.txt").await;
        match file {
            Ok(file) => {
                let mut text_todos: Vec<&str> = file.split("-").collect();
                text_todos.pop();
                let mut count = 0;
                while count < text_todos.len() {
                    let todo = Todo::new(text_todos[count].to_string(), text_todos[count + 1].contains("true"));
                    self.todos.push(todo);
                    count +=2
                }
            },
            Err(_err) => println!("Primeira sessão! Ainda não existe o arquivo de sessão!")
        }
    }

    async fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    async fn update_todo(&mut self, index: usize, message: String) {
        self.todos[index -1].message = message;
    }

    async fn remove_todo(&mut self, index:usize) {
        self.todos.remove(index - 1);
    }

    async fn get_todo(&mut self, index:usize) -> Option<&Todo> {
        if index > self.todos.len() || index == 0 {
            None
        } else {
            Some(&self.todos[index - 1])
        }
    }

    async fn resolve_todo(&mut self, index:usize) {
        self.todos[index -1].resolved = true;
    }

    async fn get_todos(&mut self) -> Vec<Todo> {
        self.todos.clone()
    }

    async fn write_storage(&mut self) -> Result<(), TerminalError> {
        let content = self.todos.iter()
            .map(|todo| format!("{}-{}-", todo.message, todo.resolved)).collect::<Vec<String>>();
        write("session.txt", content.join("")).await.map_err(TerminalError::Stdout)?;
        Ok(())
    }
}