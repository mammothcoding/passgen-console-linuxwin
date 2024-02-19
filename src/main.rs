

fn main() {
    use std::io::{self, BufRead};
    use std::process;
    use rand::Rng;
    const PASSWORD_LEN: usize = 30;
    const LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
    const NUMBERS_CHARSET: &[u8] = b"0123456789";
    const SPEC_SYMB_CHARSET: &[u8] = b")([]{}*&^%$#@!~";
    const LET_NUM_DRC_FREE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ\
                            abcdefghjkmnpqrstuvwxyz\
                            23456789";


    pub struct Rules {
        letters: bool,
        numbs: bool,
        spec_symbs: bool,
        let_num_drc_free: bool,
    }

    let start_rules = Rules {
        letters: true,
        numbs: true,
        spec_symbs: true,
        let_num_drc_free: true,
    };

    impl Rules {
        fn init(self) -> Rules {
            let letters;
            let numbs;
            let spec_symbs;
            let let_num_drc_free;

            println!("Hello, mother fucker!");
            println!("Letters?");
            let input = io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<String>();
            match input {
                Ok(ans) =>
                    match ans.as_str() {
                        "y" => letters = true,
                        "n" => letters = false,
                        _ => {
                            println!("Illegal input! Bye!");
                            process::exit(1);
                        },
                    }
                Err(_) => {
                    process::exit(1);
                }
            }

            println!("Numbers?");
            let input = io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<String>();
            match input {
                Ok(ans) =>
                    match ans.as_str() {
                        "y" => numbs = true,
                        "n" => numbs = false,
                        _ => {
                            println!("Illegal input! Bye!");
                            process::exit(1);
                        },
                    }
                Err(_) => {
                    process::exit(1);
                }
            }

            println!("Special symbols?");
            let input = io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<String>();
            match input {
                Ok(ans) =>
                    match ans.as_str() {
                        "y" => spec_symbs = true,
                        "n" => spec_symbs = false,
                        _ => {
                            println!("Illegal input! Bye!");
                            process::exit(1);
                        },
                    }
                Err(_) => {
                    process::exit(1);
                }
            }

            println!("Letters and numbers without doubly readable characters?");
            let input = io::stdin()
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .parse::<String>();
            match input {
                Ok(ans) =>
                    match ans.as_str() {
                        "y" => let_num_drc_free = true,
                        "n" => let_num_drc_free = false,
                        _ => {
                            println!("Illegal input! Bye!");
                            process::exit(1);
                        },
                    }
                Err(_) => {
                    process::exit(1);
                }
            }

            Rules {
                letters,
                numbs,
                spec_symbs,
                let_num_drc_free,
            }
        }

        fn generate_pass(self) {
            let mut rng = rand::thread_rng();
            let mut pass_charset: & [u8] = b"";
            //let mut pass_charset: & [u8] = LET_NUM_DRC_FREE;

            /*if self.let_num_drc_free == true {
                pass_charset = &*[pass_charset, LET_NUM_DRC_FREE].concat();
            } else {
                if self.letters == true {
                    pass_charset = &*[pass_charset, LETTERS_CHARSET].concat();
                }
                if self.numbs == true {
                    pass_charset = &*[pass_charset, NUMBERS_CHARSET].concat();
                }
            }
            if self.spec_symbs == true {
                pass_charset = &*[pass_charset, SPEC_SYMB_CHARSET].concat();
            }*/
            if pass_charset.len() == 0 {
                pass_charset = &*[pass_charset, LET_NUM_DRC_FREE].concat();
            }

            let password: String = (0..PASSWORD_LEN)
                .map(|_| {
                    let idx = rng.gen_range(0..pass_charset.len());
                    pass_charset[idx] as char
                })
                .collect();

            println!("Password is:");
            println!("{:?}", password);
        }
    }

    start_rules.init().generate_pass();
}
