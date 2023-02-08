pub mod terminal;
pub mod todo;
pub mod terminalerror;
pub mod todos;
pub mod cli;
use crate::terminal::UserInterface;
use crate::cli::TodoCli;
use crate::terminal::Terminal;
use crate::todos::Todos;

#[tokio::main]
async fn main() {
    let mut terminal = terminal::Terminal::new();
    let mut cli = TodoCli::new(Box::new(Terminal::new()),Box::new(Todos::new()));
    if let Err(error) = cli.run().await {
        terminal.show_error(error).await;
    }
}