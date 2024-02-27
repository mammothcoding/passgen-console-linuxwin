pub mod ui {
    use crate::generator::generator::Generator;
    use ratatui::layout::{Alignment, Flex, Layout, Rect};
    use ratatui::prelude::{Line, Modifier, Span, Style, Stylize};
    use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
    use ratatui::Frame;

    pub fn draw_ui(f: &mut Frame, generator: &Generator) {
        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().light_green().on_black())
            .border_type(BorderType::Double)
            .on_black();
        f.render_widget(
            main_block,
            centered_rect(Rect::new(0, 0, f.size().width, 36), 60, 36),
        );

        let title1 = vec![
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "MAMMOTHCODING PASSGEN",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw("")),
        ];
        let par = Paragraph::new(title1)
            .block(
                Block::new()
                    .title("")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::new().light_green().on_black())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(
            par,
            centered_rect(Rect::new(0, 1, f.size().width, 5), 36, 5),
        );
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
        f.render_widget(
            par,
            centered_rect(Rect::new(0, 6, f.size().width, 5), 30, 5),
        );

        let pwd_len_field_area = centered_rect(Rect::new(0, 12, f.size().width, 3), 30, 3);
        let mut pwd_len_field = Paragraph::new(generator.pass_len.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Password length (4 - 10000)"),
        );
        pwd_len_field = if generator.field_position == "pwd_len" {
            pwd_len_field.yellow()
        } else {
            pwd_len_field.white()
        };
        f.render_widget(pwd_len_field, pwd_len_field_area);
        f.set_cursor(
            pwd_len_field_area.x + generator.cursor_position as u16 + 1,
            pwd_len_field_area.y + 1,
        );

        for field in [
            ("letters", "include letters", 15),
            ("numbs", "include numbers", 18),
            ("spec_symbs", "include special symbols", 21),
            ("let_num_drc_free", "exclude \"0oOiIlL1\" symbols", 24),
        ] {
            let on_criteria = if generator.get(field.0) { "+" } else { "-" };
            let fi_area = centered_rect(Rect::new(0, field.2, f.size().width, 3), 40, 3);
            let mut field_par = Paragraph::new(format!(" {}   {}", on_criteria, field.1))
                .block(Block::default().borders(Borders::ALL));
            field_par = if generator.field_position == field.0 {
                field_par.yellow()
            } else {
                field_par.white()
            };
            f.render_widget(field_par, fi_area);
        }

        if generator.pwd != "" {
            let pwd_text = vec![
                Line::from(Span::raw("")),
                Line::from(Span::raw(&generator.pwd).bold()),
                Line::from(Span::raw("")),
                Line::from(
                    Span::raw("this password was copied to clipboard")
                        .italic()
                        .on_gray(),
                ),
            ];
            let par = Paragraph::new(pwd_text)
                .block(
                    Block::new()
                        .title("Password is")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .black(),
                )
                .black()
                .on_white()
                .alignment(Alignment::Center);
            f.render_widget(
                par,
                centered_rect(Rect::new(0, 28, f.size().width, 6), 45, 6),
            );
        }
    }

    fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
        let horizontal = Layout::horizontal([width]).flex(Flex::Center);
        let vertical = Layout::vertical([height]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    // helper function to create a centered rect using up certain percentage of the available rect `r`
    // let area = f.size();
    // let area = centered_rect(60, 20, area);
    /*fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
            .split(r);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
            .split(popup_layout[1])[1]
    }*/
}
