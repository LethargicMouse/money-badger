use std::io;

use ratatui::{DefaultTerminal, Frame, crossterm::event};

fn main() {
    ratatui::run(run_app).unwrap()
}

fn run_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(render)?;
        if event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
