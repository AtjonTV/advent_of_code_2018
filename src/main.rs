// Copyright 2023 Thomas Obernosterer.
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;
use std::ops::Sub;
use std::time::Instant;
use levenshtein::levenshtein;

#[allow(dead_code)]
enum Solutions {
    Day1Part1,
    Day1Part2,
    Day2Part1,
    Day2Part2,
}

fn main() {
    let app_start = Instant::now();
    const EXAMPLE: bool = false;

    run_solution(Solutions::Day1Part1, EXAMPLE);
    run_solution(Solutions::Day1Part2, EXAMPLE);
    run_solution(Solutions::Day2Part1, EXAMPLE);
    run_solution(Solutions::Day2Part2, EXAMPLE);
    println!("Finished in {:?}", Instant::now().sub(app_start));
}

fn run_solution(solution: Solutions, use_example: bool) {
    match solution {
        Solutions::Day1Part1 => day1_part1(use_example),
        Solutions::Day1Part2 => day1_part2(use_example),
        Solutions::Day2Part1 => day2_part1(use_example),
        Solutions::Day2Part2 => day2_part2(use_example),
    }
}

fn get_input_path(day_name: &str, part_name: &str, use_example: bool) -> String {
    if use_example {
        format!("./inputs/{}/{}_example.txt", day_name, part_name)
    } else {
        format!("./inputs/{}/{}.txt", day_name, part_name)
    }
}

fn read_file_by_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path.clone()).expect(&*format!("Could not read input file: {}", path));

    input
    .lines()
    .map(|x| x.to_string())
    .filter(|x| !x.is_empty())
    .collect()
}

fn assert<T: PartialEq + Debug>(use_example: bool, expect_example: T, expect: T, actual: T) {
    if use_example {
        assert_eq!(expect_example, actual);
    } else {
        assert_eq!(expect, actual);
    }
}

fn day1_part1(use_example: bool) {
    let solution_start = Instant::now();
    let input_path = get_input_path("day1", "part1", use_example);
    let input = read_file_by_lines(&input_path);

    let mut sum = 0;
    for line in input {
        if let Ok(number) = line.trim().parse::<i32>() {
            sum += number;
        }
    }

    let result = sum;
    assert(use_example, 3, 423, result);
    println!("Day 1 Part 1 ({:?}): {}", Instant::now().sub(solution_start), sum);
}

fn day1_part2(use_example: bool) {
    let solution_start = Instant::now();
    let input_path = get_input_path("day1", "part2", use_example);
    let input = read_file_by_lines(&input_path);

    let mut matching_sum = None;
    let mut sum = 0;
    let mut past_sums: HashSet<i32> = HashSet::new();
    past_sums.insert(0);

    while matching_sum.is_none() {
        for line in &input {
            if let Ok(number) = line.trim().parse::<i32>() {
                sum += number;
            }
            if past_sums.contains(&sum) {
                matching_sum = Some(sum);
                break;
            }
            past_sums.insert(sum);
        }
    }

    let result = matching_sum.expect("Failed to find matching");
    assert(use_example, 10, 61126, result);
    println!("Day 1 Part 2 ({:?}): {}", Instant::now().sub(solution_start), result);
}

fn day2_part1(use_example: bool) {
    let solution_start = Instant::now();
    let input_path = get_input_path("day2", "part1", use_example);
    let input = read_file_by_lines(&input_path);

    let mut total_matching_two = 0;
    let mut total_matching_three = 0;
    for line in input {
        let line_result =
            line
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                if map.contains_key(&c) {
                    if let Some(count) = map.get(&c) {
                        map.insert(c, count + 1);
                    }
                } else {
                    map.insert(c, 1);
                }
                map
            });
        total_matching_two += line_result
            .iter()
            .find_map(|(&key, &val)| if val == 2 { Some(key) } else { None })
            .iter()
            .count();
        total_matching_three += line_result
            .iter()
            .find_map(|(&key, &val)| if val == 3 { Some(key) } else { None })
            .iter()
            .count();
    }

    let result = total_matching_two * total_matching_three;
    assert(use_example, 12, 7688, result);
    println!("Day 2 Part 1 ({:?}): {}", Instant::now().sub(solution_start), result);
}

fn day2_part2(use_example: bool) {
    let solution_start = Instant::now();
    let input_path = get_input_path("day2", "part2", use_example);
    let input = read_file_by_lines(&input_path);

    let mut ids: Vec<String> = Vec::new();
    for line in input {
        let line_result =
            line
                .chars()
                .fold(HashMap::new(), |mut map, c| {
                    if map.contains_key(&c) {
                        if let Some(count) = map.get(&c) {
                            map.insert(c, count + 1);
                        }
                    } else {
                        map.insert(c, 1);
                    }
                    map
                });
        let has_two_match = line_result
            .iter()
            .find_map(|(&key, &val)| if val == 2 { Some(key) } else { None })
            .iter()
            .count();
        let has_three_match = line_result
            .iter()
            .find_map(|(&key, &val)| if val == 3 { Some(key) } else { None })
            .iter()
            .count();

        if has_two_match != 0  || has_three_match != 0 {
            ids.push((*line.trim()).to_string());
        }
    }

    // sort the ids so that we can use a sliding window to find the ids we need
    ids.sort();

    let mut ids_match: Option<(String, String)> = None;
    let mut ids_window = ids.windows(2);
    'outer: while let Some(window) = ids_window.next() {
        if levenshtein(window[0].as_str(), window[1].as_str()) == 1 {
            ids_match = Some((window[0].clone(), window[1].clone()));
            break 'outer;
        }
    }

    if ids_match.is_none() {
        println!("Day 2 Part 2: No match found, exiting");
        return;
    }

    let (first, second): (String, String) = ids_match.unwrap();

    let mut common_chars: Vec<String> = Vec::new();
    for i in 0..first.len() {
        let fchar = first.get(i..(i+1));
        let schar = second.get(i..(i+1));

        if fchar.is_some() && schar.is_some() {
            if fchar.unwrap().eq(schar.unwrap()) {
                common_chars.push(first[i..(i+1)].to_string());
            }
        }
    }
    let result = common_chars.join("");

    assert(use_example, String::from("1"), String::from("lsrivmotzbdxpkxnaqmuwcchj"), result.clone());
    println!("Day 2 Part 2 ({:?}): {}", Instant::now().sub(solution_start), result);
}

#[allow(dead_code)]
fn example_part(use_example: bool) {
    let solution_start = Instant::now();
    let input_path = get_input_path("x", "y", use_example);
    #[allow(unused_variables)]
    let input = read_file_by_lines(&input_path);

    // TODO: Process input and calculate result

    let result = 0;
    assert(use_example, 1, 1, result);
    println!("Day X Part Y ({:?}): {}", Instant::now().sub(solution_start), result);
}
