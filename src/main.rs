fn main() {
    let mut terminal = todo::terminal::Terminal::new();
    if let Err(error) = todo::terminal::loop_todo() {
        terminal.show_error(error)
    }
}
