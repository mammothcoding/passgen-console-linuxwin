pub mod ui {
    use crate::generator::generator::Generator;
    use ratatui::layout::Alignment::Center;
    use ratatui::layout::{Alignment, Flex, Layout, Rect};
    use ratatui::prelude::{Line, Modifier, Span, Style, Stylize};
    use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
    use ratatui::Frame;

    pub fn draw_ui(f: &mut Frame, generator: &Generator) {
        if f.size().height < 40 {
            // Expand check
            let expand_text = format!(
                "▲▼ Please expand the window to accommodate the contents / Пожалуйста, разверните окно для вмещения содержимого ▲▼ {}/40",
                f.size().height
            );
            f.render_widget(
                Paragraph::new(expand_text.to_string())
                    .light_green()
                    .on_black()
                    .alignment(Alignment::Center),
                centered_rect(
                    Rect::new(0, 0, f.size().width, 1),
                    expand_text.len() as u16,
                    1,
                ),
            );
        } else {
            // Main block
            let main_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().gray().on_black())
                .border_type(BorderType::Double)
                .on_black();
            f.render_widget(
                main_block,
                centered_rect(Rect::new(0, 0, f.size().width, 40), 70, 40),
            );

            // Title
            let par = Paragraph::new("MAMMOTHCODING PASSGEN")
                .add_modifier(Modifier::BOLD)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .cyan()
                .on_black()
                .alignment(Alignment::Center);
            f.render_widget(
                par,
                centered_rect(Rect::new(0, 1, f.size().width, 3), 60, 3),
            );

            // Legend
            let legend = if generator.lang.as_str() == "en" {
                vec![
                    Line::from(Span::raw("Press ")),
                    Line::from(vec![
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to exit, "),
                        Span::styled("F9", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to switch interface lang, "),
                    ]),
                    Line::from(vec![
                        Span::styled(
                            "Tab, ▲, ▼, Space",
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" to manage a rules"),
                    ]),
                    Line::from(vec![
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to generate a password"),
                    ]),
                ]
            } else {
                vec![
                    Line::from(Span::raw("Нажмите ")),
                    Line::from(vec![
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" для выхода, "),
                        Span::styled("F9", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" для переключения языка интерфейса, "),
                    ]),
                    Line::from(vec![
                        Span::styled(
                            "Tab, ▲, ▼, Space",
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" для выбора правил"),
                    ]),
                    Line::from(vec![
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" для генерации пароля"),
                    ]),
                ]
            };
            let par = Paragraph::new(legend)
                .block(Block::new().title(""))
                .style(Style::new().light_green().on_black())
                .alignment(Alignment::Center);
            f.render_widget(
                par,
                centered_rect(Rect::new(0, 4, f.size().width, 7), 60, 7),
            );

            // Lang indicator
            let ind = if generator.lang.as_str() == "en" {
                Line::from(vec![
                    Span::raw("En").bold().white().on_blue(),
                    Span::raw("Ru").black().on_gray(),
                ])
            } else {
                Line::from(vec![
                    Span::raw("En").black().on_gray(),
                    Span::raw("Ru").bold().white().on_blue(),
                ])
            };
            f.render_widget(
                Paragraph::new(ind).alignment(Alignment::Right),
                centered_rect(Rect::new(0, 36, f.size().width, 1), 68, 1),
            );

            // Password length input area
            let pwd_len_field_area = centered_rect(Rect::new(0, 11, f.size().width, 3), 44, 3);
            let mut pwd_len_field = Paragraph::new(generator.pwd_len.as_str()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(
                        "{} ({} - {})",
                        if generator.lang.as_str() == "en" {
                            "Password length"
                        } else {
                            "Длина пароля"
                        },
                        generator.min_pwd_len,
                        generator.max_pwd_len
                    ))
                    .title_alignment(Center),
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

            // Rules
            let fields = if generator.lang.as_str() == "en" {
                [
                    ("letters", "include lowercase letters", 14),
                    ("u_letters", "include capital letters", 17),
                    ("numbs", "include numbers", 20),
                    ("spec_symbs", "include special symbols", 23),
                    (
                        "convenience_criterion",
                        "strong & usability password",
                        26,
                    ),
                ]
            } else {
                [
                    ("letters", "включая маленькие буквы", 14),
                    ("u_letters", "включая большие буквы", 17),
                    ("numbs", "включая цифры", 20),
                    ("spec_symbs", "включая спец. символы", 23),
                    (
                        "convenience_criterion",
                        "сильный и удобный пароль",
                        26,
                    ),
                ]
            };

            for field in fields {
                let on_criteria = if generator.get(field.0) { "+" } else { "-" };
                let fi_area = centered_rect(Rect::new(0, field.2, f.size().width, 3), 44, 3);
                let mut field_par = Paragraph::new(format!(" {}   {}", on_criteria, field.1))
                    .block(Block::default().borders(Borders::ALL));
                field_par = if generator.field_position == field.0 {
                    field_par.yellow()
                } else {
                    field_par.white()
                };
                f.render_widget(field_par, fi_area);
            }

            // Password result area
            let mut pwd = generator.pwd.clone();
            if pwd.len() > 40 {
                pwd = format!("{}...", &pwd[..39].to_string());
            }
            if generator.pwd != "" {
                let text: String = if generator.lang.as_str() == "en" {
                    if generator.errors.0 != "" {
                        generator.errors.0.clone()
                    } else {
                        "this password was copied to clipboard".to_string()
                    }
                } else {
                    if generator.errors.1 != "" {
                        generator.errors.1.clone()
                    } else {
                        "пароль был скопирован в буфер обмена".to_string()
                    }
                };
                let pwd_text = vec![
                    Line::from(Span::raw("")),
                    Line::from(Span::raw(pwd).bold()),
                    Line::from(Span::raw("")),
                    Line::from(
                        Span::raw(text)
                            .italic()
                            .on_gray(),
                    ),
                ];
                let pwd_title = if generator.lang.as_str() == "en" {
                    "Password is"
                } else {
                    "Ваш пароль"
                };
                let par = Paragraph::new(pwd_text)
                    .block(
                        Block::new()
                            .title(pwd_title)
                            .title_alignment(Center)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .black(),
                    )
                    .black()
                    .on_white()
                    .alignment(Alignment::Center);
                f.render_widget(
                    par,
                    centered_rect(Rect::new(0, 30, f.size().width, 6), 55, 6),
                );
            }

            // Footer
            let footer = if generator.lang.as_str() == "en" {
                "Made with RUST | 2024 | https://github.com/mammothcoding"
            } else {
                "Создано на языке RUST | 2024 | https://github.com/mammothcoding"
            };
            let par = Paragraph::new(footer)
                .block(Block::default().borders(Borders::TOP).gray().on_black())
                .cyan()
                .on_black()
                .alignment(Alignment::Center);
            f.render_widget(
                par,
                centered_rect(Rect::new(0, 37, f.size().width, 2), 68, 2),
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
}
