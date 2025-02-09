mod variable;
mod utils;
mod config;
// use utils::helper::print_max_points;

fn main() {
    variable::const_test("Rust");

    let upper = utils::to_uppercase("rust");
    println!("Uppercase: {}", upper);

    let sum = utils::add(10, 20);
    println!("Sum: {}", sum);

    utils::helper::print_max_points();
}
