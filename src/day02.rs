use std::{fs::File, io::Read};

use itertools::Itertools;
use core::simd::prelude::*;

pub fn solve(contents: &String) -> std::io::Result<String> {
    let reports: Vec<Vec<u8>> = contents.lines().map(|line| -> Vec<u8> {line.split_ascii_whitespace().map(|level| -> u8 { level.parse::<u8>().expect("Could not parse level")}).collect()}).collect();
    let part1 = part1(&reports);
    let part2 = part2(&reports);
    Ok(format!("Part 1: {part1}, Part 2: {part2}"))
}
fn safe(report: &Vec<u8>) -> bool {
    let r: Vec<i8> = report.windows(2).map(|window| window[0].checked_signed_diff(window[1]).unwrap()).collect();
    if (r.iter().all(|window| *window > 0) || r.iter().all(|window| *window < 0)) && r.iter().all(|window| { window.abs() <= 3 }) {
        true
    }
    else {
        false
    }
}
fn safe_with_dampener(report: &Vec<u8>) -> bool {
    if safe(report) {
        true
    }
    else {
        for i in 0..report.len() {
            let mut dummy = report.clone();
            dummy.remove(i);
            if safe(&dummy) {
                return true;
            }   
        }
        false
    }
}

fn part1(reports: &Vec<Vec<u8>>) -> u32 {
    reports.iter().filter(|report: &&Vec<u8>| safe(*report)).count() as u32
}
fn part2(reports: &Vec<Vec<u8>>) -> u32 {
    reports.iter().filter(|report: &&Vec<u8>| safe_with_dampener(*report)).count() as u32
}

#[test]
fn test_safe(){
    let test_vecs = vec![
        vec![7,6,4,2,1],
        vec![1,2,7,8,9],
        vec![9,7,6,2,1],
        vec![1,3,2,4,5],
        vec![8,6,4,4,1],
        vec![1,3,6,7,9]];
    assert!(safe(&test_vecs[0]));
    assert!(!safe(&test_vecs[1]));
    assert!(!safe(&test_vecs[2]));
    assert!(!safe(&test_vecs[3]));
    assert!(!safe(&test_vecs[4]));
    assert!(safe(&test_vecs[5]));
}

#[test]
fn test_safe_with_dampener(){
    let test_vecs = vec![
        vec![7,6,4,2,1],
        vec![1,2,7,8,9],
        vec![9,7,6,2,1],
        vec![1,3,2,4,5],
        vec![8,6,4,4,1],
        vec![1,3,6,7,9]];
    assert!(safe_with_dampener(&test_vecs[0]));
    assert!(!safe_with_dampener(&test_vecs[1]));
    assert!(!safe_with_dampener(&test_vecs[2]));
    assert!(safe_with_dampener(&test_vecs[3]));
    assert!(safe_with_dampener(&test_vecs[4]));
    assert!(safe_with_dampener(&test_vecs[5]));
}