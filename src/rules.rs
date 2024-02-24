pub mod rules {
    use std::process::{Command, Stdio};
    use arboard::Clipboard;

    pub struct Rules {
        pub letters: bool,
        pub numbs: bool,
        pub spec_symbs: bool,
        pub let_num_drc_free: bool,
        pub cursor_position: usize,
        pub pass_len: String,
        pub pwd: String,
    }

    impl Rules {
        pub fn default() -> Rules {
            Rules {
                letters: false,
                numbs: false,
                spec_symbs: true,
                let_num_drc_free: true,
                cursor_position: 1,
                pass_len: "8".parse().unwrap(),
                pwd: "".parse().unwrap(),
            }
        }

        pub fn move_cursor_left(&mut self) {
            let cursor_moved_left = self.cursor_position.saturating_sub(1);
            self.cursor_position = self.clamp_cursor(cursor_moved_left);
        }

        pub fn move_cursor_right(&mut self) {
            let cursor_moved_right = self.cursor_position.saturating_add(1);
            self.cursor_position = self.clamp_cursor(cursor_moved_right);
        }

        pub fn enter_char(&mut self, new_char: char) {
            self.pass_len.insert(self.cursor_position, new_char);
            self.move_cursor_right();
        }

        pub fn delete_char(&mut self) {
            let is_not_cursor_leftmost = self.cursor_position != 0;
            if is_not_cursor_leftmost {
                // Method "remove" is not used on the saved text for deleting the selected char.
                // Reason: Using remove on String works on bytes instead of the chars.
                // Using remove would require special care because of char boundaries.

                let current_index = self.cursor_position;
                let from_left_to_current_index = current_index - 1;

                // Getting all characters before the selected character.
                let before_char_to_delete = self.pass_len.chars().take(from_left_to_current_index);
                // Getting all characters after selected character.
                let after_char_to_delete = self.pass_len.chars().skip(current_index);

                // Put all characters together except the selected one.
                // By leaving the selected one out, it is forgotten and therefore deleted.
                self.pass_len = before_char_to_delete.chain(after_char_to_delete).collect();
                self.move_cursor_left();
            }
        }

        pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
            new_cursor_pos.clamp(0, self.pass_len.len())
        }

        pub fn reset_cursor(&mut self) {
            self.cursor_position = 0;
        }

        pub fn cursor_to_end(&mut self) {
            self.cursor_position = self.pass_len.len();
        }

        pub fn submit_message(&mut self) {
            self.pwd = self.generate_pass();

            if cfg!(unix) {
                let mut pipe = Command::new("echo").arg("-n").arg(self.pwd.clone()).stdout(Stdio::piped()).spawn().unwrap();
                let pipe_out = pipe.stdout.take().expect("Failed to take pipe stdout");
                let mut out = Command::new("xclip").arg("-selection").arg("clipboard").stdin(pipe_out).spawn().unwrap();
                out.wait().expect("Failed to run xclip");
            } else {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(self.pwd.clone()).expect("Copy to clipboard error");
            }

            /*let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(self.pwd.clone()).expect("Copy to clipboard error");*/

            //self.pass_len.clear();
            //self.reset_cursor();
        }

        /*fn pass_the_pwd<B: Backend>(&self, pwd: String, terminal: &mut Terminal<B>) {
            //terminal.draw(|f| main_ui(f, self))?;

            terminal.draw(|f| {
                let pwd_text = vec![
                    Line::from(Span::raw("")),
                    Line::from(Span::styled(pwd, Style::default().add_modifier(Modifier::BOLD))),
                    Line::from(Span::raw("")),
                ];
                let par = Paragraph::new(pwd_text)
                    .block(Block::new()
                        .title("Password is")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .red()
                    )
                    .black().on_white()
                    .alignment(Alignment::Center);
                f.render_widget(par, centered_rect(Rect::new(0, 50, f.size().width, 5), 30, 5));
            }).expect("panic in pass_the_pwd");


        }*/
    }
}