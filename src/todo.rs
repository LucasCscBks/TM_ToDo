use std::fmt;

#[derive(Debug, Clone)]
pub struct Todo {
    pub message: String,
    pub resolved: bool
}

impl Todo {
    pub fn new(message: String, resolved: bool) -> Self {
        Todo { 
            message,
            resolved 
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.message)
    }
}