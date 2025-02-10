mod variable;
mod utils;
mod config;
mod base_type;
mod my_struct;
mod my_match;
mod all_pattern;
// use utils::helper::print_max_points;

fn main() {
    // variable::const_test("Rust");

    // let upper = utils::to_uppercase("rust");
    // println!("Uppercase: {}", upper);

    // let sum = utils::add(10, 20);
    // println!("Sum: {}", sum);

    // utils::helper::print_max_points();

    // base_type::base_type_main();
    // my_struct::test();
    // my_match::match_if();
    all_pattern::test();
}
