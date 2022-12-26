pub mod terminal;
pub mod todo;
pub mod terminalerror;
pub mod todos;

fn main() {
    let mut terminal = terminal::Terminal::new();
    if let Err(error) = terminal::loop_todo() {
        terminal.show_error(error)
    }
}
