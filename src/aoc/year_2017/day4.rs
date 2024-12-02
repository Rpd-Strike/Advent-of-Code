use std::fmt::Display;

use itertools::Itertools;

use crate::utils::{input_lines_iter, split_and_parse};

pub fn solution_one() -> impl Display {
    let rows = input_lines_iter()
        .map(split_and_parse::<String>)
        .collect_vec();

    rows.into_iter()
        .map(|words| {
            let n = words.len();
            for i in 0..n {
                for j in (i + 1)..n {
                    if words[i] == words[j] {
                        return false;
                    }
                }
            }
            return true;
        })
        .map(|w| (w == true) as i32)
        .sum::<i32>()
}

pub fn solution_two() -> impl Display {
    0
}
