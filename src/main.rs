

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
        letters: false,
        numbs: false,
        spec_symbs: true,
        let_num_drc_free: true,
    };

    impl Rules {
        /*fn init(self) -> Rules {
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
        }*/

        fn generate_pass(self) {
            let mut rng = rand::thread_rng();
            let mut pass_assembly: Vec<&[u8]> = Vec::new();
            //let mut pass_charset: Vec<u8> = Vec::new();

            //let mut pass_charset: & [u8] = LET_NUM_DRC_FREE;

            if self.let_num_drc_free {
                pass_assembly.push(LET_NUM_DRC_FREE);
            } else {
                if self.letters {
                    pass_assembly.push(LETTERS_CHARSET);
                }
                if self.numbs {
                    pass_assembly.push(NUMBERS_CHARSET);
                }
            }
            if self.spec_symbs {
                pass_assembly.push(SPEC_SYMB_CHARSET);
            }

            let pass_charset: Vec<u8> = pass_assembly.into_iter().flatten().cloned().collect();
            println!("Pass_charset is:\n{:?}", pass_charset);

            let password: String = (0..PASSWORD_LEN)
                .map(|_| pass_charset[rng.gen_range(0..pass_charset.len())] as char)
                .collect();

            println!("Password is: {:?}", password);
        }
    }

    start_rules.generate_pass();
}
