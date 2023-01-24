use crate::todo::Todo;

#[derive(Debug)]
pub struct Todos {
    todos: Vec<Todo>,
}

pub trait TodoStorage {
    fn add_todo(&mut self, todo: Todo);
    fn update_todo(&mut self, index: usize, message: String);
    fn remove_todo(&mut self, index:usize);
    fn get_todo(&mut self, index:usize) -> Option<&Todo>;
    fn get_todos(&mut self) -> Vec<Todo>;
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            todos: Vec::new()
        }
    }
}

impl TodoStorage for Todos {
    fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    fn update_todo(&mut self, index: usize, message: String) {
        self.todos[index -1] = Todo::new(message);
    }

    fn remove_todo(&mut self, index:usize) {
        self.todos.remove(index - 1);
    }

    fn get_todo(&mut self, index:usize) -> Option<&Todo> {
        if index > self.todos.len() || index == 0 {
            None
        } else {
            Some(&self.todos[index - 1])
        }
    }

    fn get_todos(&mut self) -> Vec<Todo> {
        self.todos.clone()
    }
}