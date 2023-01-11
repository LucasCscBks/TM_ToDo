use crate::todo::Todo;

#[derive(Debug)]
pub struct Todos {
    todos: Vec<Todo>,
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

    pub fn get_todo(&mut self, index:usize) -> Option<&Todo> {
        if index > self.todos.len() || index == 0 {
            None
        } else {
            Some(&self.todos[index - 1])
        }
    }

    pub fn get_todos(&mut self) -> Vec<Todo> {
        self.todos.clone()
    }

}