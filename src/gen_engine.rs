pub mod gen_engine {
    use crate::generator::generator::Generator;
    use rand::Rng;

    const LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz";
    const NUMBERS_CHARSET: &[u8] = b"0123456789";
    const SPEC_SYMB_CHARSET: &[u8] = b")([]{}*&^%$#@!~";
    const LET_NUM_DRC_FREE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz\
                            23456789"; // double readable characters free / без двоякочитаемых символов

    impl Generator {
        pub fn generate_pass(&self) -> String {
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
            //println!("Pass_charset is:\n{:?}", pass_charset);

            (0..self.pass_len.parse::<u32>().unwrap())
                .map(|_| pass_charset[rng.gen_range(0..pass_charset.len())] as char)
                .collect()

            //println!("Password is: {:?}", password);
        }
    }
}
