#![feature(portable_simd)]
#![feature(unsigned_signed_diff)]
#![feature(ascii_char)]
use std::{fs::File, io::{self, Read}, path::Path};

use advent24::{day01, day02, day03, day04, day05, day06};
use utils::get_problem_input;



mod utils;
fn main() -> std::io::Result<()>{
    let solution = day01::solve(&get_problem_input(Path::new("input/day01.txt"))?);
    println!("Day01 solution = {}", solution.unwrap());
    let solution = day02::solve(&get_problem_input(Path::new("input/day02.txt"))?);
    println!("Day02 solution = {}", solution.unwrap());
    let solution = day03::solve(&get_problem_input(Path::new("input/day03.txt"))?);
    println!("Day03 solution = {}", solution.unwrap());
    let solution = day04::solve(&get_problem_input(Path::new("input/day04.txt"))?).unwrap();
    println!("Day04 solution = Part1: {} , Part2: {}", solution.0, solution.1);
    let solution = day05::solve(&get_problem_input(Path::new("input/day05.txt"))?).unwrap();
    println!("Day05 solution = Part1: {} , Part2: {}", solution.0, solution.1);
    let solution = day06::solve(&get_problem_input(Path::new("input/day06.txt"))?).unwrap();
    println!("Day06 solution = Part1: {} , Part2: {}", solution.0, solution.1);
    Ok(())
}
