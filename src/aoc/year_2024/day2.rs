use std::fmt::Display;

use crate::utils::{input_lines_iter, split_and_parse};

fn check_arr(l: Vec<i32>) -> bool {
    // increasing
    let mut increasing = true;
    for i in 1..(l.len()) {
        let diff = l[i] - l[i - 1];
        if diff < 1 || diff > 3 {
            increasing = false;
        }
    }
    if increasing {
        return true;
    }
    let mut increasing = true;
    for i in 1..(l.len()) {
        let diff = l[i - 1] - l[i];
        if diff < 1 || diff > 3 {
            increasing = false;
        }
    }
    increasing
}

pub fn solution_one() -> impl Display {
    let rows = input_lines_iter()
        .map(split_and_parse::<i32>)
        .map(check_arr)
        .map(|t| (t == true) as i32)
        .sum::<i32>();

    rows
}

pub fn solution_two() -> impl Display {
    let rows = input_lines_iter()
        .map(split_and_parse::<i32>)
        .map(|l| {
            for i in 0..(l.len()) {
                let mut new_v = l[0..i].to_vec();
                new_v.extend_from_slice(l[(i + 1)..].to_vec().as_slice());
                if check_arr(new_v) {
                    return true;
                }
            }
            return false;
        })
        .map(|t| (t == true) as i32)
        .sum::<i32>();

    rows
}
