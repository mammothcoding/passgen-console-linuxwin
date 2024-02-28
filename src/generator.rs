pub mod generator {
    use arboard::Clipboard;
    use std::process::{Command, Stdio};

    const CIRCUITED_FIELDS: [&str; 6] = [
        "pwd_len",
        "letters",
        "u_letters",
        "numbs",
        "spec_symbs",
        "let_num_drc_free",
    ];

    pub struct Generator {
        pub letters: bool,
        pub u_letters: bool,
        pub numbs: bool,
        pub spec_symbs: bool,
        pub let_num_drc_free: bool,
        pub cursor_position: usize,
        pub field_position: String,
        pub pwd_len: String,
        pub min_pwd_len: u32,
        pub max_pwd_len: u32,
        pub pwd: String,
        pub lang: String,
    }

    impl Generator {
        pub fn default() -> Generator {
            Generator {
                letters: false,
                u_letters: false,
                numbs: false,
                spec_symbs: true,
                let_num_drc_free: true,
                cursor_position: 1,
                field_position: "pwd_len".to_string(),
                pwd_len: "8".to_string(),
                min_pwd_len: 4,
                max_pwd_len: 10000,
                pwd: "".to_string(),
                lang: "en".to_string(),
            }
        }

        pub fn get(&self, field_string: &str) -> bool {
            match field_string {
                "letters" => self.letters.clone(),
                "u_letters" => self.u_letters.clone(),
                "numbs" => self.numbs.clone(),
                "spec_symbs" => self.spec_symbs.clone(),
                "let_num_drc_free" => self.let_num_drc_free.clone(),
                _ => true,
            }
        }

        pub fn set(&mut self, field_string: &str, new_val: bool) {
            match field_string {
                "letters" => self.letters = new_val,
                "u_letters" => self.u_letters = new_val,
                "numbs" => self.numbs = new_val,
                "spec_symbs" => self.spec_symbs = new_val,
                "let_num_drc_free" => self.let_num_drc_free = new_val,
                _ => {}
            }
        }

        pub fn switch_lang(&mut self) {
            if self.lang == "en" {
                self.lang = "ru".to_string();
            } else {
                self.lang = "en".to_string();
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
            if &new_char == &' ' {
                if self.field_position != "pwd_len" {
                    let cur_status = self.get(&self.field_position).clone();
                    self.set(
                        &self.field_position.clone(),
                        if cur_status { false } else { true },
                    );
                };
            } else {
                self.pwd_len.insert(self.cursor_position, new_char);
                self.move_cursor_right();
            };
        }

        pub fn backspace_char(&mut self) {
            let is_not_cursor_leftmost = self.cursor_position != 0;
            if is_not_cursor_leftmost {
                let current_index = self.cursor_position;
                let before_char_to_delete = self.pwd_len.chars().take(current_index - 1);
                let after_char_to_delete = self.pwd_len.chars().skip(current_index);
                self.pwd_len = before_char_to_delete.chain(after_char_to_delete).collect();
                self.move_cursor_left();
            }
        }

        pub fn delete_char(&mut self) {
            let is_not_cursor_rightmost = self.cursor_position != self.pwd_len.parse().unwrap();
            if is_not_cursor_rightmost {
                let current_index = self.cursor_position;
                let before_char_to_delete = self.pwd_len.chars().take(current_index);
                let after_char_to_delete = self.pwd_len.chars().skip(current_index + 1);
                self.pwd_len = before_char_to_delete.chain(after_char_to_delete).collect();
            }
        }

        pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
            new_cursor_pos.clamp(0, self.pwd_len.len())
        }

        pub fn reset_cursor(&mut self) {
            self.cursor_position = 0;
        }

        pub fn cursor_to_end(&mut self) {
            self.cursor_position = self.pwd_len.len();
        }

        pub fn circ_cursor(&mut self) {
            let circ_last_idx = CIRCUITED_FIELDS.len() - 1;
            let cur_index = CIRCUITED_FIELDS
                .iter()
                .position(|&r| &r == &self.field_position)
                .unwrap();
            if cur_index < circ_last_idx {
                self.field_position = CIRCUITED_FIELDS[cur_index + 1].to_string();
            } else {
                self.field_position = CIRCUITED_FIELDS[0].to_string();
            }
        }

        pub fn up_cursor(&mut self) {
            let circ_last_idx = CIRCUITED_FIELDS.len() - 1;
            let cur_index = CIRCUITED_FIELDS
                .iter()
                .position(|&r| &r == &self.field_position)
                .unwrap();
            if cur_index > 0 {
                self.field_position = CIRCUITED_FIELDS[cur_index - 1].to_string();
            } else {
                self.field_position = CIRCUITED_FIELDS[circ_last_idx].to_string();
            }
        }

        pub fn submit_to_pwd(&mut self) {
            if self.is_valid_user_input() {
                let mut pwd = self.generate_pass();
                while !self.is_valid_pwd_by_consist(pwd.clone()) {
                    pwd = self.generate_pass();
                }
                self.pwd = pwd;

                if cfg!(unix) {
                    let mut pipe = Command::new("echo")
                        .arg("-n")
                        .arg(self.pwd.clone())
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();
                    let pipe_out = pipe.stdout.take().expect("Failed to take pipe stdout");
                    let mut out = Command::new("xclip")
                        .arg("-selection")
                        .arg("clipboard")
                        .stdin(pipe_out)
                        .spawn()
                        .unwrap();
                    out.wait().expect("Failed to run xclip");
                } else {
                    let mut clipboard = Clipboard::new().unwrap();
                    clipboard
                        .set_text(self.pwd.clone())
                        .expect("Copy to clipboard error");
                }
            } else {
                self.cursor_position = 1;
                self.field_position = "pwd_len".parse().unwrap();
                self.pwd_len = "8".parse().unwrap();
            }

            //self.pwd_len.clear();
            //self.reset_cursor();
        }

        fn is_valid_user_input(&self) -> bool {
            let parse_res = self.pwd_len.parse::<u32>();
            match parse_res {
                Ok(val) => {
                    if val < self.min_pwd_len || val > self.max_pwd_len {
                        false
                    } else {
                        true
                    }
                }
                Err(_err) => false,
            }
        }
    }
}
