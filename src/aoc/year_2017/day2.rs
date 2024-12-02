use std::fmt::Display;

use itertools::{sorted, Itertools};

use crate::utils::{input_lines_iter, split_and_parse};

pub fn solution_one() -> impl Display {
    let rows: i32 = input_lines_iter()
        .map(split_and_parse::<i32>)
        .map(|el| {
            let s = sorted(el.into_iter()).collect_vec();
            let small = s[0];
            let big = s.last().unwrap();
            big - small
        })
        .sum();

    rows
}

pub fn solution_two() -> impl Display {
    let rows = input_lines_iter().map(split_and_parse::<i32>).collect_vec();
    let ans = rows
        .into_iter()
        .map(|r| {
            let n = r.len();
            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    if r[i] % r[j] == 0 {
                        return r[i] / r[j];
                    }
                }
            }
            todo!()
        })
        .sum::<i32>();

    ans
}
