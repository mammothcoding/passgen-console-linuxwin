mod gen_engine;
pub mod generator;
pub mod ui;

use crate::generator::generator::Generator;
use crate::ui::ui::draw_ui;
use crossterm::{
    event,
    event::*,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let generator = Generator::default();
    let res = run_app(&mut terminal, generator);

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    //terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut generator: Generator) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, &generator))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::F(9) => {
                        generator.switch_lang();
                    }
                    KeyCode::Enter => {
                        generator.submit_to_pwd();
                    }
                    KeyCode::Backspace => {
                        generator.backspace_char();
                    }
                    KeyCode::Delete => {
                        generator.delete_char();
                    }
                    KeyCode::Left => {
                        generator.move_cursor_left();
                    }
                    KeyCode::Right => {
                        generator.move_cursor_right();
                    }
                    KeyCode::Home => {
                        generator.reset_cursor();
                    }
                    KeyCode::End => {
                        generator.cursor_to_end();
                    }
                    KeyCode::Tab => {
                        generator.circ_cursor();
                    }
                    KeyCode::Up => {
                        generator.up_cursor();
                    }
                    KeyCode::Down => {
                        generator.circ_cursor();
                    }
                    KeyCode::Char(to_insert) => {
                        generator.enter_char(to_insert);
                    }
                    _ => {}
                }
            }
        }
    }
}
