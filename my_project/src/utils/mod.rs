
/*
pub mod math;
pub mod string;
has to use with
    let sum = utils::math::add(10, 20);
    println!("Sum: {}", sum);

    let upper = utils::string::to_uppercase("rust");
    println!("Uppercase: {}", upper);
*/


pub mod math;
pub mod string;
pub mod helper;
pub use math::add;
pub use string::to_uppercase;
