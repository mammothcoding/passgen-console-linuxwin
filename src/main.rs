pub mod generator;
pub mod rules;

use crate::rules::rules::Rules;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use ratatui::{prelude::*, widgets::*, layout::*, layout::Constraint::*};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create Rules and run it
    let rules = Rules::default();
    let res = run_app(&mut terminal, rules);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut rules: Rules) -> io::Result<()> {
    loop {
        terminal.draw(|f| main_ui(f, &rules))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                /*KeyCode::Enter => {
                    terminal.draw(|f| pass_ui(f, &rules))?;
                }*/
                KeyCode::Esc => {
                    return Ok(());
                }
                KeyCode::Enter => {
                    rules.submit_message();
                },
                KeyCode::Char(to_insert) => {
                    rules.enter_char(to_insert);
                }
                KeyCode::Backspace => {
                    rules.delete_char();
                }
                KeyCode::Left => {
                    rules.move_cursor_left();
                }
                KeyCode::Right => {
                    rules.move_cursor_right();
                }
                _ => {}
            }
        }
    }
}

fn main_ui(f: &mut Frame, rules: &Rules) {
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().light_green().on_black())
        .border_type(BorderType::Double)
        .on_black();
    f.render_widget(main_block, centered_rect(Rect::new(0, 0, f.size().width, f.size().height), 60, f.size().height));


    let title1 = vec![
        Line::from(Span::raw("")),
        Line::from(Span::styled("MAMMOTHCODING PASSGEN", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("")),
    ];
    let par = Paragraph::new(title1)
        .block(Block::new().title("").borders(Borders::ALL).border_type(BorderType::Rounded))
        .style(Style::new().light_green().on_black())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(par, centered_rect(Rect::new(0, 1, f.size().width, 5), 36, 5));
    //println!("Password is: {:?}", f.size());

    let title2 = vec![
        Line::from(Span::raw("Press ")),
        Line::from(vec![
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
        ]),
        Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to generate a password"),
        ]),
    ];
    let par = Paragraph::new(title2)
        .block(Block::new().title(""))
        .style(Style::new().light_green().on_black())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(par, centered_rect(Rect::new(0, 6, f.size().width,  5), 30, 5));


    let input_area = centered_rect(Rect::new(0, 12, f.size().width,  3), 30, 3);
    let input = Paragraph::new(rules.pass_len.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Password length (4 - 10000)"));
    f.render_widget(input, input_area);
    f.set_cursor(
        input_area.x + rules.cursor_position as u16 + 1,
        input_area.y + 1,
    );


    if rules.pwd != "" {
        let pwd_text = vec![
            Line::from(Span::raw("")),
            Line::from(Span::raw(&rules.pwd).bold()),
            Line::from(Span::raw("")),
            Line::from(Span::raw("this password was copied to clipboard").italic().on_gray()),
        ];
        let par = Paragraph::new(pwd_text)
            .block(Block::new()
                .title("Password is")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .black()
            )
            .black().on_white()
            .alignment(Alignment::Center);
        f.render_widget(par, centered_rect(Rect::new(0, 40, f.size().width, 6), 45, 6));
    }
}



fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([width]).flex(Flex::Center);
    let vertical = Layout::vertical([height]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
