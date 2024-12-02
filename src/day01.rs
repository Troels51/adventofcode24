use std::{fs::File, io::Read};

use itertools::Itertools;
use core::simd::prelude::*;

pub fn solve(contents: &String) -> std::io::Result<String> {
    let mut lists: (Vec<u32>, Vec<u32>) = contents.lines().map(|line| -> (u32, u32) {
        line.split_ascii_whitespace().map(|location_id| location_id.parse::<u32>().expect("Could not parse to int") ).collect_tuple().expect("Could not collect to tuple")
    }).unzip();
    lists.0.sort();
    lists.1.sort();
    let part1 = part1(&lists.0, &lists.1);
    let part2 = part2(&lists.0, &lists.1);
    Ok(format!("Part 1: {part1}, Part 2: {part2}"))
}

fn part1(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    std::iter::zip(left.chunks_exact(16), right.chunks_exact(16)).map(|(x,y)| {
        let a = u32x16::from_slice(x);
        let b = u32x16::from_slice(y);
        let k = a.saturating_sub(b) | b.saturating_sub(a);
        k.reduce_sum()}
    ).sum()
    
    // std::iter::zip(left, right).map(|(x,y)| x.abs_diff(*y)).sum()
}
fn part2(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    left.iter().map(|x| x * right.iter().filter(|current| x == *current ).count() as u32).sum()
}