pub mod generator {
    use arboard::Clipboard;
    use std::process::{Command, Stdio};

    const CIRCUITED_FIELDS: [&str; 6] = [
        "pwd_len",
        "letters",
        "u_letters",
        "numbs",
        "spec_symbs",
        "convenience_criterion",
    ];

    pub struct Generator {
        pub letters: bool,
        pub u_letters: bool,
        pub numbs: bool,
        pub spec_symbs: bool,
        pub convenience_criterion: bool,
        pub cursor_position: usize,
        pub rules_position: String,
        pub pwd_len: String,
        pub min_pwd_len: u32,
        pub max_pwd_len: u32,
        pub pwd: String,
        pub lang: String,
        pub errors: (String, String),
    }

    impl Generator {
        pub fn default() -> Generator {
            Generator {
                letters: false,
                u_letters: false,
                numbs: false,
                spec_symbs: true,
                convenience_criterion: true,
                cursor_position: 1,
                rules_position: "pwd_len".to_string(),
                pwd_len: "8".to_string(),
                min_pwd_len: 4,
                max_pwd_len: 10000,
                pwd: "".to_string(),
                lang: "en".to_string(),
                errors: ("".to_string(), "".to_string()),
            }
        }

        pub fn get_rule_state(&self, rule_name: &str) -> bool {
            match rule_name {
                "letters" => self.letters.clone(),
                "u_letters" => self.u_letters.clone(),
                "numbs" => self.numbs.clone(),
                "spec_symbs" => self.spec_symbs.clone(),
                "convenience_criterion" => self.convenience_criterion.clone(),
                _ => true,
            }
        }

        pub fn set_rule_state(&mut self, rule_name: &str, new_val: bool) {
            match rule_name {
                "letters" => self.letters = new_val,
                "u_letters" => self.u_letters = new_val,
                "numbs" => self.numbs = new_val,
                "spec_symbs" => self.spec_symbs = new_val,
                "convenience_criterion" => self.convenience_criterion = new_val,
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
                if self.rules_position != "pwd_len" {
                    let cur_status = self.get_rule_state(&self.rules_position).clone();
                    self.set_rule_state(
                        &self.rules_position.clone(),
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
                .position(|&r| &r == &self.rules_position)
                .unwrap();
            if cur_index < circ_last_idx {
                self.rules_position = CIRCUITED_FIELDS[cur_index + 1].to_string();
            } else {
                self.rules_position = CIRCUITED_FIELDS[0].to_string();
            }
        }

        pub fn up_cursor(&mut self) {
            let circ_last_idx = CIRCUITED_FIELDS.len() - 1;
            let cur_index = CIRCUITED_FIELDS
                .iter()
                .position(|&r| &r == &self.rules_position)
                .unwrap();
            if cur_index > 0 {
                self.rules_position = CIRCUITED_FIELDS[cur_index - 1].to_string();
            } else {
                self.rules_position = CIRCUITED_FIELDS[circ_last_idx].to_string();
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
                    let pipe = Command::new("echo")
                        .arg("-n")
                        .arg(self.pwd.clone())
                        .stdout(Stdio::piped())
                        .spawn();
                    if let Err(_err) = &pipe {
                        self.errors = (
                            "echo error by copy to clipbord!".to_string(),
                            "При вставке в буфер обмена произошла ошибка echo!".to_string(),
                        );
                    } else {
                        let pipe_out = pipe
                            .unwrap()
                            .stdout
                            .take()
                            .expect("Failed to take pipe stdout!");
                        let out = Command::new("xclip")
                            .arg("-selection")
                            .arg("clipboard")
                            .stdin(pipe_out)
                            .spawn();
                        if let Err(_err) = &out {
                            self.errors = (
                                "\'xclip\' packet needed for copy to clipbord!".to_string(),
                                "Для вставки в буфер обмена установите пакет \'xclip\'!"
                                    .to_string(),
                            );
                        } else {
                            let owait = out.unwrap().wait();
                            if let Err(_err) = &owait {
                                self.errors = (
                                    "Failed to run xclip!".to_string(),
                                    "Failed to run xclip!".to_string(),
                                );
                                owait.unwrap();
                            }
                        }
                    }
                } else {
                    let clipboard = Clipboard::new();
                    if let Err(_err) = &clipboard {
                        self.errors = (
                            "Copy to clipboard error!".to_string(),
                            "Ошибка копирования в буфер обмена!".to_string(),
                        );
                    } else {
                        let clip = clipboard.unwrap().set_text(self.pwd.clone());
                        if let Err(_err) = &clip {
                            self.errors = (
                                "Copy to clipboard error!".to_string(),
                                "Ошибка копирования в буфер обмена!".to_string(),
                            );
                        } else {
                            clip.unwrap();
                        }
                    }
                }
            } else {
                self.cursor_position = 1;
                self.rules_position = "pwd_len".parse().unwrap();
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
