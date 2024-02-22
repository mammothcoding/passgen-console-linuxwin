pub mod generator;

use generator::generator::Rules;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use ratatui::{prelude::*, widgets::*, layout::*, layout::Constraint::*};

fn main() -> Result<(), Box<dyn Error>> {
    ///////////////////////////////////
    //Rules::default().generate_pass();
    ///////////////////////////////////

    // setup terminal
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

    //main_ui
    terminal.draw(|f| main_ui(f))?;

    //on keys ui
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    terminal.draw(|f| pass_ui(f, &rules))?;
                }
                KeyCode::Esc => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

fn main_ui(f: &mut Frame) {
    fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
        let horizontal = Layout::horizontal([width]).flex(Flex::Center);
        let vertical = Layout::vertical([height]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().cyan().on_black())
        .border_type(BorderType::Double)
        .on_black();
    f.render_widget(block, f.size());


    let title1 = vec![
        Line::from(Span::raw("")),
        Line::from(Span::styled("MAMMOTHCODING PASSGEN", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("")),
    ];
    let par = Paragraph::new(title1)
        .block(Block::new().title("").borders(Borders::ALL).border_type(BorderType::Double))
        .style(Style::new().light_green().on_black())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(par, centered_rect(Rect::new(0, 2, f.size().width, 5), 36, 5));
    //println!("Password is: {:?}", f.size());

    let title2 = vec![
        Line::from(Span::raw("Press ")),
        Line::from(vec![
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
        ]),
        Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to generate a passwords"),
        ]),
    ];
    let par = Paragraph::new(title2)
        .block(Block::new().title(""))
        .style(Style::new().light_green().on_black())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(par, centered_rect(Rect::new(0, 7, f.size().width,  5), 30, 5));



}

fn pass_ui(f: &mut Frame, rules: &Rules) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
                .as_ref(),
        )
        .split(f.size());



    let pass = Rules::default().generate_pass();
    let content = vec![Line::from(Span::raw(format!("{}", pass)))];

    let mut messages: Vec<ListItem> = Vec::new();
    messages.push(ListItem::new(content));
    let messages = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Password is:"));
    f.render_widget(messages, chunks[2]);
}

