use std::io;

#[derive(Debug)]
pub enum TerminalError {
    Stdout(io::Error),
    Stdin(io::Error),
}

impl TerminalError {
    pub fn error_type(self) -> String {
        match self {
            Self::Stdin(err) => format!("Erro {}", err),
            Self::Stdout(err) => format!("Erro {}", err)
        }
    }
}