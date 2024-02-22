pub mod generator;

use generator::generator::Rules;

fn main() {

    let start_rules = Rules {
        letters: false,
        numbs: false,
        spec_symbs: true,
        let_num_drc_free: true,
    };
    start_rules.generate_pass();
}
