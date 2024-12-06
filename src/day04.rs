use std::{fs::File, io::Read, sync::Arc};

use core::simd::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn solve(contents: &String) -> std::io::Result<(i64, i64)> {
    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let xs: Vec<(usize, usize)> = lines
        .iter()
        .enumerate()
        .map(|(line_nr, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(char_nr, value)| -> Option<(usize, usize)> {
                    if *value == 'X' {
                        Some((line_nr, char_nr))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect();

    let a_s: Vec<(usize, usize)> = lines
        .iter()
        .enumerate()
        .map(|(line_nr, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(char_nr, value)| -> Option<(usize, usize)> {
                    if *value == 'A' {
                        Some((line_nr, char_nr))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect();
    let part1: u32 = xs.iter().map(|x| check_xmas(&lines, *x)).sum();
    let part2: u32 = a_s.iter().map(|a| if check_mas(&lines, *a) { 1} else {0}).sum();

    Ok((part1.into(), part2.into()))
}
fn check_pos(lines: &Vec<Vec<char>>, pos: (usize, usize), check_for: char) -> bool {
    if let Some(line) = lines.get(pos.0) {
        if let Some(char) = line.get(pos.1) {
            if *char == check_for {
                return true;
            }
        }
    }
    false
}
fn check_direction(
    lines: &Vec<Vec<char>>,
    middle: (usize, usize),
    diffs: [(isize, isize); 3],
) -> bool {
    if let [
        (Some(pos0_0), Some(pos0_1)),
        (Some(pos1_0), Some(pos1_1)),
        (Some(pos2_0), Some(pos2_1)),
    ] = diffs.map(|diff| {
        (
            middle.0.checked_add_signed(diff.0),
            middle.1.checked_add_signed(diff.1),
        )
    }) {
        check_pos(lines, (pos0_0, pos0_1), 'M')
            && check_pos(lines, (pos1_0, pos1_1), 'A')
            && check_pos(lines, (pos2_0, pos2_1), 'S')
    } else {
        false
    }
}
// These can be converted into hex rotation math, but i will keep the cases seperate because i am lazy
fn check_straight(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(0, 1), (0, 2), (0, 3)])
}
fn check_backwards(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(0, -1), (0, -2), (0, -3)])
}
fn check_up(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(-1, 0), (-2, 0), (-3, 0)])
}
fn check_down(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(1, 0), (2, 0), (3, 0)])
}
fn check_down_right(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(1, 1), (2, 2), (3, 3)])
}
fn check_down_left(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(1, -1), (2, -2), (3, -3)])
}
fn check_up_right(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(-1, 1), (-2, 2), (-3, 3)])
}
fn check_up_left(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    check_direction(lines, pos, [(-1, -1), (-2, -2), (-3, -3)])
}

fn check_xmas(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> u32 {
    // Check 6 different ways of getting xmas from an x
    let mut xmasses: u32 = 0;
    let checks = vec![
        check_straight,
        check_backwards,
        check_up,
        check_down,
        check_down_right,
        check_down_left,
        check_up_right,
        check_up_left,
    ];
    for check in checks {
        if check(lines, pos) {
            xmasses += 1;
        }
    }
    xmasses
}
fn check_mas(lines: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    // x-masses can't appear if the A is on the edge
    if pos.0 == 0 || pos.1 == 0 || pos.0 == lines.len() || pos.1 == lines.first().unwrap().len() {
        return false;
    }
    // This checks all the rotations of MAS around an A, i could probably get clever with rotation math, but i will just put all the combinations here
    // Check
    // MOS
    // OAO
    // MOS
    check_pos(lines, (pos.0 - 1, pos.1 - 1), 'M')
        && check_pos(lines, (pos.0 + 1, pos.1 - 1), 'M')
        && check_pos(lines, (pos.0 - 1, pos.1 + 1), 'S')
        && check_pos(lines, (pos.0 + 1, pos.1 + 1), 'S')
    ||
    // Rotate once clockwise
    check_pos(lines, (pos.0 - 1, pos.1 - 1), 'M')
        && check_pos(lines, (pos.0 - 1, pos.1 + 1), 'M')
        && check_pos(lines, (pos.0 + 1, pos.1 + 1), 'S')
        && check_pos(lines, (pos.0 + 1, pos.1 - 1), 'S')
    ||
    // Rotate once clockwise
    check_pos(lines, (pos.0 - 1, pos.1 - 1), 'S')
        && check_pos(lines, (pos.0 + 1, pos.1 - 1), 'S')
        && check_pos(lines, (pos.0 - 1, pos.1 + 1), 'M')
        && check_pos(lines, (pos.0 + 1, pos.1 + 1), 'M')
        ||
    // Rotate once clockwise
    check_pos(lines, (pos.0 - 1, pos.1 - 1), 'S')
        && check_pos(lines, (pos.0 - 1, pos.1 + 1), 'S')
        && check_pos(lines, (pos.0 + 1, pos.1 + 1), 'M')
        && check_pos(lines, (pos.0 + 1, pos.1 - 1), 'M')
}

#[test]
fn test_case1() {
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let b = solve(&input.to_string()).unwrap();
    assert_eq!(b.0, 18);
}

#[test]
fn test_case2() {
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    let b = solve(&input.to_string()).unwrap();
    assert_eq!(b.1, 9);
}
