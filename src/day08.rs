use std::{borrow::BorrowMut, cmp::Ordering, collections::HashSet, fs::File, io::Read, iter::Map, sync::Arc, thread::sleep, time::Duration, vec};

use core::simd::prelude::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use regex::Regex;
#[derive(Debug, Clone)]
struct Calibration {
    solution: u64,
    operands: Vec<u64>,
}
#[derive(PartialEq, Eq, Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

pub fn solve(contents: &String) -> std::io::Result<(i64, i64)> {
    let calibrations: Vec<Calibration> = contents.lines().map(|line| {
        let mut iter = line.split(':');
        let solution: u64 = iter.next().unwrap().parse().unwrap();
        let operands: Vec<u64> = iter.next().unwrap().split_ascii_whitespace().map(|operand|operand.parse().unwrap()).collect();
        Calibration {
            solution,
            operands
        }
    }).collect();
    
    // Part1:
    // TODO: No need to save the operations
    let calibration_operations = calibrations.par_iter().filter_map(|calibration| {
        if let Some(operations) = get_operations(&calibration) {
            Some((calibration, operations))
        }
        else {
            None
        }
    }
    );
    let part1 : u64 = calibration_operations.map(|cal| cal.0.solution).sum();

    let calibration_operations = calibrations.par_iter().filter_map(|calibration| {
        if let Some(operations) = get_operations_with_concat(&calibration) {
            Some((calibration, operations))
        }
        else {
            None
        }
    }
    );
    let part2 : u64 = calibration_operations.map(|cal| cal.0.solution).sum();

    Ok((part1 as i64, part2 as i64))
}

fn get_operations(calibration: &Calibration) -> Option<Vec<Operation>> {
    // We could do some backtracking to quicker find the solution. But i am just going through all the combinations
    let mut operations_combinations = itertools::repeat_n([Operation::Add,Operation::Multiply], calibration.operands.len() - 1).multi_cartesian_product();
    let operation = operations_combinations.find(|operations| check_calibration(calibration, operations));
    operation
}

fn get_operations_with_concat(calibration: &Calibration) -> Option<Vec<Operation>> {
    // This could be folded together with other get_operations
    let mut operations_combinations = itertools::repeat_n([Operation::Add,Operation::Multiply, Operation::Concat], calibration.operands.len() - 1).multi_cartesian_product();
    let operation = operations_combinations.find(|operations| check_calibration(calibration, operations));
    operation
}

fn check_calibration(calibration: &Calibration, operations: &Vec<Operation>) -> bool {
    let mut operands = calibration.operands.iter();
    let first: u64 = *operands.next().unwrap();
    let result = operands.zip(operations.iter()).fold(first, |acc, (operand, operation)| match operation {
        Operation::Add => acc + operand,
        Operation::Multiply => acc * operand,
        Operation::Concat => (acc.to_string() + &operand.to_string()).parse().unwrap(),
    });
    result == calibration.solution
}

#[test]
fn test_get_operations() {
    let calibration = Calibration {
        solution: 190,
        operands: vec![10,19]
    };
    let operations = get_operations(&calibration);
    assert_eq!(operations, Some(vec![Operation::Multiply]));

    let calibration = Calibration {
        solution: 3267,
        operands: vec![81,40,27]
    };
    let operations = get_operations(&calibration);
    assert_eq!(operations, Some(vec![Operation::Add, Operation::Multiply]));

    let calibration = Calibration {
        solution: 292,
        operands: vec![11,6,16,20]
    };
    let operations = get_operations(&calibration);
    assert_eq!(operations, Some(vec![Operation::Add, Operation::Multiply, Operation::Add]));
}