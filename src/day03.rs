use std::{fs::File, io::Read};

use itertools::Itertools;
use regex::Regex;
use core::simd::prelude::*;

pub fn solve(contents: &String) -> std::io::Result<String> {
    let re = Regex::new(r"mul+\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap(); // Capture numbers within mul(a,b)
    let mults: Vec<(u32, u32)> = re.captures_iter(contents).map(|cap| (cap.name("a").unwrap().as_str().parse::<u32>().unwrap(), cap.name("b").unwrap().as_str().parse::<u32>().unwrap())).collect();
    let part1: u32 = mults.iter().map(|(a,b)| a*b).sum();
    let do_dont_re = Regex::new(r"(?s)don't\(\).*?(do\(\)|$)").unwrap(); // Remove everything within don't() <XXX> do() or end of string
    let enabled = do_dont_re.replace_all(&contents, "");

    let enabled_mults: Vec<(u32, u32)> = re.captures_iter(&enabled).map(|cap| (cap.name("a").unwrap().as_str().parse::<u32>().unwrap(), cap.name("b").unwrap().as_str().parse::<u32>().unwrap())).collect();
    let part2: u32 = enabled_mults.iter().map(|(a,b)| a*b).sum();
    Ok(format!("Part 1: {part1}, Part 2: {part2}"))
}