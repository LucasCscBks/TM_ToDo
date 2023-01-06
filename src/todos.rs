use crate::todo::Todo;
use console::style;
use rand::prelude::random;

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
        if index > self.todos.len() || index == 0 {
            println!("{}", style("Número de Todo Inválido!").red().bold())
        } else {
            self.todos.remove(index - 1);
            println!("{}" , style("Todo removido com Sucesso!!").white().bold())
        }
    }

    pub fn get_todo(&mut self, index:usize) -> Option<&Todo> {
        if index > self.todos.len() || index == 0 {
            None
        } else {
            Some(&self.todos[index - 1])
        }
    }

    pub fn show_todos(&mut self) {
        if self.todos.is_empty() {
            println!("{}", style("Nenhum todo adicionado ainda!".to_uppercase()).bold().red());  
        } else {
            let mut count = 1;
            println!("{}" , style("Minha lista de todos: ").bold());
            for i in &self.todos {
                let x: u8 = random();
                println!("{} : {:?}", count, style(&i.message.to_uppercase()).color256(x));
                count += 1
            }      
        }
    }
}