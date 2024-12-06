use std::{borrow::BorrowMut, cmp::Ordering, fs::File, io::Read, sync::Arc};

use core::simd::prelude::*;
use itertools::Itertools;
use regex::Regex;

pub fn solve(contents: &String) -> std::io::Result<(i64, i64)> {
    let (page_ordering_rules, updates) = contents.split_once("\r\n\r\n").unwrap();
    let page_ordering_rules: Vec<(u8, u8)> = page_ordering_rules
        .lines()
        .map(|line| {
            let (x, y) = line.split('|').collect_tuple().unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u8>> = updates
        .lines()
        .map(|line| -> Vec<u8> { line.split(',').map(|s| s.parse::<u8>().unwrap()).collect() })
        .collect();
    
    let (updates_following_rules, mut updates_not_following_rules): (Vec<Vec<u8>>, Vec<Vec<u8>>) =
        updates.into_iter().partition(|update| {
            page_ordering_rules
                .iter()
                .any(|rule| !check_rule(*rule, update))
        });
    let part1: u32 = updates_following_rules
        .iter()
        .map(|update| *update.get(update.len() / 2).unwrap() as u32)
        .sum();

    for update in updates_not_following_rules.iter_mut() {
        sort_by_rules(&page_ordering_rules, update);
    }
    let part2: u32 = updates_not_following_rules
        .into_iter()
        .map(|update| *update.get(update.len() / 2).unwrap() as u32)
        .sum();

    Ok((part1.into(), part2.into()))
}

fn check_rule(rule: (u8, u8), update: &Vec<u8>) -> bool {
    // If we find the second first it is ok, as long as we don't find the first page afterwards, so we check the entire array, and if it is not found the rule is also held
    let mut found_second = false;
    for page in update {
        if *page == rule.0 {
            if (found_second) {
                return false;
            } else {
                return true;
            }
        }
        if *page == rule.1 {
            found_second = true;
        }
    }
    true
}

fn sort_by_rules(rules: &Vec<(u8, u8)>, update: &mut Vec<u8>) {
    update.sort_by(|a, b| {
        for rule in rules {
            if *a == rule.0 && *b == rule.1 {
                return Ordering::Less;
            }
            if *b == rule.0 && *a == rule.1 {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
}
