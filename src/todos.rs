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
}