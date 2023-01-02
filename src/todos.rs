use crate::todo::Todo;
#[derive(Debug)]
pub struct Todos {
    pub todos: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Self {
        Todos {
            todos: Vec::new()
        }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn update_todo(&mut self, index: usize, message: String) {
        self.todos[index -1] = Todo::new(message);
    }

    pub fn remove_todo(&mut self, index:usize) {
        self.todos.remove(index - 1);
    }
}