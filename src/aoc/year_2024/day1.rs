use std::{collections::BTreeMap, fmt::Display};

use itertools::{sorted, Itertools};

use crate::utils::{input_lines_iter, split_and_parse};

pub fn solution_one() -> impl Display {
    let (a, b): (Vec<_>, Vec<_>) = input_lines_iter()
        .map(split_and_parse::<i32>)
        .map(|l| (l[0], l[1]))
        .unzip();
    let a = sorted(a).collect_vec();
    let b = sorted(b).collect_vec();
    let n = a.len();

    (0..n).map(|i| i32::abs_diff(a[i], b[i])).sum::<u32>()
}

pub fn solution_two() -> impl Display {
    let (a, b): (Vec<_>, Vec<_>) = input_lines_iter()
        .map(split_and_parse::<i32>)
        .map(|l| (l[0], l[1]))
        .unzip();

    let mut m: BTreeMap<i32, i32> = BTreeMap::new();
    for v in b {
        let x = m.entry(v).or_insert(0);
        *x += 1;
    }
    let mut sum = 0;
    for v in a {
        let cnt = m.entry(v).or_default();
        sum += v * *cnt;
    }
    sum
}
