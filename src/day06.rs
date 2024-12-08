use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum MapComponent {
    Empty,
    Obstacle,
    Guard,
    GuardHasBeenHere(Vec<(isize, isize)>), // Also save the directions
}
pub fn solve(contents: &String) -> std::io::Result<(i64, i64)> {
    let map: Vec<Vec<MapComponent>> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => MapComponent::Empty,
                    '^' => MapComponent::Guard,
                    '#' => MapComponent::Obstacle,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let current_guard_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| match c {
                MapComponent::Guard => Some((x, y)),
                _ => None,
            })
        })
        .unwrap();
    let (part1, positions_visited) = part1(&map, current_guard_pos);
    let part2 = part2(&map, positions_visited, current_guard_pos);

    Ok((part1, part2.into()))
}
// Returns the result of part1 plus the positions visited by the guard which can be used in part2
fn part1(
    map: &Vec<Vec<MapComponent>>,
    current_guard_pos: (usize, usize),
) -> (i64, HashSet<(usize, usize)>) {
    let mut current_guard_pos = current_guard_pos;
    let mut current_direction: (isize, isize) = (0, -1);

    let mut positions_visited = HashSet::<(usize, usize)>::new();
    // The coordinate system is all over the place, i just switched things around untill my animation was correct
    while let Some(next_position) = get_next_position(current_guard_pos, current_direction, &map) {
        // print_board(&map);
        // sleep(Duration::from_millis(10));
        match map[next_position.1][next_position.0] {
            MapComponent::Empty | MapComponent::Guard | MapComponent::GuardHasBeenHere(_) => {
                positions_visited.insert(current_guard_pos);
                current_guard_pos = next_position;
            }
            MapComponent::Obstacle => {
                current_direction = turn_direction(current_direction);
            }
        }
    }
    positions_visited.insert(current_guard_pos);

    (positions_visited.len() as i64, positions_visited)
}

fn part2(
    map: &Vec<Vec<MapComponent>>,
    positions_visited: HashSet<(usize, usize)>,
    current_guard_pos: (usize, usize),
) -> i64 {
    // We only need to put an obstacle in the positions_visited because the guard will never reach encounter an obstacle in any other place
    // TODO: We can do even less if we keep the direction the guard was going in position_visited, then we just start from that position and direction
    // And check for loop there
    positions_visited
        .par_iter()
        .filter(|position| {
            let mut new_map = map.clone(); // I could skip this clone by just passing the position and map to check_for_loop
            new_map[position.1][position.0] = MapComponent::Obstacle;
            check_for_loop(new_map, current_guard_pos)
        })
        .count() as i64
}
fn check_for_loop(mut map: Vec<Vec<MapComponent>>, current_guard_pos: (usize, usize)) -> bool {
    // Go through the motions of the guard, but annotate where he went with which direction he was walking
    // If she ever walks in the same spot with the same direction, then we know it is a loop
    let mut current_direction: (isize, isize) = (0, -1);
    let mut current_guard_pos = current_guard_pos;
    while let Some(next_position) = get_next_position(current_guard_pos, current_direction, &map) {
        match map[next_position.1][next_position.0] {
            MapComponent::Empty | MapComponent::Guard => {
                map[next_position.1][next_position.0] =
                    MapComponent::GuardHasBeenHere(vec![current_direction]);
                current_guard_pos = next_position;
            }
            MapComponent::Obstacle => {
                current_direction = turn_direction(current_direction);
            }
            MapComponent::GuardHasBeenHere(ref directions) => {
                if directions.contains(&current_direction) {
                    return true;
                } else {
                    let new_directions = [&[current_direction], directions.as_slice()].concat();
                    map[next_position.1][next_position.0] =
                        MapComponent::GuardHasBeenHere(new_directions);
                    current_guard_pos = next_position;
                }
            }
        }
    }
    false
}

fn get_next_position(
    current_position: (usize, usize),
    direction: (isize, isize),
    map: &Vec<Vec<MapComponent>>,
) -> Option<(usize, usize)> {
    if let Some(next_x) = current_position.0.checked_add_signed(direction.0) {
        if next_x < map[0].len() {
            if let Some(next_y) = current_position.1.checked_add_signed(direction.1) {
                if next_y < map.len() {
                    return Some((next_x, next_y));
                }
            }
        }
    }
    None
}
fn turn_direction(direction: (isize, isize)) -> (isize, isize) {
    // There is some math with remainders we can do to get the same result, but whatever, it's late
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("Not a direction we can handle"),
    }
}
#[allow(unused)]
fn print_board(map: &Vec<Vec<MapComponent>>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for line in map {
        for component in line {
            match component {
                MapComponent::Empty => print!("{}", '.'),
                MapComponent::Obstacle => print!("{}", '#'),
                MapComponent::Guard => print!("{}", '^'),
                MapComponent::GuardHasBeenHere(_) => print!("{}", 'X'),
            }
        }
        print!("\r\n");
    }
    print!("\r\n");
    print!("\r\n");
}
