pub mod gen_engine {
    use crate::generator::generator::Generator;
    use rand::Rng;
    use rand_hc::Hc128Rng;
    use rand_isaac::Isaac64Rng;
    use rand_seeder::rand_core::SeedableRng;

    const LETTERS_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const U_LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUMBERS_CHARSET: &[u8] = b"0123456789";
    const SPEC_SYMB_CHARSET: &[u8] = b")([]{}*&^%$#@!~";
    const SIMP_SYMB_CHARSET: &[u8] = b"*&%$#@!"; // set without inconvenient symbols / набор без неудобных символов
    const CONVENIENT_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz\
                            23456789"; // set without ambiguous and inconvenient letters with numbers / набор без двоякочитаемых и неудобных букв с цифрами
    const CONVENIENT_LETTERS_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz"; // set without ambiguous and inconvenient letters with numbers / набор без двоякочитаемых и неудобных букв с цифрами

    impl Generator {
        pub fn generate_pass(&mut self) -> String {
            let mut isaac_seeder = Isaac64Rng::from_entropy();
            let mut rng = Hc128Rng::from_rng(&mut isaac_seeder).unwrap();
            let mut pass_assembly: Vec<&[u8]> = Vec::new();

            if self.convenience_criterion
                || (!self.letters && !self.u_letters && !self.numbs && !self.spec_symbs)
            {
                self.convenience_criterion = true;
                pass_assembly.push(CONVENIENT_CHARSET);
            } else {
                if self.letters {
                    pass_assembly.push(LETTERS_CHARSET);
                }
                if self.u_letters {
                    pass_assembly.push(U_LETTERS_CHARSET);
                }
                if self.numbs {
                    pass_assembly.push(NUMBERS_CHARSET);
                }
                if self.spec_symbs {
                    pass_assembly.push(SPEC_SYMB_CHARSET);
                }
            }

            let pass_charset: Vec<u8> = pass_assembly.into_iter().flatten().cloned().collect();
            let mut pass_candidate_vec: Vec<u8> = Vec::new();
            let pass_processing_len: u32 = self.pwd_len.parse::<u32>().unwrap();

            if self.convenience_criterion {
                let letters_charset: Vec<u8> =
                    CONVENIENT_LETTERS_CHARSET.into_iter().cloned().collect();
                let simp_symb_charset: Vec<u8> = SIMP_SYMB_CHARSET.into_iter().cloned().collect();

                // gen first pass symbol from all letters
                pass_candidate_vec.push(letters_charset[rng.gen_range(0..letters_charset.len())]);

                // gen main pass body
                for _ in 0..(pass_processing_len - 2) {
                    pass_candidate_vec.push(pass_charset[rng.gen_range(0..pass_charset.len())]);
                }

                // gen last pass symbol from simple symbols
                pass_candidate_vec
                    .push(simp_symb_charset[rng.gen_range(0..simp_symb_charset.len())]);

                String::from_utf8(pass_candidate_vec).unwrap()
            } else {
                (0..pass_processing_len)
                    .map(|_| pass_charset[rng.gen_range(0..pass_charset.len())] as char)
                    .collect()
            }
        }

        pub fn is_valid_pwd_by_consist(&self, pass: String) -> bool {
            let pwd_in_bytes = pass.clone().into_bytes();

            let check_to_available_for = |symbols: &[u8]| -> bool {
                let mut res = false;
                for ch in &pwd_in_bytes {
                    if symbols.contains(&ch) {
                        res = true;
                        break;
                    }
                }
                res
            };

            // compliance check
            if self.letters || self.convenience_criterion {
                if !check_to_available_for(LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.u_letters || self.convenience_criterion {
                if !check_to_available_for(U_LETTERS_CHARSET) {
                    return false;
                }
            }
            if self.numbs || self.convenience_criterion {
                if !check_to_available_for(NUMBERS_CHARSET) {
                    return false;
                }
            }
            if self.spec_symbs || self.convenience_criterion {
                if !check_to_available_for(SPEC_SYMB_CHARSET) {
                    return false;
                }
            }
            true
        }
    }
}
