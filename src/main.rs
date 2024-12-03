#![feature(portable_simd)]
#![feature(unsigned_signed_diff)]

use std::{fs::File, io::{self, Read}, path::Path};

use utils::get_problem_input;

mod day01;
mod day02;
mod day03;

mod utils;
fn main() -> std::io::Result<()>{
    let solution = day01::solve(&get_problem_input(Path::new("input/day01.txt"))?);
    println!("Day01 solution = {}", solution.unwrap());
    let solution = day02::solve(&get_problem_input(Path::new("input/day02.txt"))?);
    println!("Day02 solution = {}", solution.unwrap());
    let solution = day03::solve(&get_problem_input(Path::new("input/day03.txt"))?);
    println!("Day03 solution = {}", solution.unwrap());
    Ok(())
}
